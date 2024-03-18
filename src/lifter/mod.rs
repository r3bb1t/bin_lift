mod getters;
mod setters;

use crate::util::get_int_type;
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::IntType;
use inkwell::values::{BasicValueEnum, IntValue};
use std::cell::RefCell;
use std::collections::HashMap;
use zydis::ffi::MemoryInfo;
use zydis::{MachineMode, Register};

// Stolen from retdec code
/**
 * What should instruction operand loading method do if types of
 * loaded operands are not the same.
 *
*/
#[allow(non_camel_case_types, unused)]
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

pub struct LifterX86<'a, 'b, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub mode: &'a MachineMode,
    pub regs_hashmap: RefCell<HashMap<Register, BasicValueEnum<'b>>>,
}

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    /// Warning: Panics if got an invalid operand
    /// Calculates address of memory about to be written/read
    pub fn calc_mem_operand(
        &'a self,
        mem_info: &MemoryInfo,
    ) -> Result<BasicValueEnum, BuilderError> {
        // panic if got an invalid input. TODO: replace panicking with custom error
        assert!(
            !(mem_info.index == Register::NONE
                && mem_info.base == Register::NONE
                && mem_info.disp.displacement == 0),
            "Received an invalid memory operand {mem_info:?}"
        );

        let builder = self.builder;
        // Or do we need sign extend?
        let mut addr = self.context.i64_type().const_int(0, false);

        // If we have base, then add it to result
        if mem_info.base != Register::NONE {
            let base = self
                .load_reg(&mem_info.base.largest_enclosing(*self.mode))?
                .into_int_value();
            // 0 + base
            addr = builder.build_int_add(addr, base, "base_")?;

            if mem_info.disp.has_displacement {
                let displacement = self
                    .context
                    .i64_type()
                    .const_int(mem_info.disp.displacement as u64, true);

                let displacement_ext_or_trunc =
                    self.create_s_ext_or_trunc(displacement, base.get_type())?;
                addr = builder.build_int_add(displacement_ext_or_trunc, addr, "")?;
            }
        }

        // If we have index, then add it
        if mem_info.index != Register::NONE {
            let index = self.load_reg(&mem_info.index)?.into_int_value();
            // If we have scale, then multiply and add index * scale
            if mem_info.scale != 0 {
                let scale = self
                    .context
                    .i8_type()
                    .const_int(mem_info.scale as u64, false);
                let multiplied = builder.build_int_mul(index, scale, "scale_multiplied")?;
                addr = builder.build_int_add(addr, multiplied, "")?;
            }
            // Otherwise add just index
            addr = builder.build_int_add(addr, index, "")?;
        }

        // Not expecting this to be possible tbh
        if mem_info.index == Register::NONE
            && mem_info.base == Register::NONE
            && mem_info.disp.has_displacement
        {
            addr = self
                .context
                .i64_type()
                .const_int(mem_info.disp.displacement as u64, true);
        }

        Ok(BasicValueEnum::from(addr))
    }

    //  region:     -- Helpers
    pub(in crate::lifter) fn create_s_ext_or_trunc(
        &self,
        value: IntValue<'a>,
        dest: IntType<'a>,
    ) -> Result<IntValue<'a>, BuilderError> {
        let vty = value.get_type();
        let builder = self.builder;

        if vty.get_bit_width() < dest.get_bit_width() {
            Ok(builder.build_int_s_extend(value, dest, "")?)
        } else {
            Ok(builder.build_int_truncate(value, dest, "")?)
        }
    }
    pub fn get_int_type(&self, register: &Register) -> IntType<'ctx> {
        get_int_type(self.context, &register, self.mode)
    }
    //  endregion:     -- Helpers
}
