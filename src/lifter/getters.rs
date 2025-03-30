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
        let possible_size_in_llvm_ty = self.context.custom_width_int_type(possible_size.into());
        match operand.kind {
            DecodedOperandKind::Reg(register) => {
                let reg_val = self.load_register_value(&register)?;
                if let PossibleLLVMValueEnum::IntValue(value) = reg_val {
                    if value.get_type().get_bit_width() < 128 {
                        let zext_or_trunc_ed =
                            self.create_z_ext_or_trunc(value, possible_size_in_llvm_ty)?;
                        return Ok(zext_or_trunc_ed.into());
                    }
                }
                Ok(reg_val)
            }
            DecodedOperandKind::Mem(ref memory_info) => {
                let mem_val = self.load_mem(memory_info, possible_size, false)?;
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

    pub(super) fn load_two_first_ops(
        &self,
        operands: &[DecodedOperand],
    ) -> Result<[PossibleLLVMValueEnum<'ctx>; 2]> {
        let first_op: IntValue<'_> = self
            .load_single_op(&operands[0], operands[0].size)?
            .try_into()?;
        let second_op: IntValue<'_> = self
            .load_single_op(&operands[1], operands[0].size)?
            .try_into()?;

        let second_op_zext = self.create_z_ext_or_trunc(second_op, first_op.get_type())?;
        Ok([first_op.into(), second_op_zext.into()])
    }

    pub(super) fn load_two_first_ints(
        &self,
        ops: &[DecodedOperand],
    ) -> Result<[IntValue<'ctx>; 2]> {
        let first_op: IntValue<'_> = self.load_single_op(&ops[0], ops[0].size)?.try_into()?;
        let second_op: IntValue<'_> = self.load_single_op(&ops[1], ops[0].size)?.try_into()?;

        let second_op_zext = self.create_z_ext_or_trunc(second_op, first_op.get_type())?;
        Ok([first_op, second_op_zext])
    }

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

    fn load_mem(
        &self,
        mem: &MemoryInfo,
        first_operand_size: u16,
        lea: bool,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;

        let mem_op_addr = self.retdec_calc_mem_operand(mem)?;

        if lea {
            Ok(mem_op_addr)
        } else {
            let t = self
                .context
                .custom_width_int_type(first_operand_size.into());
            //let pt = self.context.ptr_type(AddressSpace::default());
            //let addr2 = builder.build_int_to_ptr(mem_op_addr, pt, "")?;
            //let load = builder.build_load(t, addr2, "experimental_load_mem_")?;

            //let pointee = builder.build_alloca(t, "pointee_for_gep")?;

            //let gepped = unsafe {
            //    builder.build_gep(
            //        pointee.get_type(),
            //        self.stackmemory,
            //        &[mem_op_addr],
            //        "gep_loaded",
            //    )?
            //};
            //dbg!(gepped);

            //let loaded = builder.build_load(t, gepped, "loaded_from_gep")?;

            let pointer = unsafe {
                builder.build_gep(
                    self.context.i8_type(),
                    self.stackmemory,
                    &[mem_op_addr],
                    "GEPSTORE",
                )?
            };

            let retval = builder.build_load(t, pointer, "Loadxd")?.into_int_value();

            Ok(retval)
            //Ok(loaded.into_int_value())

            //Ok(load.into_int_value())
            //todo!()
        }
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
            let ip = self
                .get_max_int_type()
                // NOTE: Ip tracking isn't implemented yet
                .const_int(self.runtime_address.get(), false);
            return Ok(ip.into());
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
