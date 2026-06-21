pub mod error;
pub use error::{Error, Result};

use crate::compiler::{ALL_REGS_IN_MIN_SIZE, CPU_FLAGS};
use crate::miscellaneous::ExtendedRegisterEnum;
use std::{cell::Cell, collections::HashMap};

pub(crate) use definintions::LiftValue;
use llvmkit::ir::{
    Brand, ConstantFolder, FunctionValue, IRBuilder, IntDyn, IntValue, Module, PointerValue,
    Positioned, Unverified,
};
use zydis::{MachineMode, Register};

mod common;
mod definintions;
mod flagops;
mod getters;
mod mergen_getters_and_setters;
pub(crate) mod semantics;
mod setters;

type LifterBuilder<'m, 'ctx> = IRBuilder<'m, 'ctx, Brand<'ctx>, ConstantFolder, Positioned, IntDyn>;

pub struct LifterX86<'m, 'ctx> {
    pub module: &'m Module<'ctx, Brand<'ctx>, Unverified>,
    pub builder: Option<LifterBuilder<'m, 'ctx>>,
    pub mode: MachineMode,
    pub(super) regs_hashmap: HashMap<ExtendedRegisterEnum, LiftValue<'ctx>>,
    pub stackmemory: PointerValue<'ctx>,
    pub runtime_address: Option<Cell<u64>>,
}

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub fn new(
        module: &'m Module<'ctx, Brand<'ctx>, Unverified>,
        mode: MachineMode,
        func_value: FunctionValue<'ctx, IntDyn>,
        runtime_address: Option<u64>,
    ) -> Result<Self> {
        let regs_hashmap = prep_regs_hashmap_experimental(func_value, mode)?;
        let entry_basic_block = func_value.append_basic_block(module, "entry");
        let builder = IRBuilder::new_for::<IntDyn>(module).position_at_end(entry_basic_block);

        let stackmemory = builder.build_array_alloca(
            module.i128_type(),
            module.i128_type().as_dyn().const_int_raw(0x1000, false)?,
            "stackmemory",
        )?;

        Ok(Self {
            module,
            builder: Some(builder),
            mode,
            regs_hashmap,
            stackmemory,
            runtime_address: runtime_address.map(Cell::new),
        })
    }
}

fn prep_regs_hashmap_experimental<'ctx>(
    fn_val: FunctionValue<'ctx, IntDyn>,
    mode: MachineMode,
) -> Result<HashMap<ExtendedRegisterEnum, LiftValue<'ctx>>> {
    let mut registers_hashmap = HashMap::new();
    let regs: [Register; 17] = ALL_REGS_IN_MIN_SIZE.map(|reg| reg.largest_enclosing(mode));

    for (id, reg) in regs.into_iter().enumerate() {
        let slot = u32::try_from(id).map_err(|_| llvmkit::ir::IrError::InvalidOperation {
            message: "register argument index exceeds u32::MAX",
        })?;
        let value: IntValue<'ctx, IntDyn> = fn_val.param(slot)?.try_into()?;
        registers_hashmap.insert(reg.into(), value.into());
    }

    for (id, cpu_flag) in CPU_FLAGS.into_iter().enumerate() {
        let raw_slot = regs.len() + id;
        let slot = u32::try_from(raw_slot).map_err(|_| llvmkit::ir::IrError::InvalidOperation {
            message: "flag argument index exceeds u32::MAX",
        })?;
        let value: IntValue<'ctx, IntDyn> = fn_val.param(slot)?.try_into()?;
        registers_hashmap.insert(cpu_flag, value.into());
    }

    Ok(registers_hashmap)
}
