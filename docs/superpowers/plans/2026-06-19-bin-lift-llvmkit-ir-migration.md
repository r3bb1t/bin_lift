# bin_lift llvmkit IR Migration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace bin_lift's Inkwell-based IR construction with llvmkit's branded, typestate-driven IR API while preserving the current x86 lifting behavior and producing verified textual LLVM IR.

**Architecture:** The migration inverts ownership around `Module::with_new`: callers run compilation inside a fresh branded module closure, and `Compiler`/`LifterX86` borrow `Module<'ctx, B, Unverified>` instead of owning an Inkwell `Context`, `Module`, and `Builder`. The first cut targets verified `.ll` generation only; target-machine optimization and object emission are handled by the companion backend-gap plan.

**Tech Stack:** Rust 2021 bin_lift, Zydis 4.1.1, local llvmkit workspace crate, llvmkit `Module<'ctx, B, Unverified>`, `IRBuilder`, branded values/types, and llvmkit verifier.

---

## Preconditions

This plan assumes llvmkit exposes the final safe public shape:

```rust
Module::with_new::<_, _, _>(name, |module| { ... });
IRBuilder::new_for::<R>(&module);
module.i64_type();
module.add_function::<R>(name, fn_ty, Linkage::External)?;
```

If llvmkit still requires `module.core()` for construction, land the llvmkit `ModuleCore` privacy/capability cutover first. Do not migrate bin_lift onto the raw-core escape hatch.

## File Structure

- Modify: `Cargo.toml` — replace Inkwell dependency with local llvmkit dependency.
- Modify: `src/compiler/error.rs` — remove Inkwell error types and accept llvmkit `IrError`.
- Modify: `src/lifter/error.rs` — same error migration for lifter code.
- Modify: `src/lifter/definintions.rs` — replace Inkwell `BasicValueEnum`/`IntValue`/`FloatValue` wrappers with branded llvmkit value wrappers.
- Modify: `src/compiler/mod.rs` — replace `Context` ownership with branded module borrowing, create the protected function through llvmkit, and return verified IR text.
- Modify: `src/lifter/mod.rs` — replace `Context`, `Builder`, and owned `Module` fields with `&Module<Unverified>` and llvmkit `IRBuilder`.
- Modify: `src/compiler/contexts.rs` — create initial register constants from llvmkit integer types.
- Modify: `src/lifter/getters.rs` — port immediate/register/memory operand reads.
- Modify: `src/lifter/setters.rs` — port register and flag writes.
- Modify: `src/lifter/mergen_getters_and_setters.rs` — port stack-memory alloca, GEP, load, and store.
- Modify: `src/lifter/flagops.rs` and `src/lifter/semantics/x86/*.rs` — port arithmetic/logical/select/cast/branch semantics.
- Modify: `examples/simple_add.rs`, `examples/new_lift_add.rs`, `examples/lift_vmp_trace.rs` — run compilation inside `Module::with_new` and print `format!("{verified}")`.
- Test: existing examples plus a new integration test file `tests/llvmkit_ir_generation.rs`.

---

### Task 1: Switch the dependency and errors

**Files:**
- Modify: `Cargo.toml:11-15`
- Modify: `src/compiler/error.rs:1-20`
- Modify: `src/lifter/error.rs:1-20`

- [ ] **Step 1: Replace the dependency**

Change `Cargo.toml` dependencies to:

```toml
[dependencies]
llvmkit = { path = "../rllvm/.worktrees/brand-first-pass-safety/llvmkit" }
thiserror = "2"
zydis = { version = "4.1.1", features = ["default"] }
```

- [ ] **Step 2: Update compiler errors**

Replace Inkwell imports in `src/compiler/error.rs` with llvmkit:

```rust
use llvmkit::ir::IrError;
use thiserror::Error;

pub(crate) type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Ir(#[from] IrError),

    #[error("optimization backend is not configured for llvmkit textual IR output")]
    OptimizationBackendUnavailable,

    #[error("{0}")]
    Message(String),
}
```

- [ ] **Step 3: Update lifter errors**

Replace Inkwell imports in `src/lifter/error.rs` with llvmkit:

```rust
use llvmkit::ir::IrError;
use thiserror::Error;

use super::ExtendedRegisterEnum;

pub(crate) type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Ir(#[from] IrError),

    #[error("Tried to convert zydis::Register::NONE to something")]
    RegisterConverError,

    #[error("Tried to unwrap a register which doesn't exist. {0:?}")]
    RegisterDoesntExist(ExtendedRegisterEnum),

    #[error("Unsupported instruction {0:?}")]
    UnsupportedInstr(zydis::Mnemonic),

    #[error("Unsupported operand shape")]
    UnsupportedOperand,

    #[error("Tried to convert a value to an incompatible kind")]
    ConvertError,
}
```

- [ ] **Step 4: Check expected compiler errors**

Run:

```bash
cargo check
```

