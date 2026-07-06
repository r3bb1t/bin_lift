# bin_lift v2 — Foundational Redesign: a streaming, oracle-driven binary→IR lifter

> Design spec produced via the brainstorming workflow and approved on 2026-07-06.
> The implementation plan derived from this spec lives alongside it (see the writing-plans output).

## Context

`bin_lift` today (crate `zydis2llvmir`, v0.0.1-dev-4) is a single-crate, **static, straight-line x86-64 → LLVM 18** lifter: the caller decodes a whole `Vec<FullInstruction>` with zydis, hands it to `Compiler::lift_function()`, and gets back one `i64 protected(17 GPRs, 18 flags)` function with a `[4096 x i128]` stack alloca. Branches just bump RIP (`cond_br.rs` is empty), calls are a TODO, there is no event/feedback system, no ABI/signature awareness, and the LLVM backend is hardwired.

The goal is a **from-scratch API/architecture redesign** (breaking changes accepted) into a **"lego" foundation crate** for large lifting/deobfuscation systems. Its sole job: translate raw instruction bytes into IR while acting as a well-abstracted building block that:

- **receives instructions as a live stream** from many producers (emulators, hypervisors, Frida, Unicorn, Sogen, trace files);
- **provides feedback events** ("a branch happened", "a call happened") and — crucially — can **suspend at a decision point and ask an oracle** whether a branch is real or fake (opaque predicate), or ask an emulator to snapshot → probe → roll back before committing;
- is **function-signature / OS-API aware** (lift a `call` into ntdll as a typed call);
- emits IR **plus hints/attributes** for downstream analysis crates;
- exposes a **stable C-ABI** so it can be the foundation for plugins in other software (Frida, IDA, Ghidra, Binary Ninja, x64dbg).

Long-term vision: broader than Mergen — all architectures, all formats, eventually UEFI — but **x86-64 only for now**, with remill/Sogen-style OS-API modeling. This document is the design + implementation plan for the first cut.

## Locked decisions (from brainstorming)

| # | Decision | Choice |
|---|---|---|
| 1 | Input model | **Streaming push + events** — `step(insn) -> outcome`, IR built incrementally |
| 2 | Crate scope | **Lift-core only** — takes already-decoded, arch-neutral instructions; decoders + IR-analysis are siblings |
| 3 | Plugin target | **Powers plugins for other tools** — needs a stable **C-ABI/FFI** surface |
| 4 | Abstraction | **Seams now, x86+LLVM first** — arch-neutral + IR-builder traits, one concrete path |
| 5 | State-in-IR | **Mergen-style SSA values** — register→current-value map internally |
| 6 | Memory-in-IR | **Mergen-style flat address space** — concrete/symbolic, real loads/stores + `inttoptr` |
| 7 | Call lifting | **Always fully typed**; unknown signature ⇒ **synthesize a prototype from the calling convention** |
| 8 | Platform scope | **Traits + Windows-x64 platform + starter ntdll/Win32 signature DB** |
| 9 | Migration | **Clean-room rewrite** of semantics against the new traits (old code = correctness oracle) |
| 10 | Branch resolution | **Hybrid** — oracle-first concretization, then LLVM-pass cleanup |
| 11 | IR backend | **llvmkit primary (dogfood, pure Rust)**; inkwell/LLVM as the optimization bridge |
| 12 | Assumptions | **Assumption-free core.** No baked-in packer heuristics; all special knowledge is **caller-declared** through an explicit interface |

### Core principle: the lifter is assumption-free

Mergen hardcodes domain- and packer-specific heuristics into the lifter itself: "ignore writes to this region because it's a Themida section," a fixed `STACKP_VALUE` sentinel to distinguish real returns from ROP, prioritized "Themida control cursor" loop slots, viability filters, etc. **We reject that.** The core lifter makes **no** guesses about what a region means, what a sentinel is, or whether a predicate is opaque. Instead:

- **Declared facts (caller → lifter, proactive):** the caller *tells* the lifter things — "ignore writes to `[X,Y)`", "`[A,B)` is scratch/volatile", "`[C,D)` is read-only constant image", "the canonical entry stack pointer is `Z`", "this address is a loop-carried slot", "the value at this site is constant `K`", "this predicate is always-true". Registered before and during lifting via an `AssumptionProvider`.
- **Reactive queries (lifter → caller):** when the lifter hits something no fact covers, it **suspends and asks** (the `Query`/`resume` path / Oracle) rather than applying a heuristic.

