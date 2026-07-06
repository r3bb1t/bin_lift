# Milestone 2 — `lift-llvmkit` Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax. These are **integration** tasks against a live dependency (llvmkit v0.0.3), not pure transcription: the **golden `.ll` output is the precise behavioral spec** for each task, and the routing table below is the exact API to call. Implementers bind the wrapper internals against the real API using the proven spike patterns, and iterate until the golden test passes.

**Goal:** Build `crates/lift-llvmkit` — a golden-`.ll`-tested `IrBuilder` backend over llvmkit — and extend the `lift-core` `IrBuilder` trait with the integer data-flow surface the x86 semantics (M3) will need.

**Architecture:** `LlvmkitBuilder` wraps an llvmkit `Module`+`IRBuilder` and implements `lift_core::IrBuilder` with `type Value = llvmkit::ir::Value<'ctx,B>` (width-erased) and `type Block = usize` (index into a label table). The whole build runs inside a `Module::with_new` brand closure; the builder is held as `Option<IRBuilder<..Positioned, Dyn>>` and taken/replaced around positioning + terminators. Data-flow ops route per the table below.

**Tech Stack:** Rust (workspace, edition 2021), `llvmkit = "=0.0.3"` (pure Rust, builds on **stable rustc 1.96.1** — spike-confirmed, no nightly needed).

## Status / prior work

- **Task 1 (spike) is DONE** — committed `002280b`. `crates/lift-llvmkit` exists (Cargo.toml, empty `src/lib.rs`, `tests/spike.rs`), added to the workspace, `llvmkit = "=0.0.3"` builds on stable, and a 3-test spike passes (add, multi-block branch, IntDyn). This plan covers Tasks 2–7. The full spike report is at `.superpowers/sdd/m2-spike-report.md` and the op-routing enumeration is captured in the table below — **implementers should read the spike report first**; keep `tests/spike.rs` as reference (it may be deleted or folded into golden tests in Task 7).

## Global Constraints