Expected: many unresolved `inkwell::*` imports remain. That is correct for this task.

---

### Task 2: Replace the local value/type wrapper layer

**Files:**
- Modify: `src/lifter/definintions.rs:1-104`
- Modify callsites later through Tasks 5-8.

- [ ] **Step 1: Replace wrapper imports and types**

Replace `src/lifter/definintions.rs` with:

```rust
use llvmkit::ir::{FloatDyn, FloatType, FloatValue, IntDyn, IntType, IntValue, ModuleBrand};

use super::{Error, Result};

#[derive(Debug, Clone, Copy)]
pub(crate) enum LiftValue<'ctx, B: ModuleBrand> {
    Int(IntValue<'ctx, IntDyn, B>),
    Float(FloatValue<'ctx, FloatDyn, B>),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum LiftType<'ctx, B: ModuleBrand> {
    Int(IntType<'ctx, IntDyn, B>),
    Float(FloatType<'ctx, FloatDyn, B>),
}

impl<'ctx, B: ModuleBrand> From<IntValue<'ctx, IntDyn, B>> for LiftValue<'ctx, B> {
    fn from(value: IntValue<'ctx, IntDyn, B>) -> Self {
        Self::Int(value)
    }
}

impl<'ctx, B: ModuleBrand> From<FloatValue<'ctx, FloatDyn, B>> for LiftValue<'ctx, B> {
    fn from(value: FloatValue<'ctx, FloatDyn, B>) -> Self {
        Self::Float(value)
    }
}

impl<'ctx, B: ModuleBrand> From<IntType<'ctx, IntDyn, B>> for LiftType<'ctx, B> {
    fn from(value: IntType<'ctx, IntDyn, B>) -> Self {
        Self::Int(value)
    }
}

impl<'ctx, B: ModuleBrand> From<FloatType<'ctx, FloatDyn, B>> for LiftType<'ctx, B> {
    fn from(value: FloatType<'ctx, FloatDyn, B>) -> Self {
        Self::Float(value)
    }
}

impl<'ctx, B: ModuleBrand> TryFrom<LiftValue<'ctx, B>> for IntValue<'ctx, IntDyn, B> {
    type Error = Error;

    fn try_from(value: LiftValue<'ctx, B>) -> Result<Self> {
        match value {
            LiftValue::Int(value) => Ok(value),
            LiftValue::Float(_) => Err(Error::ConvertError),
        }
    }
}

impl<'ctx, B: ModuleBrand> TryFrom<LiftType<'ctx, B>> for IntType<'ctx, IntDyn, B> {
    type Error = Error;

    fn try_from(value: LiftType<'ctx, B>) -> Result<Self> {
        match value {
            LiftType::Int(value) => Ok(value),
            LiftType::Float(_) => Err(Error::ConvertError),
        }
    }
}
```

- [ ] **Step 2: Keep compatibility aliases for internal migration only**

At the bottom of the same file, add temporary internal aliases so callsites can migrate in small steps:

```rust
pub(crate) type PossibleLLVMValueEnum<'ctx, B> = LiftValue<'ctx, B>;
pub(crate) type PossibleLLVMTypeEnum<'ctx, B> = LiftType<'ctx, B>;
```

- [ ] **Step 3: Check expected errors**

Run:

```bash
cargo check
```

Expected: generic argument errors at old `PossibleLLVMValueEnum<'ctx>` and `PossibleLLVMTypeEnum<'ctx>` callsites. Fix those in later tasks rather than weakening the aliases.

---

### Task 3: Invert compiler ownership around `Module::with_new`

**Files:**
- Modify: `src/compiler/mod.rs:1-193`
- Modify: `examples/simple_add.rs:26-49`

- [ ] **Step 1: Replace compiler imports**

Use these imports at the top of `src/compiler/mod.rs`:

```rust
use crate::lifter::LifterX86;
use crate::lifter::semantics::Lifter;
use crate::miscellaneous::ExtendedRegisterEnum;

use error::Error;
use llvmkit::ir::{
    FunctionValue, IRBuilder, IntDyn, IntValue, IrResult, Linkage, Module, ModuleBrand, Type,
    Unverified,
};
use zydis::{FullInstruction, InstructionAttributes, MachineMode, Register};
```

- [ ] **Step 2: Replace `Compiler` struct**

Use this shape:

```rust
pub struct Compiler<'m, 'ctx, B: ModuleBrand> {
    module: &'m Module<'ctx, B, Unverified>,
    mode: MachineMode,
    pub lifter: LifterX86<'m, 'ctx, B>,
    func_value: FunctionValue<'ctx, IntDyn, B>,
}
```

- [ ] **Step 3: Replace constructor**

