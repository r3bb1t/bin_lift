mod getters;
mod retdec_common;
mod retdec_getters;
mod retdec_setters;
mod setters;

pub mod semantics;

use crate::compiler::{ALL_REGS_IN_MIN_SIZE, CPU_FLAGS};
use crate::miscellaneous::ExtendedRegister;
use crate::util::get_int_type;
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, IntType};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, IntValue};
use semantics::Lifter;
use std::cell::RefCell;
use std::collections::HashMap;
use zydis::ffi::DecodedOperandKind;
use zydis::{FullInstruction, MachineMode, Register};

// Stolen from retdec code
/**
 * What should instruction operand loading method do if types of
 * loaded operands are not the same.
 *
*/
#[allow(non_camel_case_types, unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum eOpConv {
    /// Throw exception.
    Throw,
    /// Operand types does not have to be equal.
    Nothing,
    /// Convert to destination integer type using ZEXT or TRUNC.
    /// If source is FP type converts it using bitcast.
    ZEXT_TRUNC_OR_BITCAST,
    /// Convert to destination integer type using SEXT or TRUNC.
    /// If source is FP type converts it using bitcast.
    SEXT_TRUNC_OR_BITCAST,
    /// Convert to destination FP type using FPCast (FPExt, BitCast,
    /// or FPTrunc).
    /// If source is integer type converts it using bitcast.
    FPCAST_OR_BITCAST,
    /// Convert to destination FP type using SIToFP.
    /// Source must be integer, destination fp, or LLVM asserts.
    SITOFP_OR_FPCAST,
    /// Convert to destination FP type using UIToFP.
    /// Source must be integer, destination fp, or LLVM asserts.
    UITOFP_OR_FPCAST,
}

// TODO: Consider moving it to x86 folder and extracting important methods
// into trait
pub struct LifterX86<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    pub mode: MachineMode,
    pub regs_hashmap: RefCell<HashMap<ExtendedRegister, BasicValueEnum<'ctx>>>,
    pub func_value: FunctionValue<'ctx>,
}

impl<'ctx> LifterX86<'ctx> {
    pub fn new(context: &'ctx Context, mode: MachineMode) -> Self {
        let builder = context.create_builder();
        let module = create_module(context, &mode);
        let func_value = create_func(&mode, context, &module);

        let regs_hashmap = prep_regs_hashmap(&func_value, &mode);
        Self {
            context,
            builder,
            module,
            mode,
            regs_hashmap: RefCell::new(regs_hashmap),
            func_value,
        }
    }

    pub fn lift_basic_block(
        &self,
        instructions: &Vec<FullInstruction>,
    ) -> Result<&FunctionValue, BuilderError> {
        let entry_basic_block = self.context.append_basic_block(self.func_value, "entry");
        self.builder.position_at_end(entry_basic_block);
        for ins in instructions {
            self.lift_instr(ins.clone())?;
            if ins.mnemonic == zydis::Mnemonic::ADD {
                let ops = ins.operands();

                let first_op_matched = if let DecodedOperandKind::Mem(mem) = &ops[0].kind {
                    mem.base == zydis::Register::RAX || mem.index == zydis::Register::RAX
                } else {
                    false
                };

                let second_matched =
                    matches!(ops[1].kind, DecodedOperandKind::Reg(zydis::Register::AL));
                if second_matched && first_op_matched {
                    break;
                }
            }
        }

        Ok(&self.func_value)
    }

    fn get_max_int_size(&self) -> IntType {
        let example_reg = Register::AX.largest_enclosing(self.mode); // random rax for convenience
        let int_type = get_int_type(self.context, &example_reg, &self.mode);
        int_type
    }

    fn prep_main_func(&self) {
        let fn_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", fn_type, None);

        let entry_bb = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry_bb);