- **Rust edition `2021`** for `lift-llvmkit`; dependency `llvmkit = "=0.0.3"` (exact pin — reproducible golden output; the printer was patched recently).
- **No `unsafe`** (`#![forbid(unsafe_code)]` in `lift-llvmkit`, matching lift-core and llvmkit).
- **Value type:** `type Value = llvmkit::ir::Value<'ctx, B>` (fully width-erased; carries its own type via `TypeId`). **Block type:** `type Block = usize` (index into an internal `Vec<BasicBlockLabel>` + `Vec<Option<BasicBlock>>`).
- **Return marker:** use `IRBuilder::new(&m)` (the **`Dyn`** return marker) so `build_ret(value: Value)` accepts erased values (confirmed: `IntoReturnValue<Dyn>` is blanket-impl'd for `V: IsValue`).
- **Constant folding is ON by default** — arithmetic on constants folds away. **Golden `.ll` tests MUST use function parameters (or loaded values) as operands** to see a real instruction; never assert an `add`/`and`/etc. over two constants.
- **Errors:** llvmkit `build_*` return `IrResult` (= `Result<_, IrError>`). The `IrBuilder` trait methods return values, so the backend `.expect("<op>: <detail>")`s them (a construction/width error is a lifter bug). The `Value ↔ IntValue<IntDyn>` round-trips are also fallible → `.expect()`.
- **Whole build inside `with_new`:** `LlvmkitBuilder` borrows `&'m Module<'ctx, B, Unverified>`; it cannot escape the `for<'brand>` closure. Tests build + print inside the closure and return the `.ll` `String` out. (How `Session` composes with this is M3, not M2.)
- **Golden `.ll` format** (from the spike, verbatim): leading `; ModuleID = '<name>'`; `define <ret> @<name>(<params>) {`; block label lines end `:`; 2-space instruction indent; blank line between blocks; unnamed params render `%0, %1`. Normalize only trailing whitespace if needed; never normalize semantics.
- **Commits** end with the trailer:
  ```
  Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
  ```
- **Verify:** `cargo test -p lift-llvmkit` and `cargo test -p lift-core` green; `cargo clippy -p lift-core -p lift-llvmkit -- -D warnings` clean.

## The op-routing table (exact llvmkit v0.0.3 API — the spec for data-flow tasks)

`b` is the positioned `IRBuilder`. Erase any `IntValue`/`ConstantIntValue` to `Value` via `.as_value()`. Bridge an erased `Value` into `IntValue<'ctx, IntDyn, B>` via `IntValue::<IntDyn, B>::try_from(v)?` (or `IntoIntValue`). Make a dyn type via `let ty = m.custom_width_int_type(bits)?;`.

| IrBuilder method | llvmkit call | notes |
|---|---|---|
| `const_int(width, v)` | `m.custom_width_int_type(width)?.const_int_checked(v as i64)?.as_value()` | `const_int` does NOT compile for `IntDyn`; use `_checked`. Fallible. |
| `const_zero(width)` / `const_ones(width)` | `…custom_width_int_type(width)?.const_zero()/.const_all_ones().as_value()` | infallible for IntDyn |
| `iadd/isub/imul` | `b.build_int_add_dyn/…_sub_dyn/…_mul_dyn(a, c, name)?` | erased `Value`→`Value` directly |
| `and/or/xor` | `b.build_int_and_dyn/…_or_dyn/…_xor_dyn(a, c, name)?` | erased directly |
| `not(v)` | `b.build_int_xor_dyn(v, ones, name)?` where `ones = custom_width_int_type(width_of(v))?.const_all_ones().as_value()` | **no dyn `not`**; synthesize. Get width via `v.ty()`/`bit_width` (or pass width in). |
| `shl/lshr/ashr` | `b.build_int_shl_dyn/…_lshr_dyn/…_ashr_dyn(a, c, name)?` | erased directly |
| `urem(a,c)` | `b.build_int_urem::<IntDyn,_,_,_>(IntValue::try_from(a)?, IntValue::try_from(c)?, name)?.as_value()` | **no dyn form**; call typed builder with `W=IntDyn` via round-trip |
| `icmp(pred, a, c)` | `b.build_int_cmp::<IntDyn,_,_,_>(pred.into(), IntValue::try_from(a)?, IntValue::try_from(c)?, name)?.as_value()` | result is `IntValue<bool>` (i1); erase to `Value`. `pred.into()` maps our `IcmpPred`→`llvmkit::ir::IntPredicate` |
| `zext/sext/trunc(v, width)` | branch on `width` vs `v`'s runtime width: `<` → `build_trunc_dyn`, `>` → `build_zext_dyn`/`build_sext_dyn`, `==` → return `v`. Operands are `IntValue<IntDyn>`↔`IntType<IntDyn>`; erase result. | strictly narrower/wider only — the equal-width branch is on YOU |
| `zext_or_trunc(v, width)` | same width-branch as above (zext for widening) | no helper exists |
| `select(cond, a, c)` | `b.build_select::<_, IntValue<'ctx,IntDyn,B>, _>(IntValue::<bool>::try_from(cond)?, IntValue::try_from(a)?, IntValue::try_from(c)?, name)?.as_value()` | `SelectArm` sealed; use `IntValue<IntDyn>` arms |
| `load(addr, width)` | `let p = b.build_int_to_ptr::<IntDyn,_>(IntValue::try_from(addr)?, ptr_ty, "p")?; b.build_load(m.custom_width_int_type(width)?, p, name)?` | `build_load` returns `Value`. `ptr_ty` = the module's pointer type (confirm the constructor, e.g. `m.ptr_type()`). |
| `store(addr, value)` | `let p = b.build_int_to_ptr::<IntDyn,_>(IntValue::try_from(addr)?, ptr_ty, "p")?; b.build_store(value, p)?` | `build_store` accepts erased `Value` (`V: IsValue`) |
| `br(block)` | take builder, `b.build_br(labels[block])?` | `labels[block]: BasicBlockLabel` is `Copy` |
| `cond_br(cond, t, f)` | take builder, `b.build_cond_br(IntValue::<bool>::try_from(cond)?, labels[t], labels[f])?` | cond is `IntoIntValue<bool>` |
| `switch(scrut, default, cases)` | take builder, `build_switch(scrut, labels[default], name)?` then `.add_case(const, labels[case])` per case, then `.finish()` | confirm the exact switch-builder API against source |
| `ret(value)` | take builder, `b.build_ret(value)?` | Dyn marker accepts erased `Value` |
| `unreachable()` | take builder, `b.build_unreachable()?` | consumes |
| `const_addr(va)` | `const_int(64, va)` | addresses are 64-bit constants |
| `new_block(label)` | `let bb = func.append_basic_block(&m, label); self.labels.push(bb.label()); self.blocks.push(Some(bb)); returns index` | store label (Copy) + BasicBlock (to position, consumed once) |
| `position_at(idx)` | `let bb = self.blocks[idx].take().expect("already positioned"); self.builder = Some(IRBuilder::new(&m).position_at_end(bb));` | fresh builder per block |

**`IcmpPred` (lift-core) → `llvmkit::ir::IntPredicate`:** `Eq/Ne/Ult/Ule/Ugt/Uge/Slt/Sle/Sgt/Sge` (identical variant set; the spike census only needs Eq, Ne, Ult, Ugt, Slt but define all ten).

---

### Task 2: `LlvmkitBuilder` scaffold + control-flow `IrBuilder` impl

**Files:** Create `crates/lift-llvmkit/src/builder.rs`; modify `crates/lift-llvmkit/src/lib.rs` (exports, `#![forbid(unsafe_code)]`); Create `crates/lift-llvmkit/tests/control_flow.rs`.

**Interfaces produced:** `LlvmkitBuilder<'m,'ctx,B>` with `fn new(module: &'m Module<'ctx,B,Unverified>, func: FunctionValue<'ctx,Dyn,B>) -> Self` and `fn finish(&self) -> String` (prints the module); `impl lift_core::IrBuilder for LlvmkitBuilder` covering the 8 control-flow methods (`const_addr`, `new_block`, `position_at`, `br`, `cond_br`, `switch`, `ret`, `unreachable`).

- [ ] **Step 1: Establish the value/block/builder plumbing.** Implement the struct (fields: `&'m Module`, `func`, `builder: Option<IRBuilder<'m,'ctx,B,ConstantFolder,Positioned,Dyn>>`, `labels: Vec<BasicBlockLabel>`, `blocks: Vec<Option<BasicBlock<..>>>`), `new`, `new_block`, `position_at`, and the `take_builder()` helper for consuming terminators. Use the spike's proven closure/label/Option patterns (see `.superpowers/sdd/m2-spike-report.md`).
- [ ] **Step 2: Write the failing golden test** (`tests/control_flow.rs`): inside `Module::with_new`, create an `i64 f(i64 %0)` (Dyn builder), drive a `LlvmkitBuilder` to emit entry→`br`→tail→`ret %0`, print, and assert the exact `.ll` (mirror the spike's `spike_multi_block_branch` output). Run: `cargo test -p lift-llvmkit control_flow` → FAIL (methods unimplemented).
- [ ] **Step 3: Implement `const_addr`, `br`, `cond_br`, `switch`, `ret`, `unreachable`** per the routing table (take/replace the `Option<builder>` around terminators). For `switch`, confirm the exact `build_switch(...).add_case(...).finish()` API against `ir_builder.rs` and adjust.
- [ ] **Step 4: Golden tests** for `cond_br` (an `i1` param branching two ways) and `switch` (2–3 cases + default). Assert exact `.ll`. Run: `cargo test -p lift-llvmkit` → PASS.
- [ ] **Step 5: Commit** (`feat(lift-llvmkit): LlvmkitBuilder + control-flow IrBuilder impl` + trailer; add `src/builder.rs`, `src/lib.rs`, `tests/control_flow.rs`).

---

### Task 3: Extend the `lift-core` `IrBuilder` trait + `IcmpPred` + `RecordingBuilder`

**Files:** Modify `crates/lift-core/src/ir.rs` (add methods to the `IrBuilder` trait, add `IcmpPred`, extend `RecordingBuilder`); modify `crates/lift-core/src/lib.rs` (re-export `IcmpPred`).

**Interfaces produced:** `IrBuilder` gains required methods `const_int(&mut self, width: u32, value: u64) -> Value`, `const_zero(width)`, `const_ones(width)`, `iadd/isub/imul(a,b) -> Value`, `and/or/xor(a,b)`, `not(a)`, `shl/lshr/ashr(a,b)`, `urem(a,b)`, `icmp(pred: IcmpPred, a, b) -> Value`, `zext/sext/trunc/zext_or_trunc(v, width) -> Value`, `select(cond, a, b) -> Value`, `load(addr, width) -> Value`, `store(addr, value)`. New `enum IcmpPred { Eq, Ne, Ult, Ule, Ugt, Uge, Slt, Sle, Sgt, Sge }`.

- [ ] **Step 1: Write the failing `RecordingBuilder` test** in `ir.rs` `#[cfg(test)] mod tests`: drive the new ops and assert the op-log (e.g. `iadd` → `"iadd"`, `const_int 64 0x10` → `"const64 0x10"`, `icmp Ult` → `"icmp ult"`, etc. — pick a consistent, testable log format). Run → FAIL (methods don't exist).
- [ ] **Step 2: Add the methods to the `IrBuilder` trait** (required methods, each with a doc comment) and the `IcmpPred` enum (derive `Clone, Copy, PartialEq, Eq, Debug`). Re-export `IcmpPred` from `lib.rs`.
- [ ] **Step 3: Implement all new methods on `RecordingBuilder`** (log a stable string per op, including width/pred where relevant). Run: `cargo test -p lift-core ir` → PASS.
- [ ] **Step 4: Confirm `Session`/`lift-x86`(absent) still compile** — the additions are new required methods; only `RecordingBuilder` and (future) `LlvmkitBuilder` impl `IrBuilder`, so `cargo test -p lift-core` stays green. Run full `cargo test -p lift-core`.
- [ ] **Step 5: Commit** (`feat(lift-core): extend IrBuilder with data-flow ops + IcmpPred` + trailer).

---

### Task 4: Arithmetic / bitwise / shifts in `LlvmkitBuilder`

**Files:** Modify `crates/lift-llvmkit/src/builder.rs`; Create `crates/lift-llvmkit/tests/arith.rs`.

**Interfaces produced:** `LlvmkitBuilder` implements `const_int/const_zero/const_ones`, `iadd/isub/imul`, `and/or/xor/not`, `shl/lshr/ashr`, `urem` per the routing table.

- [ ] **Step 1: Write failing golden tests** (`tests/arith.rs`): a function `i64 f(i64 %0, i64 %1)` whose body does `%0 op %1` for each op, printed and asserted (operands are PARAMS to defeat constant folding). For each op assert the exact instruction line (e.g. `%r = add i64 %0, %1`, `%r = and i64 %0, %1`, `%r = shl i64 %0, %1`, `%r = urem i64 %0, %1`). For `not`, assert `%r = xor i64 %0, -1`. Run → FAIL.
- [ ] **Step 2: Implement the erased-`Value` ops** (`iadd/isub/imul`, `and/or/xor`, `shl/lshr/ashr`) via the `_dyn` builders; `const_int/const_zero/const_ones` via `custom_width_int_type` + `const_int_checked`/`const_zero`/`const_all_ones`.
- [ ] **Step 3: Implement `not` (xor all-ones) and `urem` (round-trip to `IntValue<IntDyn>` + typed `build_int_urem::<IntDyn,_,_,_>`)** per the table. Derive the operand width from the `Value`'s type (`v.ty()` / bit width) for the all-ones constant.
- [ ] **Step 4: Run** `cargo test -p lift-llvmkit arith` → PASS; full `cargo test -p lift-llvmkit` → PASS.
- [ ] **Step 5: Commit** (`feat(lift-llvmkit): integer arithmetic/bitwise/shift ops` + trailer).

---

### Task 5: Compares + casts + select in `LlvmkitBuilder`

**Files:** Modify `crates/lift-llvmkit/src/builder.rs`; Create `crates/lift-llvmkit/tests/cmp_cast_select.rs`.

**Interfaces produced:** `icmp`, `zext/sext/trunc/zext_or_trunc`, `select` implemented per the table.

- [ ] **Step 1: Write failing golden tests:** `icmp` for a couple of predicates (`ult`, `eq`, `slt`) producing `%r = icmp ult i64 %0, %1` (result i1); a width-changing chain `trunc i64→i32`, `zext i32→i64`, `sext i8→i32`, and `zext_or_trunc` for both a narrowing and an equal-width (equal-width must emit NO cast — assert the value passes through); a `select i1 %c, i64 %0, i64 %1`. Run → FAIL.
- [ ] **Step 2: Implement `icmp`** (map `IcmpPred`→`llvmkit::ir::IntPredicate` via a `From`/match; round-trip operands to `IntValue<IntDyn>`; `build_int_cmp::<IntDyn,_,_,_>`; erase result i1 to `Value`).
- [ ] **Step 3: Implement the casts** with the width branch (`dst < src` → `build_trunc_dyn`; `dst > src` → `build_zext_dyn`/`build_sext_dyn`; `dst == src` → return input unchanged). `zext_or_trunc` uses zext for widening. Operands/results round-trip `Value`↔`IntValue<IntDyn>`/`IntType<IntDyn>`.
- [ ] **Step 4: Implement `select`** (round-trip cond→`IntValue<bool>`, arms→`IntValue<IntDyn>`, `build_select::<_, IntValue<IntDyn,B>, _>`, erase result). Run: `cargo test -p lift-llvmkit` → PASS.
- [ ] **Step 5: Commit** (`feat(lift-llvmkit): compares, casts, select` + trailer).

---

### Task 6: Minimal memory primitives (`load` / `store`)

**Files:** Modify `crates/lift-llvmkit/src/builder.rs`; Create `crates/lift-llvmkit/tests/memory.rs`.

**Interfaces produced:** `load(addr, width) -> Value`, `store(addr, value)` — via `inttoptr` to the module pointer type, then `build_load`/`build_store`.

- [ ] **Step 1: Confirm the pointer-type constructor** against `ir_builder.rs`/`module.rs` (the routing table assumes `m.ptr_type()`/`PointerType` — verify the exact name; opaque pointers). Note the finding in the report.
- [ ] **Step 2: Write a failing golden test** (`tests/memory.rs`): a function that `store`s a param to an address (another param) and `load`s it back at a given width; assert the `.ll` shows `inttoptr`, `store`, `inttoptr`, `load` (exact lines). Operands are params (no folding). Run → FAIL.
- [ ] **Step 3: Implement `load`/`store`** per the table (`build_int_to_ptr::<IntDyn,_>` from the address `Value`, then `build_load(custom_width_int_type(width)?, ptr, name)` / `build_store(value, ptr)`). Run → PASS.
- [ ] **Step 4: Note in the crate docs** that this is a *flat* raw load/store; the `MemoryModel`/`solve_load` aliasing layer is M4.
- [ ] **Step 5: Commit** (`feat(lift-llvmkit): minimal flat load/store primitives` + trailer).

---

### Task 7: Facade, `verify()`, end-to-end golden test, docs

**Files:** Modify `crates/lift-llvmkit/src/lib.rs` (re-exports, crate docs); Create `crates/lift-llvmkit/tests/end_to_end.rs`; delete or fold `tests/spike.rs`.

- [ ] **Step 1: End-to-end golden test:** build a realistic small function resembling a lifted `ADD` (load two "registers" from a state region or take params, `iadd`, compute a couple of flags via `icmp`/`and`, `store`/`ret`), print, assert the `.ll`, AND run `m.verify()` (expect `Ok`). This exercises the whole surface together.
- [ ] **Step 2: Facade & docs:** re-export `LlvmkitBuilder` (and a small helper for the `with_new` entry, e.g. `fn build_module<R>(name, |builder| -> R) -> R` if it reads cleanly) from `lib.rs`. Document the **brand-closure usage contract** (whole build inside `with_new`), the constant-folding golden-test caveat, and the M3 (`Session` composition) / M4 (`MemoryModel`) forward points.
- [ ] **Step 3: Remove/retire `tests/spike.rs`** (its cases are now covered by the golden tests) — or convert any still-unique probe into a kept test. Run full `cargo test -p lift-llvmkit`.
- [ ] **Step 4: Full verification gate:** `cargo test -p lift-core` and `cargo test -p lift-llvmkit` green; `cargo clippy -p lift-core -p lift-llvmkit -- -D warnings` clean.
- [ ] **Step 5: Commit** (`feat(lift-llvmkit): facade, verify integration, end-to-end golden test` + trailer).

## Verification (whole milestone)

- **Golden `.ll` tests** are the primary evidence — each op/group asserts exact printed IR; the end-to-end test also `verify()`s.
- **Cross-crate:** `cargo test -p lift-core` still green after the Task 3 trait extension (RecordingBuilder logs new ops); `cargo test -p lift-llvmkit` green; workspace clippy `-D warnings` clean; whole pure-Rust workspace builds on stable 1.96.1.
- **Constant-folding discipline:** confirm no arithmetic golden test uses two constant operands (they would fold and the assertion would be vacuous).

## Self-review / notes

- **Integration, not transcription:** wrapper internals (exact `Option`-builder dance, switch-builder call chain, pointer-type constructor) are bound against the live API using the spike patterns; the golden `.ll` is the exact behavioral spec, so a wrong binding fails its test. Implementers use **standard-tier models** (integration/judgment), read the spike report first, and consult the routing table.
- **Deferred (later milestones):** div/sdiv/srem (only `urem` is in the M1 census; add others in M3 if a handler needs them), `MemoryModel`/`solve_load` aliasing (M4), `Session` composition with the brand closure (M3), calls/GEP-for-structs (M5), optimization passes (M6, via inkwell — llvmkit has none yet).
- **If `build_ret` on the Dyn builder runtime-errors on return-type mismatch** (it checks `value.ty()` == function's declared return type), ensure test functions declare an `i64` return and return an `i64`-wide value; widen/narrow the returned `Value` to 64 bits first if needed.