```rust
impl<'m, 'ctx, B> Compiler<'m, 'ctx, B>
where
    B: ModuleBrand + 'ctx,
{
    pub fn new_with_x86_lifter(
        module: &'m Module<'ctx, B, Unverified>,
        mode: MachineMode,
        runtime_address: Option<u64>,
    ) -> Result<Self> {
        let func_value = create_func(module, mode)?;
        let lifter = LifterX86::new(module, mode, func_value, runtime_address)?;

        Ok(Self {
            module,
            mode,
            lifter,
            func_value,
        })
    }
}
```

- [ ] **Step 4: Add verified textual output helper**

Add this method inside the same `impl` block:

```rust
pub fn lift_to_ir_text(
    module: Module<'ctx, B, Unverified>,
    instructions: &[FullInstruction],
    mode: MachineMode,
    runtime_address: Option<u64>,
) -> Result<String> {
    let compiler = Compiler::new_with_x86_lifter(&module, mode, runtime_address)?;
    compiler.lift_function(instructions)?;
    let verified = module.verify()?;
    Ok(format!("{verified}"))
}
```

- [ ] **Step 5: Replace the example entrypoint**

In `examples/simple_add.rs`, replace the Inkwell context flow with:

```rust
use llvmkit::ir::Module;
use std::{error::Error, time::Instant};
use zydis::Decoder;
use zydis2llvmir::compiler::Compiler;

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mode = zydis::MachineMode::LONG_64;
    let decoder = Decoder::new64();

    let mut all_instructions = Vec::new();
    for instruction_info in decoder.decode_all(&TEST_ADDITION_NOT_PATCHED_64, 0) {
        let (_ip, _raw_bytes, instruction) = instruction_info?;
        all_instructions.push(instruction);
    }

    let ir = Module::with_new::<_, _, _>("protected", |module| {
        Compiler::lift_to_ir_text(module, &all_instructions, mode, None)
    })?;

    eprintln!("{ir}");
    println!("Elapsed: {:?}", start_time.elapsed());
    Ok(())
}
```

- [ ] **Step 6: Check expected errors**

Run:

```bash
cargo check --example simple_add
```

Expected: errors in `LifterX86::new`, `create_func`, and value wrappers. Continue with Tasks 4-8.

---

### Task 4: Port protected function creation

**Files:**
- Modify: `src/compiler/mod.rs:268-315`

- [ ] **Step 1: Replace `create_func` signature**

```rust
pub(crate) fn create_func<'ctx, B>(
    module: &Module<'ctx, B, Unverified>,
    mode: MachineMode,
) -> IrResult<FunctionValue<'ctx, IntDyn, B>>
where
    B: ModuleBrand + 'ctx,
{
```

- [ ] **Step 2: Replace function body**

```rust
    let example_reg = Register::AX.largest_enclosing(mode);
    let int_type = module.custom_width_int_type(example_reg.width(mode).into())?;

    let args_count = ALL_REGS_IN_MIN_SIZE.len() + CPU_FLAGS.len();
    let mut args = Vec::<Type<'ctx, B>>::with_capacity(args_count);

    for _ in ALL_REGS_IN_MIN_SIZE {
        args.push(int_type.as_type());
    }

    for _ in CPU_FLAGS {
        args.push(module.bool_type().as_type());
    }

    let fn_type = module.fn_type(int_type.as_type(), args, false);
    let fn_val = module.add_function::<IntDyn>("protected", fn_type, Linkage::External)?;

    for (id, reg) in ALL_REGS_IN_MIN_SIZE.into_iter().enumerate() {
        let slot = u32::try_from(id).map_err(|_| llvmkit::ir::IrError::InvalidOperation {
            message: "register argument index exceeds u32::MAX",
        })?;
        let name = reg.largest_enclosing(mode).static_string().unwrap();
        fn_val.param(slot)?.set_name(Some(name));
    }

    for (id, cpu_flag) in CPU_FLAGS.into_iter().enumerate() {
        let raw_slot = ALL_REGS_IN_MIN_SIZE.len() + id;
        let slot = u32::try_from(raw_slot).map_err(|_| llvmkit::ir::IrError::InvalidOperation {
            message: "flag argument index exceeds u32::MAX",
        })?;
        fn_val.param(slot)?.set_name(Some(&format!("{cpu_flag:?}")));
    }

    Ok(fn_val)
}
```

- [ ] **Step 3: Check function creation only**

Run:

```bash
cargo check -p bin_lift --lib
```

Expected: remaining errors are in `LifterX86` and semantics modules, not `create_func`.

---

### Task 5: Port `LifterX86` state and setup

**Files:**
- Modify: `src/lifter/mod.rs:1-160`

- [ ] **Step 1: Replace imports**

```rust
use crate::miscellaneous::ExtendedRegisterEnum;
use std::cell::Cell;
use std::collections::HashMap;

use definintions::{LiftType, LiftValue};
use llvmkit::ir::{
    ConstantFolder, FunctionValue, IRBuilder, IntDyn, IntType, IntValue, Module, ModuleBrand,
    PointerValue, Positioned, Unverified,
};
use zydis::{ffi::MemoryInfo, MachineMode, Register, RegisterClass};
```

