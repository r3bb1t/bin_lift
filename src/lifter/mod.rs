pub(super) mod error;
pub(crate) use error::{Error, Result};

use crate::miscellaneous::ExtendedRegisterEnum;
use std::{
    cell::{Cell, UnsafeCell},
    collections::HashMap,
};

use definintions::{PossibleLLVMTypeEnum, PossibleLLVMValueEnum};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::IntType,
    values::{FunctionValue, IntValue, PointerValue},
};
use zydis::{ffi::MemoryInfo, MachineMode, Register, RegisterClass};

mod common;
mod getters;
mod setters;

mod mergen_getters_and_setters;

mod flagops;
pub(crate) mod semantics;

mod definintions;

/// This is the reworked lifter. Responsible only for lifting the code
pub struct LifterX86<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    pub mode: MachineMode,
    //pub(super) regs_hashmap:
    //    RefCell<HashMap<ExtendedRegister, PossibleLLVMValueEnum<'ctx>>>,
    pub(super) regs_hashmap: UnsafeCell<HashMap<ExtendedRegisterEnum, PossibleLLVMValueEnum<'ctx>>>,
    pub stackmemory: PointerValue<'ctx>,
    pub runtime_address: Option<Cell<u64>>,
}

impl<'ctx> LifterX86<'ctx> {
    pub fn new(
        context: &'ctx Context,
        mode: MachineMode,
        func_value: FunctionValue<'ctx>,
        module: Module<'ctx>,
        runtime_address: Option<u64>,
    ) -> Result<Self> {
        let builder = context.create_builder();

        let regs_hashmap = prep_regs_hashmap_experimental(&func_value, &mode);

        let entry_basic_block = context.append_basic_block(func_value, "entry");
        builder.position_at_end(entry_basic_block);

        const STACK_SIZE: u64 = 0x1000;
        let stackmemory = builder.build_array_alloca(
            context.i128_type(),
            context.i128_type().const_int(STACK_SIZE, false),
            "stackmemory",
        )?;

        // TODO: Consider assigning SP and BP to half of the stack size

        builder.position_at_end(entry_basic_block);

        let s = Self {
            context,
            builder,
            module,
            mode,
            //regs_hashmap: RefCell::new(regs_hashmap),
            regs_hashmap: UnsafeCell::new(regs_hashmap),
            //func_value,
            stackmemory,
            runtime_address: runtime_address.map(Cell::new),
        };
        Ok(s)
    }

    pub(crate) fn runtime_address(&self) -> Option<u64> {
        self.runtime_address
            .as_ref()
            .map(|addr_cell| addr_cell.get())
    }

    pub(crate) fn increase_ip(&self, instr_length: u8) {
        let Some(current_ip_cell) = &self.runtime_address else {
            return;
        };

        let current_ip = current_ip_cell.get();
        let updated_ip = current_ip + instr_length as u64;

        current_ip_cell.set(updated_ip);
    }

    pub(super) fn get_max_int_type(&self) -> IntType<'ctx> {
        let example_reg = Register::AX.largest_enclosing(self.mode); // random rax for convenience
        self.context
            .custom_width_int_type(example_reg.width(self.mode).into())
    }

    pub(crate) fn get_register_type(&self, reg: Register) -> PossibleLLVMTypeEnum<'ctx> {
        let ctx = self.context;
        match reg.class() {
            RegisterClass::GPR8 => ctx.i8_type().into(),
            RegisterClass::GPR16 => ctx.i16_type().into(),
            RegisterClass::GPR32 => ctx.i32_type().into(),
            RegisterClass::GPR64 => ctx.i64_type().into(),
            RegisterClass::MMX => ctx.f64_type().into(),
            RegisterClass::XMM => ctx.i128_type().into(),
            RegisterClass::YMM => panic!("LLVM Doesn't support 256 bit floats"),
            RegisterClass::ZMM => panic!("LLVM Doesn't support 512 bit floats"),
            //RegisterClass::FLAGS => util::get_int_ty(ctx, reg.width(self.mode).into()).into(),
            //RegisterClass::IP => util::get_int_ty(ctx, reg.width(self.mode).into()).into(),
            RegisterClass::IP | RegisterClass::FLAGS => ctx
                .custom_width_int_type(reg.width(self.mode).into())
                .into(),
            RegisterClass::SEGMENT => ctx.i16_type().into(),
            RegisterClass::INVALID => panic!("Invalid register"),
            //_ => util::get_int_ty(ctx, reg.width(self.mode).into()).into(),
            _ => ctx
                .custom_width_int_type(reg.width(self.mode).into())
                .into(),
        }
    }

    pub(crate) fn create_z_ext_or_trunc(
        &self,
        value: IntValue<'ctx>,
        dest: IntType<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let vty = value.get_type();
        let builder = &self.builder;

        if vty.get_bit_width() < dest.get_bit_width() {
            Ok(builder.build_int_z_extend(value, dest, "")?)
        } else {
            Ok(builder.build_int_truncate(value, dest, "")?)
        }
    }

    pub fn retdec_get_default_type(&self) -> IntType<'ctx> {
        //self.retdec_get_integer_type_from_byte_size(self.retdec_get_arch_byte_size().into())
        let arch_byte_size = self.retdec_get_arch_byte_size();
        self.context.custom_width_int_type(arch_byte_size.into())
    }

    pub fn retdec_get_arch_byte_size(&self) -> u8 {
        match self.mode {
            MachineMode::LONG_64 => 8,
            MachineMode::LONG_COMPAT_32 | MachineMode::LEGACY_32 => 4,
            MachineMode::LONG_COMPAT_16 | MachineMode::LEGACY_16 | MachineMode::REAL_16 => 2,
        }
    }
}

// After the function was created, use this one to store everything in a hashmap
fn prep_regs_hashmap_experimental<'ctx>(
    fn_val: &FunctionValue<'ctx>,
    mode: &MachineMode,
) -> HashMap<ExtendedRegisterEnum, PossibleLLVMValueEnum<'ctx>> {
    let mut registers_hashmap = HashMap::new();
    let regs: [Register; 17] =
        crate::compiler::ALL_REGS_IN_MIN_SIZE.map(|reg| reg.largest_enclosing(*mode));

    for (id, reg) in regs.into_iter().enumerate() {
        registers_hashmap.insert(reg.into(), fn_val.get_nth_param(id as u32).unwrap());
    }
    // Also insert flags separately
    let mut last_index = regs.len() - 1;
    for cpu_flag in crate::compiler::CPU_FLAGS {
        last_index += 1;
        registers_hashmap.insert(cpu_flag, fn_val.get_nth_param(last_index as u32).unwrap());
    }

    registers_hashmap
        .into_iter()
        .map(|(reg, val)| (reg, val.try_into().unwrap()))
        .collect()
}