        let regs_hashmap = prep_regs_hashmap(&main_fn, &self.mode);
    }
    fn wrap_function_call(&self) -> Result<(), BuilderError> {
        let example_reg = Register::AX.largest_enclosing(self.mode); // random rax for convenience
        let int_type = get_int_type(self.context, &example_reg, &self.mode);

        const ARGS_COUNT: usize = ALL_REGS_IN_MIN_SIZE.len() + CPU_FLAGS.len();
        let regs_args: [_; ALL_REGS_IN_MIN_SIZE.len()] =
            core::array::from_fn(|_| int_type.const_zero());

        let flags_args: [_; CPU_FLAGS.len()] =
            core::array::from_fn(|_| self.context.i8_type().const_zero());

        let mut args = Vec::with_capacity(ARGS_COUNT);
        args.extend_from_slice(&regs_args);
        args.extend_from_slice(&flags_args);

        let fn_params: Vec<BasicMetadataValueEnum<'ctx>> = self
            .func_value
            .get_params()
            .iter()
            .map(|val| BasicMetadataValueEnum::IntValue(val.into_int_value()))
            .collect();

        self.builder.build_call(self.func_value, &fn_params, "")?;

        Ok(())
    }

    //  region:     -- Helpers
    pub(crate) fn create_z_ext_or_trunc(
        &self,
        value: IntValue<'ctx>,
        dest: IntType<'ctx>,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        let vty = value.get_type();
        let builder = &self.builder;

        if vty.get_bit_width() < dest.get_bit_width() {
            Ok(builder.build_int_z_extend(value, dest, "")?)
        } else {
            Ok(builder.build_int_truncate(value, dest, "")?)
        }
    }
    pub(crate) fn create_s_ext_or_trunc(
        &self,
        value: IntValue<'ctx>,
        dest: IntType<'ctx>,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        let vty = value.get_type();
        let builder = &self.builder;

        if vty.get_bit_width() < dest.get_bit_width() {
            Ok(builder.build_int_s_extend(value, dest, "")?)
        } else {
            Ok(builder.build_int_truncate(value, dest, "")?)
        }
    }
    // TODO: Think if we really need register as argument
    pub(crate) fn get_int_type(&self, register: &Register) -> IntType<'ctx> {
        get_int_type(self.context, register, &self.mode)
    }

    /// Sets some debug value for easier understanding of what's going on
    #[cfg(debug_assertions)]
    pub(crate) fn set_nop(&self, mnemonic: &zydis::Mnemonic) -> Result<(), BuilderError> {
        let i128_ty = self.context.i128_type();
        self.builder
            .build_alloca(i128_ty, &format!("debug_{}_", mnemonic))?;
        Ok(())
    }
    //  endregion:     -- Helpers
}

fn create_func<'ctx>(
    mode: &MachineMode,
    context: &'ctx Context,
    module: &Module<'ctx>,
) -> FunctionValue<'ctx> {
    let example_reg = Register::AX.largest_enclosing(*mode); // random rax for convenience
    let int_type = get_int_type(context, &example_reg, mode);

    const ARGS_COUNT: usize = ALL_REGS_IN_MIN_SIZE.len() + CPU_FLAGS.len();
    let regs_args: [BasicMetadataTypeEnum; ALL_REGS_IN_MIN_SIZE.len()] =
        core::array::from_fn(|_| int_type.into());

    let flags_args: [BasicMetadataTypeEnum; CPU_FLAGS.len()] =
        core::array::from_fn(|_| context.i8_type().into());

    let mut args = Vec::with_capacity(ARGS_COUNT);
    args.extend_from_slice(&regs_args);
    args.extend_from_slice(&flags_args);

    let fn_type = int_type.fn_type(&args, false);
    let fn_val = module.add_function("protected", fn_type, None);

    /// Inner function for converting register names
    fn get_reg_name_for_mode(reg: Register, mode: MachineMode) -> &'static str {
        reg.largest_enclosing(mode).static_string().unwrap()
    }

    // Set names for regular regs
    for (id, reg) in ALL_REGS_IN_MIN_SIZE.into_iter().enumerate() {
        fn_val
            .get_nth_param(id as u32)
            .unwrap()
            .set_name(get_reg_name_for_mode(reg, *mode));
    }

    // Set names for CPU flags
    for (id, cpu_flag) in CPU_FLAGS.into_iter().enumerate() {
        fn_val
            .get_nth_param((ALL_REGS_IN_MIN_SIZE.len() + id) as u32)
            .unwrap()
            .set_name(&format!("{cpu_flag:?}"));
    }

    fn_val
}
/// After the function was created, use this one to store everything in a hashmap
fn prep_regs_hashmap<'ctx>(
    fn_val: &FunctionValue<'ctx>,
    mode: &MachineMode,
) -> HashMap<ExtendedRegister, BasicValueEnum<'ctx>> {
    let mut registers_hashmap = HashMap::new();
    let regs: [Register; 17] = ALL_REGS_IN_MIN_SIZE.map(|reg| reg.largest_enclosing(*mode));

    for (id, reg) in regs.into_iter().enumerate() {
        registers_hashmap.insert(reg.into(), fn_val.get_nth_param(id as u32).unwrap());
    }
    // Also insert flags separately
    let mut last_index = regs.len() - 1;
    for cpu_flag in CPU_FLAGS {
        last_index += 1;
        registers_hashmap.insert(cpu_flag, fn_val.get_nth_param(last_index as u32).unwrap());
    }

    registers_hashmap
}

fn create_module<'ctx>(cx: &'ctx Context, mode: &MachineMode) -> Module<'ctx> {
    let module_name = "protected";
    let module = cx.create_module(module_name);
    module
}