- [ ] **Step 2: Replace struct**

```rust
pub struct LifterX86<'m, 'ctx, B: ModuleBrand> {
    pub module: &'m Module<'ctx, B, Unverified>,
    pub builder: IRBuilder<'ctx, ConstantFolder, Positioned, IntDyn>,
    pub mode: MachineMode,
    pub(super) regs_hashmap: HashMap<ExtendedRegisterEnum, LiftValue<'ctx, B>>,
    pub stackmemory: PointerValue<'ctx, B>,
    pub runtime_address: Option<Cell<u64>>,
}
```

- [ ] **Step 3: Replace constructor**

```rust
impl<'m, 'ctx, B> LifterX86<'m, 'ctx, B>
where
    B: ModuleBrand + 'ctx,
{
    pub fn new(
        module: &'m Module<'ctx, B, Unverified>,
        mode: MachineMode,
        func_value: FunctionValue<'ctx, IntDyn, B>,
        runtime_address: Option<u64>,
    ) -> Result<Self> {
        let regs_hashmap = prep_regs_hashmap_experimental(func_value, mode)?;
        let entry_basic_block = func_value.append_basic_block("entry");
        let builder = IRBuilder::new_for::<IntDyn>(module).position_at_end(entry_basic_block);

        const STACK_SIZE: u64 = 0x1000;
        let stackmemory = builder.build_array_alloca(
            module.i128_type(),
            module.i128_type().const_int(STACK_SIZE),
            "stackmemory",
        )?;

        Ok(Self {
            module,
            builder,
            mode,
            regs_hashmap,
            stackmemory,
            runtime_address: runtime_address.map(Cell::new),
        })
    }
}
```

- [ ] **Step 4: Replace type helpers**

```rust
pub(super) fn get_max_int_type(&self) -> IrResult<IntType<'ctx, IntDyn, B>> {
    let example_reg = Register::AX.largest_enclosing(self.mode);
    self.module
        .custom_width_int_type(example_reg.width(self.mode).into())
}

pub(crate) fn get_register_type(&self, reg: Register) -> IrResult<LiftType<'ctx, B>> {
    let ty = match reg.class() {
        RegisterClass::GPR8 => self.module.i8_type().as_dyn().into(),
        RegisterClass::GPR16 => self.module.i16_type().as_dyn().into(),
        RegisterClass::GPR32 => self.module.i32_type().as_dyn().into(),
        RegisterClass::GPR64 => self.module.i64_type().as_dyn().into(),
        RegisterClass::MMX => self.module.f64_type().as_dyn().into(),
        RegisterClass::XMM => self.module.i128_type().as_dyn().into(),
        RegisterClass::IP | RegisterClass::FLAGS => self
            .module
            .custom_width_int_type(reg.width(self.mode).into())?
            .into(),
        RegisterClass::SEGMENT => self.module.i16_type().as_dyn().into(),
        RegisterClass::INVALID => return Err(Error::RegisterConverError),
        _ => self
            .module
            .custom_width_int_type(reg.width(self.mode).into())?
            .into(),
    };
    Ok(ty)
}
```

- [ ] **Step 5: Replace dynamic zext/trunc helper**

```rust
pub(crate) fn create_z_ext_or_trunc(
    &self,
    value: IntValue<'ctx, IntDyn, B>,
    dest: IntType<'ctx, IntDyn, B>,
) -> Result<IntValue<'ctx, IntDyn, B>> {
    let source_bits = value.ty().bits();
    let dest_bits = dest.bits();

    if source_bits < dest_bits {
        Ok(self.builder.build_zext_dyn(value, dest, "")?)
    } else if source_bits > dest_bits {
        Ok(self.builder.build_trunc_dyn(value, dest, "")?)
    } else {
        Ok(value)
    }
}
```

- [ ] **Step 6: Replace register parameter map**

```rust
fn prep_regs_hashmap_experimental<'ctx, B>(
    fn_val: FunctionValue<'ctx, IntDyn, B>,
    mode: MachineMode,
) -> Result<HashMap<ExtendedRegisterEnum, LiftValue<'ctx, B>>>
where
    B: ModuleBrand + 'ctx,
{
    let mut registers_hashmap = HashMap::new();
    let regs = crate::compiler::ALL_REGS_IN_MIN_SIZE.map(|reg| reg.largest_enclosing(mode));

    for (id, reg) in regs.into_iter().enumerate() {
        let slot = u32::try_from(id).map_err(|_| Error::UnsupportedOperand)?;
        let value: IntValue<'ctx, IntDyn, B> = fn_val.param(slot)?.try_into()?;
        registers_hashmap.insert(reg.into(), value.into());
    }

    let mut last_index = regs.len() - 1;
    for cpu_flag in crate::compiler::CPU_FLAGS {
        last_index += 1;
        let slot = u32::try_from(last_index).map_err(|_| Error::UnsupportedOperand)?;
        let value: IntValue<'ctx, IntDyn, B> = fn_val.param(slot)?.try_into()?;
        registers_hashmap.insert(cpu_flag, value.into());
    }

    Ok(registers_hashmap)
}
```

