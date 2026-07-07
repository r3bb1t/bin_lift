# Milestone 3 — x86 semantics port blueprint (legacy `src/` → new seam)

> Reference captured from a legacy-semantics exploration. M3 is a **clean-room port** of the
> legacy `src/` x86 handlers onto the proven M3 foundation: a backend-generic
> `Lifter<B: IrBuilder>` (spike commit `e86438d`) driving `lift-core`'s brand-free `Session`.
> The M3 plan is NOT yet written — this is the porting map to write it from.

## Foundation already in place (M1 + M2 + M3 spike)
- `lift-core`: `Session` (brand-free; `step<B>`/`resume<B>` take `&mut B`), `Lifter<B: IrBuilder>` (generic over backend), `InsnView`/`OperandView` seam, events, assumption-free resolution.
- `lift-llvmkit`: `IrBuilder` over llvmkit with the full data-flow op surface (`const_int/zero/ones`, `iadd/isub/imul`, `and/or/xor/not`, `shl/lshr/ashr`, `urem`, `icmp(pred)`, `zext/sext/trunc/zext_or_trunc`, `select`, `load/store`, control-flow). Values are opaque `B::Value` carrying their own width. **Width mismatch on an op = a hard `.expect()` panic** → handlers must width-align operands explicitly.

## Legacy code to port (file:line refs are in `src/`)

### Register model
- `miscellaneous.rs`: `ExtendedRegisterEnum` = `zydis::Register` (1:1) **+ 20 appended flag variants** (CF..ID at 266–285 + 3 `ReservedN` fillers for RFLAGS bits 1/3/5). Two `From` impls (1:1 tables; flag variants `unreachable!()` in reverse). This is the hashmap key.
- `lifter/mod.rs`: state = `HashMap<ExtendedRegisterEnum, Value>` storing **only the largest-enclosing GPR** (RAX, never EAX/AX/AL) + 18 individual 1-bit flag entries. `get_max_int_type()` = native GPR width (64 in LONG_64).
- `common.rs`: `get_register_largest_enclosing` — zydis `largest_enclosing(mode)` **except RBP/RSP/ESP/EBP hardcoded** (zydis mishandles them). `get_register` defaults miss → `const_zero` (not undef).
- `getters.rs`: `load_register_value` — IP→runtime-addr const; FLAGS→`get_rflags_value`; **AH/CH/DH/BH → `(enclosing >> 8) & 0xFF`** (recomputed off full reg each time, no caching); else lookup by largest-enclosing key. `mergen_get_register(reg, size)` = load then zext/trunc to `size` (unless ≥128-bit / XMM).
- `setters.rs`: `store_reg` — 8-bit write → `set_val_to_sub_reg_8b` (mask `0xFF..00` or `..00FF` for high-byte, OR in, shift<<8 for AH/etc.); 16-bit → `set_val_to_sub_reg_16b` (mask `..0000`, OR); FLAGS → `set_rflags_value`; else store under largest-enclosing. **GOTCHA: 32-bit writes are NOT special-cased** — a 32-bit `EAX` write is stored raw under RAX with no zero-extension of the upper 32 bits (x86 requires upper-32 clear on 32-bit write — the port must fix this). **GOTCHA: 8-bit helper hardcodes `i64_type()`/64-bit masks** regardless of mode.
- `store_cpu_flag` truncates to 1-bit before storing; flags stored under their own key (not largest-enclosing).

### Flags — `flagops.rs` (+ overflow in `binary.rs`)
- `compute_zero_flag` = `icmp eq value, 0`. `compute_sign_flag` = `icmp slt value, 0`.
- `compute_parity_flag` = SWAR popcount-parity over the low byte in 64-bit (`&0xFF` → zext i64 → `*0x0101010101010101` → `&0x8040201008040201` → `% 0x1FF` → `&1` → `icmp eq 0`). This is the exact `urem`-using sequence M2 added `urem` for.
- `compute_aux_flag` (ADC/SBB only) = XOR trick: `((result ^ (lhs ^ rhs)) & 0x10) == (result ^ lhs ^ rhs)`. ADD/SUB/INC/DEC/NEG/XADD compute AF via inline nibble compare instead.
- **NO `compute_carry_flag` helper** — CF is ad-hoc per handler (ULT compares for add/sub, shifted-out-bit for shifts). **Port opportunity: factor CF helpers.**
- `compute_overflow_flag_add`/`_sub` live as **private fns in `binary.rs`** (XOR-then-`icmp slt 0` trick), not in flagops.
- `get_rflags_value`/`set_rflags_value` pack/unpack **bits 0–11 only** (CF..OF); NT/RF/VM/AC/... exist as separate entries but aren't round-tripped.

