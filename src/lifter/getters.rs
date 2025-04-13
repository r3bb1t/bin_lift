use super::{error::Error, error::Result, ExtendedRegisterEnum};
use super::{LifterX86, PossibleLLVMValueEnum};

use std::borrow::Borrow;

use inkwell::values::IntValue;
use zydis::{
    ffi::{DecodedOperand, DecodedOperandKind, ImmediateInfo, MemoryInfo},
    Register, RegisterClass,
};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn load_single_int_op(
        &self,
        operand: &DecodedOperand,
        possible_size: u16,
    ) -> Result<IntValue<'ctx>> {
        self.load_single_op(operand, possible_size)?.try_into()
    }

    // NOTE: Must be used only with instructions having exactly one operand if called from
    // semantics
    pub(super) fn load_single_op(
        &self,
        operand: &DecodedOperand,
        possible_size: u16,
    ) -> Result<PossibleLLVMValueEnum<'ctx>> {
        //let possible_size_in_llvm_ty = self.context.custom_width_int_type(possible_size.into());
        match operand.kind {
            DecodedOperandKind::Reg(register) => {
                let reg_val = self.mergen_get_register(&register, possible_size.into())?;
                Ok(reg_val)
            }
            DecodedOperandKind::Mem(ref memory_info) => {
                let mem_val = self.mergen_load_mem(memory_info, possible_size.into())?;
                Ok(mem_val.into())
            }
            DecodedOperandKind::Imm(ref imm) => {
                //let imm_val = self.experimental_load_imm_internal(imm, possible_size);
                let imm_val = self.load_imm_internal(imm, possible_size);
                Ok(imm_val.into())
            }
            DecodedOperandKind::Unused | DecodedOperandKind::Ptr(_) => unreachable!(),
        }
    }

    // For now these 2 funcs would be commented

    //pub(super) fn load_two_first_ops(
    //    &self,
    //    operands: &[DecodedOperand],
    //) -> Result<[PossibleLLVMValueEnum<'ctx>; 2]> {
    //    let first_op: IntValue<'_> = self
    //        .load_single_op(&operands[0], operands[0].size)?
    //        .try_into()?;
    //    let second_op: IntValue<'_> = self
    //        .load_single_op(&operands[1], operands[0].size)?
    //        .try_into()?;
    //
    //    let second_op_zext = self.create_z_ext_or_trunc(second_op, first_op.get_type())?;
    //    Ok([first_op.into(), second_op_zext.into()])
    //}
    //
    //pub(super) fn load_two_first_ints(
    //    &self,
    //    ops: &[DecodedOperand],
    //) -> Result<[IntValue<'ctx>; 2]> {
    //    let first_op: IntValue<'_> = self.load_single_op(&ops[0], ops[0].size)?.try_into()?;
    //    let second_op: IntValue<'_> = self.load_single_op(&ops[1], ops[0].size)?.try_into()?;
    //
    //    let second_op_zext = self.create_z_ext_or_trunc(second_op, first_op.get_type())?;
    //    Ok([first_op, second_op_zext])
    //}

    pub(super) fn load_flag<T: Borrow<ExtendedRegisterEnum>>(
        &self,
        cpu_flag: T,
    ) -> Result<IntValue<'ctx>> {
        let cpu_flag = cpu_flag.borrow();
        let regs_hashmap = self.regs_hashmap();
        let lookup_result = regs_hashmap.get(cpu_flag).copied();

        let reg_val = match lookup_result {
            Some(val) => val.try_into()?,
            None => self.get_max_int_type().const_zero(),
        };

        Ok(reg_val)
    }

    // New implementation (from Mergen)
    pub(crate) fn load_register_value(
        &self,
        register: &Register,
    ) -> Result<PossibleLLVMValueEnum<'ctx>> {
        let register_class = register.class();

        if register == &Register::NONE {
            return Err(Error::RegUnwrapError(ExtendedRegisterEnum::NONE));
        }

        if register_class == RegisterClass::IP {
            // TODO: Just a random number tbh. Need to add runtime address
            let int_ty = self.get_max_int_type();
            let int_value = if let Some(runtime_address) = self.runtime_address() {
                int_ty.const_int(runtime_address, false)
            } else {
                int_ty.get_undef()
            };
            return Ok(int_value.into());
        } else if register_class == RegisterClass::FLAGS {
            let rflags = self.get_rflags_value()?;
            return Ok(rflags.into());
        }

        if [Register::AH, Register::CH, Register::DH, Register::BH].contains(register) {
            let val = self.get_value_from_high_byte_register(register)?;
            return Ok(val.into());
        }

        let new_key = self.get_register_largest_enclosing(register);
        debug_assert!(new_key != Register::NONE);

        self.get_register(new_key)
    }

    fn get_value_from_high_byte_register(&self, register: &Register) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let largest_register_enclosing = self.get_register_largest_enclosing(register);

        let full_register_value: IntValue<'_> = self
            .load_register_value(&largest_register_enclosing)?
            .try_into()?;
        let full_register_value_ty = full_register_value.get_type();

        let shifted_value = builder.build_right_shift(
            full_register_value,
            full_register_value_ty.const_int(8, false),
            false,
            "",
        )?;
        let ff = full_register_value_ty.const_int(0xFF, false);
        let high_byte_value = builder.build_and(shifted_value, ff, "")?;

        Ok(high_byte_value)
    }

    fn load_imm_internal(&self, imm: &ImmediateInfo, possible_size: u16) -> IntValue<'ctx> {
        self.context
            .custom_width_int_type(possible_size.into())
            .const_int(imm.value, imm.is_signed)
    }
}