- [ ] **Step 7: Check lifter module**

Run:

```bash
cargo check -p bin_lift --lib
```

Expected: errors are now concentrated in getters/setters/semantics builder calls.

---

### Task 6: Port register getters and setters

**Files:**
- Modify: `src/lifter/getters.rs:1-180`
- Modify: `src/lifter/setters.rs:1-200`

- [ ] **Step 1: Replace getter signatures**

Use branded dynamic integer values in getter APIs:

```rust
pub(super) fn load_single_int_op(
    &self,
    operand: &DecodedOperand,
    possible_size: u16,
) -> Result<IntValue<'ctx, IntDyn, B>> {
    self.load_single_op(operand, possible_size)?.try_into()
}

pub(super) fn load_flag<T>(&self, cpu_flag: T) -> Result<IntValue<'ctx, IntDyn, B>>
where
    T: Borrow<ExtendedRegisterEnum>,
{
    let key = cpu_flag.borrow();
    self.regs_hashmap
        .get(key)
        .copied()
        .ok_or(Error::RegisterDoesntExist(*key))?
        .try_into()
}
```

- [ ] **Step 2: Replace immediate creation**

```rust
fn load_imm_internal(
    &self,
    imm: &ImmediateInfo,
    possible_size: u16,
) -> Result<IntValue<'ctx, IntDyn, B>> {
    let ty = self.module.custom_width_int_type(u32::from(possible_size))?;
    Ok(ty.const_int_raw(imm.value, imm.is_signed))
}
```

If llvmkit exposes only `const_int`, use unsigned bit-pattern construction for now:

```rust
Ok(ty.const_int(imm.value))
```

- [ ] **Step 3: Replace setter signatures**

```rust
pub(super) fn store_op<T>(&mut self, op: &DecodedOperand, value: T) -> Result<()>
where
    LiftValue<'ctx, B>: From<T>,
{
    let val = LiftValue::from(value);
    match &op.kind {
        DecodedOperandKind::Reg(reg) => self.store_reg(*reg, val.try_into()?)?,
        DecodedOperandKind::Mem(memory_info) => self.mergen_store_mem(memory_info, val)?,
        _ => return Err(Error::UnsupportedOperand),
    }
    Ok(())
}

pub(super) fn store_reg(
    &mut self,
    reg: Register,
    mut val: IntValue<'ctx, IntDyn, B>,
) -> Result<()> {
```

- [ ] **Step 4: Replace internal register store**

```rust
fn store_register_internal<T>(&mut self, reg: Register, val: T)
where
    LiftValue<'ctx, B>: From<T>,
{
    let key = self.get_register_largest_enclosing(&reg);
    self.regs_hashmap.insert(key.into(), LiftValue::from(val));
}

pub(super) fn store_cpu_flag(
    &mut self,
    flag: ExtendedRegisterEnum,
    val: IntValue<'ctx, IntDyn, B>,
) {
    self.regs_hashmap.insert(flag, val.into());
}
```

- [ ] **Step 5: Update method receivers**

Change mutating lifter methods that update register state from `&self` to `&mut self`:

```rust
pub(super) fn store_op(...)
pub(super) fn store_reg(...)
fn set_val_to_sub_reg_8b(...)
fn set_val_to_sub_reg_16b(...)
fn store_register_internal(...)
pub(super) fn store_cpu_flag(...)
```

Propagate `&mut self` to semantics methods that call these setters.

- [ ] **Step 6: Check register layer**

Run:

```bash
cargo check -p bin_lift --lib
```

Expected: remaining errors are builder method names/types and semantics receivers.

---

### Task 7: Port memory helpers

**Files:**
- Modify: `src/lifter/mergen_getters_and_setters.rs:1-133`

- [ ] **Step 1: Replace imports**

```rust
use super::{LiftValue, LifterX86, Result};
use llvmkit::ir::{IntDyn, IntValue, ModuleBrand, PointerValue};
use zydis::{ffi::MemoryInfo, Register};
```

- [ ] **Step 2: Replace `mergen_store_mem`**

```rust
pub(super) fn mergen_store_mem(
    &mut self,
    mem: &MemoryInfo,
    val: LiftValue<'ctx, B>,
) -> Result<()> {
    let pointer = self.mergen_calculate_memory_operand(mem)?;
    match val {
        LiftValue::Int(value) => self.builder.build_store(pointer, value)?,
        LiftValue::Float(value) => self.builder.build_store(pointer, value)?,
    };
    Ok(())
}
```

- [ ] **Step 3: Replace `mergen_load_mem`**