### Operands — `getters.rs`/`setters.rs`
- `load_single_op(operand, size)` dispatches `Reg→mergen_get_register` / `Mem→mergen_load_mem` / `Imm→const_int(size, value, is_signed)`. Size = `DecodedOperand.size` (u16, **bits**). Handlers load both operands at **dest.size** (implicit width alignment).
- `store_op` dispatches Reg→`store_reg` / Mem→`mergen_store_mem`; width inferred from the value's type.

### Effective address — `mergen_getters_and_setters.rs`
- `mergen_get_effective_address` = `base + index*scale + disp`, all zext to i64. Base/index loaded via `get_register` (bypasses high-byte special case). GS segment → `unimplemented!("TEB")`.
- `mergen_load_mem`/`mergen_store_mem` = GEP into one big `stackmemory` alloca (`[i128 x 0x1000]`) + `load`/`store`. **These two are the `IrBuilder::load`/`store` seam points for M3; the flat-buffer/aliasing model is M4 (`solve_load`).**
- `lift_lea` reuses `mergen_get_effective_address` (address only, no load/store).

### Dispatch — `semantics/x86/mod.rs`
- Big `match instr.mnemonic` → per-category handler methods. ~21 category modules: `binary, bitbyte, call, cmov, convert, dataxfer, flagop, logical, misc(LEA), nop, pop, push, ret, rotate, semaphore(XADD), setcc, shift, system(RDTSC), uncond_br(JMP)`.
- **`cond_br.rs` and `stringop.rs` are EMPTY** — conditional branches (Jcc) are **unimplemented** in legacy (only JMP). String ops live in `dataxfer::lift_movs_x`.
- `increase_ip` is a **compile-time `Cell<u64>`** (RIP materialized as a constant; no IR-level RIP register).
- Debug build emits the disassembly text as a dummy `alloca` name (IR breadcrumb).
- Handler pattern: **operand-load → compute result + each flag → `store_cpu_flag` per flag → `store_op(dest, result)`**. Representative bodies to port: `binary::lift_add_sub`, `lift_adc`; `logical::lift_and_andn`; `shift::lift_shl` (count-mask, CF from last-out-bit, OF only when count==1); `dataxfer::lift_mov` (MOV/MOVZX/MOVSX/MOVSXD); `semaphore::lift_xadd`.

### Function boundary — `compiler/mod.rs`
- `create_func` = **17 GPR params (native width) + 18 flag params (bool)**, returns native-width RAX. `lift_function` = per-instruction driver loop, then load RAX + zext/trunc to return type + `ret`, optional O2.
- `contexts.rs` (`CpuContext`/`StartContextX86`) = a separate concrete-initial-state mechanism (constants from Rust values), not used by the symbolic-param path.

### zydis instruction access (for the arch-neutral adapter)
- `FullInstruction`: `.mnemonic` (enum), `.length: u8`, `.operands() -> &[DecodedOperand]`.
- `DecodedOperand`: `.size: u16` (bits), `.kind: Unused|Reg(Register)|Mem(MemoryInfo)|Imm(ImmediateInfo)|Ptr`.
- `MemoryInfo`: `{segment, base, index: Register, scale: u8, disp: {has, displacement: i64}}`. `ImmediateInfo`: `{is_signed, value: u64}`.
- `Register`: `.class()`, `.width(mode)`, `.largest_enclosing(mode)`.

## Key decisions the M3 plan must make (not yet decided)
1. **The x86 `InsnView` shape.** `lift-core`'s `InsnView` is minimal (`address/len/mnemonic() -> &str`). x86 dispatch needs the mnemonic enum + typed operands. Likely: `lift-x86` defines a richer x86-instruction/operand view (its own trait or a concrete decoded type) that a `zydis` adapter implements — the arch-neutral seam sits at `Lifter<B>`/events, not at full operand modeling.
2. **`RegisterFile`** design over `B::Value`: keyed by largest-enclosing; sub-register read/write via `shl/lshr/and/or` + `trunc/zext`; flags as i1 `B::Value`s. Fix the 32-bit-write-clears-upper-32 and mode-width bugs during the port.
3. **Width discipline:** operand-load must width-align (the legacy "load src at dest.size" convention) so ops don't hit the `.expect()` panic.
4. **Function boundary:** keep flat reg/flag params (SSA value model) vs. Mergen ctx*+mem* — the SSA value model (M1 choice) favors params-in / RAX-out.
5. **Scope split with M4:** M3 = data-flow semantics (registers, flags, arithmetic/logical/shift/rotate/data-transfer/stack/setcc/cmov) + the zydis adapter + control-flow handlers that return `Transfer` (Session resolves). M4 = memory aliasing (`solve_load`/GEPTracker) + richer branch resolution. Conditional branches (Jcc, empty in legacy) are NEW work — `lift_body` returns `Transfer::ResolveBranch{..}` and the `Session` already handles it.
6. **CF factoring:** add `compute_carry_*` helpers the legacy lacked.
7. Legacy `src/` stays as the differential correctness oracle until parity, then is removed.