The result: the *same* engine handles VMProtect, Themida, UEFI, or clean code — the difference lives entirely in the facts the caller supplies, never in the lifter's code. This is what makes it a reusable lego block instead of a packer-specific tool.

## Research-derived insights that shape the design

**Mergen (read from C++ source):**
- **Concolic** (concrete+symbolic) with an internal **`backup_point`** (register array + byte-granular memory map + path-assumption map) → snapshot/restore for path exploration. This is an *internal* oracle; our external Oracle is the same idea with a live source.
- **RegisterManager**: 18 GPR `Value*` + 32 flags; **sub-registers are not stored separately** — truncate/zext at operand boundaries. **Lazy flag evaluation** via closures (compute a flag only when read).
- **GEPTracker memory aliasing**: `DenseMap<addr, {value, byteOffset}>`; `solveLoad(addr,size)` = constant-offset lookup → depth-limited (16) symbolic → path-sensitive `select`s → PHI traversal → jump-table inference (≤64 targets) → **graceful degradation** (>512 unknowns ⇒ symbolic load). Overlapping partial writes reconstructed via mask/extract/shift/or.
- **MemoryPolicy**: `CONCRETE` regions (fold from PE image/stack) vs `SYMBOLIC` (emit load IR), as sorted intervals with a default mode.
- **Control flow via `solvePath`**: classify direct (imm+RIP) vs indirect; resolve via eager-load → constraint solve → `computePossibleValues` (uses LLVM **`computeKnownBits`**). Emit **N=1 → `br`, N=2 → `condbr`, N>2 → `switch`**; filter targets by `isMemPaged`/viability. `RET` compares RSP to a sentinel stack pointer to tell a real return from a ROP dispatch.
- **Calls**: `CallEffects{argRegs, retRegs, volatileRegs, callerStackDelta, memoryEffect}`, `AbiKind ∈ {X64_MSVC, cdecl, stdcall, fastcall}`, `Compat` vs `Strict` modes; IAT/import resolution → typed call; unknown ⇒ conservative volatile-clobber (`undef`). Lifted-fn boundary is `(ctx*, mem*) -> i64` (regs loaded at entry, stored at exit; SSA in between).
- **Optimization = the deobfuscator**: **fixpoint** `O1 → GEPLoadPass → ReplaceTruncWithLoadPass → PromotePseudoStackPass → PromotePseudoMemory` until instruction-count delta 0, then **O2 once**. Ordering is load-bearing (**GEPLoadPass before PromotePseudoMemory**). ~9 custom passes; new PassManager; LLVM 18. Doing this **inline during lifting** (not module-cloning) gave a 400× speedup.

**Sogen** — ideal Oracle backend: deterministic C++ Windows user-mode emulator (Unicorn/icicle/WHP), real-DLL + syscall-level OS modeling (loads ntdll, PEB/TEB, SEH), **full snapshot/restore**, rich instruction/memory/syscall hooks, GDB stub + Python bindings. Snapshot→probe→rollback is its fuzzing workflow.

**llvmkit** — the same author's **pure-Rust LLVM IR** (tracks LLVM 22): type-state builder (positioned/unpositioned, return markers), branded modules, **Braun-algorithm SSA construction**, verifier, new-PM-style pass substrate, partial KnownBits/DemandedBits. **No codegen and no full optimization passes yet** — so the Mergen-style pass pipeline still needs real LLVM (inkwell) for now.

## Target architecture — Cargo workspace

Repo `bin_lift` becomes a workspace. Crate names are `lift-*` (adjustable); the umbrella facade re-exports a batteries-included x86/llvmkit/Windows session.

```
crates/
  lift-core       spine: events, session engine, and ALL seam traits. no LLVM, no x86.
  lift-x86        x86-64 semantics (clean-room), generic over B: IrBuilder.
  lift-llvmkit    PRIMARY IrBuilder backend over llvmkit (pure Rust) + textual-IR export.
  lift-llvm       inkwell backend + the Mergen-style optimization/deobfuscation pipeline (feature-gated; needs LLVM 18).
  lift-platform   Windows-x64 Platform (FS/GS→TEB/PEB, Win64 CC, syscall) + starter ntdll/Win32 signature DB.
  lift-oracle     Oracle adapters (Unicorn/Frida/Sogen/minidump) + a simple in-memory test oracle. (thin at first)
  lift-ffi        stable C-ABI over a concrete x86+llvmkit session; opaque handles, repr(C) events/queries; cbindgen header.
  lift            facade: re-exports a ready x86 / llvmkit / Windows-x64 session.
```