```rust
pub(super) fn mergen_load_mem(
    &self,
    mem: &MemoryInfo,
    possible_size: u32,
) -> Result<IntValue<'ctx, IntDyn, B>> {
    let pointer = self.mergen_calculate_memory_operand(mem)?;
    let load_type = self.module.custom_width_int_type(possible_size)?;
    Ok(self.builder.build_int_load_dyn(load_type, pointer, "")?)
}
```

- [ ] **Step 4: Replace memory operand GEP**

```rust
pub(crate) fn mergen_calculate_memory_operand(
    &self,
    mem: &MemoryInfo,
) -> Result<PointerValue<'ctx, B>> {
    let effective_address = self.mergen_get_effective_address(mem)?;
    let memory_operand = if mem.segment == Register::GS {
        return Err(super::Error::UnsupportedOperand);
    } else {
        self.stackmemory
    };

    Ok(self.builder.build_gep(
        self.module.i8_type(),
        memory_operand,
        [effective_address.as_dyn()],
        "",
    )?)
}
```

- [ ] **Step 5: Replace effective-address arithmetic**

```rust
pub(crate) fn mergen_get_effective_address(
    &self,
    mem: &MemoryInfo,
) -> Result<IntValue<'ctx, IntDyn, B>> {
    let i64_ty = self.module.i64_type().as_dyn();

    let base_value = if mem.base != Register::NONE {
        let base: IntValue<'ctx, IntDyn, B> = self.get_register(mem.base)?.try_into()?;
        Some(self.create_z_ext_or_trunc(base, i64_ty)?)
    } else {
        None
    };

    let scale_value = if mem.index != Register::NONE {
        let index: IntValue<'ctx, IntDyn, B> = self.get_register(mem.index)?.try_into()?;
        let index = self.create_z_ext_or_trunc(index, i64_ty)?;
        if mem.scale > 1 {
            let scale = i64_ty.const_int(u64::from(mem.scale));
            Some(self.builder.build_int_mul(index, scale, "")?)
        } else {
            Some(index)
        }
    } else {
        None
    };

    let mut effective_address = match (base_value, scale_value) {
        (Some(base), Some(scale)) => self.builder.build_int_add(base, scale, "effective_address_")?,
        (Some(base), None) => base,
        (None, Some(scale)) => scale,
        (None, None) => i64_ty.const_zero(),
    };

    if mem.disp.displacement != 0 {
        let displacement = i64_ty.const_int(mem.disp.displacement.unsigned_abs());
        effective_address = if mem.disp.displacement.is_negative() {
            self.builder.build_int_sub(effective_address, displacement, "")?
        } else {
            self.builder.build_int_add(effective_address, displacement, "")?
        };
    }

    Ok(effective_address)
}
```

- [ ] **Step 6: Check memory helpers**

Run:

```bash
cargo check -p bin_lift --lib
```

Expected: errors move to instruction semantics modules.

---

### Task 8: Port arithmetic, flags, selects, and casts

**Files:**
- Modify: `src/lifter/flagops.rs`
- Modify: `src/lifter/semantics/x86/binary.rs`
- Modify: `src/lifter/semantics/x86/bitbyte.rs`
- Modify: `src/lifter/semantics/x86/cmov.rs`
- Modify: `src/lifter/semantics/x86/convert.rs`
- Modify: `src/lifter/semantics/x86/dataxfer.rs`
- Modify: `src/lifter/semantics/x86/flagop.rs`
- Modify: `src/lifter/semantics/x86/logical.rs`
- Modify: remaining `src/lifter/semantics/x86/*.rs` files with builder calls.

- [ ] **Step 1: Replace imports in semantics files**

Use this pattern:

```rust
use llvmkit::ir::{IntDyn, IntPredicate, IntValue, ModuleBrand};
use zydis::{Instruction, Mnemonic, Operands};
```

- [ ] **Step 2: Replace `IntValue<'ctx>` signatures**

Example from `flagops.rs`:

```rust
pub(super) fn compute_aux_flag(
    &self,
    lhs: IntValue<'ctx, IntDyn, B>,
    rhs: IntValue<'ctx, IntDyn, B>,
    result: IntValue<'ctx, IntDyn, B>,
) -> Result<IntValue<'ctx, IntDyn, B>> {
```

- [ ] **Step 3: Replace integer compares**

Current Inkwell:

```rust
let zf = self.builder.build_int_compare(
    IntPredicate::EQ,
    value,
    value.get_type().const_zero(),
    "computed_zf_",
)?;
```

llvmkit:

```rust
let zf = self.builder.build_icmp(
    IntPredicate::Eq,
    value,
    value.ty().const_zero(),
    "computed_zf_",
)?;
```

Or use convenience methods where readable:

```rust
let zf = self.builder.build_icmp_eq(value, value.ty().const_zero(), "computed_zf_")?;
let cf = self.builder.build_icmp_ult(result, lhs, "")?;
```

- [ ] **Step 4: Replace bitwise and arithmetic ops**

Current Inkwell:

```rust
let result = builder.build_int_add(lhs, rhs, "real_add_")?;
let masked = builder.build_and(result, mask, "masked")?;
let not_cf = builder.build_not(cf, "not_cf")?;
```

llvmkit:

```rust
let result = builder.build_int_add(lhs, rhs, "real_add_")?;
let masked = builder.build_and(result, mask, "masked")?;
let not_cf = builder.build_int_not(cf, "not_cf")?;
```

- [ ] **Step 5: Replace select conversion**

Current Inkwell:

```rust
let result = builder
    .build_select(condition, rhs, lhs, select_text)?
    .into_int_value();
```

llvmkit:

```rust
let result = builder.build_select(condition, rhs, lhs, select_text)?;
```

- [ ] **Step 6: Replace casts**

Current Inkwell:

```rust
let extended = builder.build_int_z_extend(value, self.context.i64_type(), "")?;
let truncated = builder.build_int_truncate(value, self.context.i8_type(), "")?;
```

llvmkit dynamic-width form:

```rust
let extended = builder.build_zext_dyn(value, self.module.i64_type().as_dyn(), "")?;
let truncated = builder.build_trunc_dyn(value, self.module.i8_type().as_dyn(), "")?;
```

- [ ] **Step 7: Run per-module check loop**

After each semantics file:

```bash
cargo check -p bin_lift --lib
```

Expected: the number of Inkwell import and builder type errors decreases monotonically. Do not silence errors with `unwrap` or runtime module checks.

---

### Task 9: Rewrite `ret` without post-terminator surgery

**Files:**
- Modify: `src/lifter/semantics/x86/ret.rs:55-122`

- [ ] **Step 1: Replace imports**

```rust
use super::{LifterX86, Result};
use llvmkit::ir::{IntDyn, IntValue, ModuleBrand};
use zydis::{ffi::DecodedOperandKind, Instruction, Operands, Register};
```

- [ ] **Step 2: Replace `lift_ret` control flow**

Use this shape instead of emitting and erasing returns:

```rust
pub(super) fn lift_ret<O>(&mut self, instr: &Instruction<O>) -> Result<()>
where
    O: Operands,
{
    let ops = instr.operands();
    let rsp_value: IntValue<'ctx, IntDyn, B> = self.get_register(Register::SP)?.try_into()?;

    let rsp_addr = if let DecodedOperandKind::Imm(_) = &ops[0].kind {
        &ops[3]
    } else {
        &ops[2]
    };

    let real_val: IntValue<'ctx, IntDyn, B> = self.load_single_op(rsp_addr, rsp_addr.size)?.try_into()?;

    let rop_result = if rsp_value.get_sign_extended_constant().is_some() {
        RopResult::RealReturn
    } else {
        RopResult::RopReturn
    };

    if rop_result == RopResult::RealReturn {
        let rax: IntValue<'ctx, IntDyn, B> = self.get_register(Register::AX)?.try_into()?;
        let rax = self.create_z_ext_or_trunc(rax, self.module.i64_type().as_dyn())?;
        self.builder.build_ret(rax)?;
        return Ok(());
    }

    let arch_step = self
        .module
        .i64_type()
        .const_int(u64::from(self.retdec_get_arch_byte_size() / 8));
    let mut rsp_result = self.builder.build_int_add(rsp_value, arch_step.as_dyn(), "ret_new_rsp_")?;

    if let DecodedOperandKind::Imm(immediate) = &ops[0].kind {
        let immediate_ty = rsp_result.ty();
        let immediate_value = immediate_ty.const_int_raw(immediate.value, immediate.is_signed);
        rsp_result = self.builder.build_int_add(rsp_result, immediate_value, "")?;
    }

    self.store_reg(Register::SP, rsp_result)?;
    let _ = real_val;
    Ok(())
}
```

- [ ] **Step 3: Decide how the main lift loop stops after real return**

In `src/compiler/mod.rs::lift_function`, after `lift_instr` returns from a real return, stop lifting more instructions by making `lift_ret` return a distinct error or result enum:

```rust
pub(crate) enum LiftControl {
    Continue,
    FunctionTerminated,
}
```

Then change semantics trait methods from `Result<()>` to `Result<LiftControl>` only if continuing after a terminator causes builder-position errors. Prefer this explicit control enum over erased-instruction surgery.

- [ ] **Step 4: Check return semantics**

Run:

```bash
cargo check -p bin_lift --lib
```

Expected: no use of `erase_from_basic_block`, `set_name` on basic blocks, or `get_insert_block` remains in `ret.rs`.

---

### Task 10: Replace final return and remove optimization path

**Files:**
- Modify: `src/compiler/mod.rs:88-193`

- [ ] **Step 1: Replace `lift_function` signature**