### `lift-core` — the seams (this is the real product)

Everything downstream plugs into these. No LLVM or x86 types leak in.

- **Instruction input (arch-neutral):** `trait InsnView` / `trait OperandView` — a light read-only view a decoder frontend implements (a zydis `FullInstruction` adapter lives in `lift-x86`). Keeps `lift-core` decoder-agnostic without re-modeling all of x86.
- **Streaming engine + suspend/resume protocol:**
  ```
  enum StepOutcome {
      Continue(Events),                       // normal: emitted IR + feedback events
      Suspend(Query, ResumeToken),            // needs the caller/oracle to resolve something
      BlockEnd(Events),                        // terminator lifted
  }
  enum Query {
      BranchVerdict { site, condition, taken_target, fallthrough },   // real or fake?
      IndirectTargets { site, expr },                                 // enumerate/confirm targets
      Memory { addr, size },                                          // concrete bytes?
      Signature { target },                                           // FnSig for a call target
  }
  session.step(insn) -> StepOutcome
  session.resume(token, Answer) -> StepOutcome
  ```
  The query/resume design is deliberately **FFI-clean**: the C side drains a `Query`, probes its emulator, and calls `resume` with an `Answer` — **no Rust callback pointers cross the boundary**.
- **`trait Oracle`** (in-Rust convenience that drives the query loop): `resolve_branch`, `resolve_indirect`, `read_memory`, `snapshot`, `restore`. Sogen/Unicorn/Frida each implement it. Conceptually this is Mergen's `backup_point` with a live source.
- **`trait IrBuilder { type Value; type Type; … }`** — the backend seam. Ergonomics mirror llvmkit (positioned/unpositioned state, branded values). x86 semantics are generic over it; only backend crates touch a concrete IR library.
- **`trait MemoryModel`** — flat address space with a `solve_load`/`solve_store` interface. Regions carry **caller-declared attributes** (`Concrete`, `Symbolic`, `ReadOnly`, `Volatile/Scratch`, `IgnoreWrites`) as a `MemoryFacts` map of ranges — this is where "ignore writes to this section" lives, supplied by the caller, never inferred. The Oracle can also supply `Concrete` bytes live.
- **`trait AssumptionProvider`** — the declared-facts channel that keeps the core assumption-free. Answers, for a given site/value/region: known-constant values, opaque-predicate verdicts (always-taken/never-taken), the canonical entry stack pointer (replacing Mergen's hardcoded `STACKP_VALUE`), loop-carried slots, and target-viability rules. The lifter consults it before falling back to a `Query`. All of it is optional; with no facts declared, the lifter simply stays fully general and asks/queries instead of assuming.
- **`trait Platform`** (remill/Sogen-style OS abstraction) — segment bases (FS/GS→TEB/PEB), syscall convention, active calling conventions, and the `SignatureProvider`(s) in effect. Platform-level defaults are still *declared configuration*, not silent heuristics.
- **`trait SignatureProvider`** — `resolve(addr | name) -> Option<FnSig>`.
- **Shared types:** `Event`/`EventKind` (repr(C)-friendly: `Branch{kind,from,to,taken}`, `Call{from,target,sig}`, `Return`, `MemAccess{addr,size,rw}`, `Syscall{number}`, `SegmentAccess{seg,offset}`, `Fault`, `Unsupported`), `FnSig`, `CallingConvention`/`AbiKind`, `CallEffects{arg_regs,ret_regs,volatile_regs,stack_delta,mem_effect}` (ported from Mergen), `MemoryPolicy`.

### `lift-x86` — semantics (clean-room rewrite)

- **`RegisterFile`**: `reg → current IrBuilder::Value` map (16 GPR + RIP + RFLAGS), flags stored individually with **lazy closures**, sub-register reads/writes via truncate/zext at operand boundaries (Mergen model; keeps IR clean for optimization).
- **Dispatch**: mnemonic → handler table; category modules (`arith`, `bitwise`, `dataxfer`, `shift_rotate`, `ctrlflow`, `stack`, `flags`, `semaphore`, `system`, …), reimplemented against `IrBuilder`. The existing ~80 handlers + flag math in today's `src/lifter/semantics/x86/` and the correct XADD/flag logic are the correctness reference.
- **Effective-address calc + operand get/set** over `MemoryModel`.
- **x86 calling conventions** (Win64/cdecl/stdcall/fastcall) as `CallEffects`, surfaced to `Platform`.
- **`InsnView` adapter** for a decoder — zydis behind `feature = "zydis-frontend"` (iced optional later).
- **Lifted-function boundary**: `(ctx*, mem*) -> i64` — registers loaded from the context struct at entry into the SSA value-map, stored back at exit (Mergen-style; more composable and FFI-friendly than today's flat 35-param signature).

### Control-flow resolution (the headline feature)

At a branch/indirect-jump/call whose target or realness isn't statically obvious, the session runs the **Hybrid** strategy:
1. **Oracle first (if attached):** emit `Query::BranchVerdict`/`IndirectTargets`/`Memory`; the caller snapshots, probes the path(s), rolls back, and answers with a concrete verdict/targets/bytes. Feed those in as `Concrete` facts.
2. **Consult declared facts** via `AssumptionProvider` (known constants, opaque-predicate verdicts, canonical stack pointer for real-vs-ROP `RET`, target-viability). These come from the caller — the engine never applies a built-in packer sentinel or "this is a Themida section" rule.
3. **Emit IR by cardinality** (Mergen's `solvePath` shape): N=1 → `br`; N=2 → `condbr` (extract condition from the value or synthesize `icmp eq`); N>2 → `switch`; unresolved → symbolic + `Unsupported`/`NeedResolution` event.
4. **LLVM cleanup (deobfuscation):** run the Mergen-style pass pipeline to fold opaque predicates, DCE dead edges, and narrow indirect targets via KnownBits. A pluggable `BranchStrategy` also allows `OracleOnly` or `OptimizerOnly` (pure-Mergen) modes.

Streaming stays caller-driven (the emulator/Frida emits the real executed order); Mergen's internal **worklist block-discovery** is retained as inspiration for a later *static* driver mode, out of scope for the first cut.

### IR backend + the llvmkit reconciliation

`IrBuilder` has two intended impls:
- **`lift-llvmkit` (primary):** pure-Rust construction/analysis; can export textual `.ll`.
- **`lift-llvm` (feature-gated):** inkwell/LLVM 18 — the **only** path that can currently run the optimization/deobfuscation pipeline and codegen.

**Reconciliation:** ship the **bridge** first — build IR with llvmkit, export `.ll`, run the Mergen-style pipeline through inkwell/LLVM, and read results/attributes back — so the deobfuscation works end-to-end from day one. In parallel, **grow llvmkit's own passes** (it already has partial KnownBits/DemandedBits and a pass substrate) to shrink and eventually remove the bridge. An `Optimizer` seam makes the two swappable. The pipeline itself is ported directly from Mergen: fixpoint `O1 → GEPLoad → ReplaceTruncWithLoad → PromotePseudoStack → PromotePseudoMemory` (ordering preserved) → `O2` once, applied **inline** to a single module (no cloning).

### `lift-platform`, `lift-oracle`, `lift-ffi`

- **`lift-platform`**: `WindowsX64` implementing `Platform` (FS/GS→TEB/PEB offsets, Win64 CC via `CallEffects`, syscall/SSN model) + a **starter `SignatureProvider`** seeded from real ntdll/Win32 exports (grow over time; Sogen's syscall/PEB/TEB handling is the reference).
- **`lift-oracle`**: the `Oracle` trait lives in core; this crate holds adapters — start with a simple deterministic in-memory test oracle (drives TDD) and stubs/notes for Unicorn (Rust FFI), Frida (single-step), and Sogen (GDB-stub/CLI/C++ FFI).
- **`lift-ffi`**: `#[no_mangle] extern "C"` over a concrete x86 + llvmkit session — opaque handles, `#[repr(C)]` `Event`/`Query`/`Answer` structs, a drain/resume loop, and **fact-declaration entry points** (`lift_declare_memory_fact`, `lift_declare_assumption`, `lift_set_stack_base`, …) so an external tool can push the same caller-declared knowledge across the boundary. cbindgen-generated header. This is the plugin foundation for Frida/IDA/Ghidra/x64dbg.

## Implementation phases (milestones)

1. **Workspace + `lift-core` seams** — traits, event/query/answer types, `CallEffects`/`FnSig`/`MemoryPolicy`, the `Session` engine skeleton with `step`/`resume`. Compiles with a no-op backend + test oracle.
2. **`lift-llvmkit` backend** — implement `IrBuilder` + flat `MemoryModel` over llvmkit; `.ll` export. Golden-IR tests for a handful of ops.
3. **`lift-x86` core semantics** — RegisterFile (lazy flags, sub-reg trunc/zext), dispatch, operands, effective address; port arithmetic/logical/data-transfer/shift/rotate/stack from the old crate as a correctness oracle (TDD, instruction-by-instruction).
4. **Control flow + memory aliasing** — `condbr`/`switch` emission by cardinality; the GEPTracker-style byte-granular `solve_load` (constant-offset → symbolic → select → PHI → jump-table → graceful degradation); `RET` real-vs-ROP; streaming branch `Query`/`resume`.
5. **Calls, ABI, platform** — `CallEffects` marshalling, always-typed calls with CC-synthesized fallback, `WindowsX64` platform + starter ntdll/Win32 DB, syscall handling.
6. **Optimization bridge (`lift-llvm`)** — inkwell backend + the Mergen fixpoint pipeline via `.ll` bridge; wire the Hybrid strategy's cleanup stage; KnownBits indirect-target narrowing.
7. **`lift-ffi` + facade + an example oracle** — C-ABI, cbindgen header, and one working end-to-end example (a small trace lifted with the in-memory oracle, then a Frida/Unicorn stub).

Each phase is independently testable; phases 2–5 follow strict TDD against the old crate and Mergen semantics as oracles.

## What we port vs. reference

- **Reference (don't copy):** old `src/lifter/semantics/x86/*` handlers, `flagops.rs`, `mergen_getters_and_setters.rs` — correctness oracles for the clean-room rewrite. Mergen's `RegisterManager`, `GEPTracker`, `AbiCallContract`, `LifterPipeline` — algorithmic references.
- **Port directly:** the optimization pass pipeline shape/ordering; `CallEffects`/ABI model; the flat-memory + concrete/symbolic policy *mechanism*; the `solvePath` cardinality→IR rule.
- **Explicitly do NOT port** — Mergen's baked-in packer heuristics: the fixed `STACKP_VALUE` sentinel, "ignore writes to Themida sections", prioritized Themida "control cursor" loop-slot discovery, and similar hardcoded assumptions. Every one of these is re-expressed as a **caller-declared fact** (`AssumptionProvider`/`MemoryFacts`), never inferred by the core.
- **New:** the streaming `step`/`resume` protocol, the `Oracle` and `AssumptionProvider` seams, the `IrBuilder`/`MemoryModel`/`Platform`/`SignatureProvider` traits, and the C-ABI surface.

## Verification

- **Unit/semantics (TDD):** per-instruction golden-IR tests; differential check of the new x86 semantics against the old crate's output on the existing examples (`simple_add`, `new_lift_add`, and the 17k-instruction `lift_vmp_trace.bin`).
- **Memory aliasing:** targeted tests for overlapping partial writes / partial reads across slots (the GEPTracker cases).
- **Control flow / oracle:** a deterministic in-memory oracle that scripts branch verdicts + memory; assert the session suspends with the right `Query` and emits `br`/`condbr`/`switch` correctly; assert a scripted "fake branch" is folded away after the LLVM cleanup pass.
- **Assumption-free guarantee:** with **no** facts declared, assert the lifter never silently assumes — e.g. it emits a `Query`/symbolic load rather than dropping a write; then declare an `IgnoreWrites`/opaque-predicate fact and assert the behavior changes accordingly. This pins the "no baked-in heuristics" principle with tests.
- **Deobfuscation end-to-end:** lift the VMP trace with the Hybrid pipeline; confirm opaque predicates are eliminated and indirect jumps become `switch`es (compare against Mergen-style expectations).
- **FFI:** a C smoke test that drives a session through `step`/drain-`Query`/`resume`/`finish` and reads the module.
- **Build:** `lift-llvmkit` path builds with no system LLVM; `lift-llvm`/optimization behind a feature that requires LLVM 18.

## Open items to confirm during implementation

- **llvmkit bridge vs. grow-native** for the optimization pipeline (approach: bridge first, grow llvmkit passes in parallel). This is the one place where "llvmkit primary" and "Mergen-style LLVM deobfuscation" are in tension.
- **Crate granularity** (7 crates may be split further or merged for the first cut — e.g. folding `lift-oracle` into examples initially).
- **Lifted-function boundary** shape (`(ctx*, mem*) -> i64`, Mergen-style) vs. keeping a flatter register-parameter signature.