```rust
pub fn lift_function(&mut self, instructions: &[FullInstruction]) -> Result<()> {
```

- [ ] **Step 2: Replace loop receiver**

```rust
for instruction in instructions {
    match self.lifter.lift_instr(instruction) {
        Ok(_) => {}
        Err(e) => match e {
            crate::lifter::Error::UnsupportedInstr(_) => {}
            _ => return Err(Error::Message(e.to_string())),
        },
    }
}
```

- [ ] **Step 3: Replace final return emission**

```rust
let rax = Register::AX.largest_enclosing(self.mode);
if let Ok(rax_val) = self.lifter.load_register_value(&rax) {
    let rax_as_int: IntValue<'ctx, IntDyn, B> = rax_val.try_into()?;
    let expected_retval_type = self.func_value.signature().return_type().try_into()?;
    let rax_with_correct_size = self.lifter.create_z_ext_or_trunc(rax_as_int, expected_retval_type)?;
    self.lifter.builder.build_ret(rax_with_correct_size)?;
}

Ok(())
```

- [ ] **Step 4: Remove `optimize_results` from the core API**

Delete the `optimize_results: bool` parameter from `lift_function`. Keep optimization out of this IR migration. The companion backend plan restores an optional post-processing path using external tooling or future llvmkit passes.

- [ ] **Step 5: Check compiler module**

Run:

```bash
cargo check -p bin_lift --lib
```

Expected: no imports from `llvmkit::ir` target-machine or pass-builder APIs because llvmkit does not provide backend code generation.

---

### Task 11: Port examples and add a real IR-generation test

**Files:**
- Modify: `examples/simple_add.rs`
- Modify: `examples/new_lift_add.rs`
- Modify: `examples/lift_vmp_trace.rs`
- Create: `tests/llvmkit_ir_generation.rs`

- [ ] **Step 1: Use `Module::with_new` in every example**

Each example should follow this shape:

```rust
let ir = Module::with_new::<_, _, _>("protected", |module| {
    Compiler::lift_to_ir_text(module, &all_instructions, mode, runtime_address)
})?;

println!("{ir}");
```

- [ ] **Step 2: Create integration test**

Create `tests/llvmkit_ir_generation.rs`:

```rust
use llvmkit::ir::Module;
use zydis::Decoder;
use zydis2llvmir::compiler::Compiler;

const ADD_BYTES: [u8; 3] = [0x01, 0xD8, 0xC3];

#[test]
fn lifts_add_ret_to_verified_ir() -> Result<(), Box<dyn std::error::Error>> {
    let decoder = Decoder::new64();
    let mut instructions = Vec::new();
    for item in decoder.decode_all(&ADD_BYTES, 0) {
        let (_ip, _raw, instruction) = item?;
        instructions.push(instruction);
    }

    let ir = Module::with_new::<_, _, _>("protected", |module| {
        Compiler::lift_to_ir_text(module, &instructions, zydis::MachineMode::LONG_64, None)
    })?;

    assert!(ir.contains("define"));
    assert!(ir.contains("@protected"));
    assert!(ir.contains("add"));
    assert!(ir.contains("ret"));
    Ok(())
}
```

- [ ] **Step 3: Run the integration test**

```bash
cargo test --test llvmkit_ir_generation
```

Expected: PASS and generated text contains `define`, `@protected`, `add`, and `ret`.

---

### Task 12: Final cleanup and verification

**Files:**
- Modify: `Readme.md`
- Modify: any source file still importing Inkwell.

- [ ] **Step 1: Update README dependency wording**

Replace the dependency section with:

```markdown
# Dependencies

For verified LLVM IR text generation:

- Rust stable
- `llvmkit` from the sibling `rllvm` workspace
- `zydis` for instruction decoding

Native optimization and object emission are outside the llvmkit migration target. Use the backend bridge plan if native output is required.
```

- [ ] **Step 2: Search for Inkwell leftovers**

Run:

```bash
cargo check
```

Then use repository search for:

```text
inkwell::|Context::create|create_module|create_builder|print_to_stderr|print_to_file|run_passes|PassBuilderOptions|TargetMachine
```

Expected: no source/example matches except in the backend-gap plan documentation.

- [ ] **Step 3: Run final gates**

```bash
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --example simple_add
```

Expected:
- format check passes;
- clippy passes without `#[allow]` attributes;
- tests pass;
- `simple_add` prints verified LLVM IR text for `@protected`.

---

## Execution Notes

- Keep all lifting semantics inside `Module::with_new`; do not return branded handles outside the closure.
- Do not add runtime module-id checks to simulate brand safety.
- Do not migrate to `module.core()` even if the current llvmkit branch still exposes it.
- Treat every dynamic register width as `IntDyn` until the lifter can statically split 8/16/32/64-bit register paths.
- Keep backend optimization out of this plan; it is covered by `2026-06-19-bin-lift-llvmkit-backend-gap.md`.
