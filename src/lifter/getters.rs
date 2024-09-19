use crate::lifter::LifterX86;
use crate::miscellaneous::ExtendedRegister;
use crate::util;
use inkwell::builder::BuilderError;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};
use inkwell::AddressSpace;
use zydis::ffi::{DecodedOperand, DecodedOperandKind, ImmediateInfo, MemoryInfo};
use zydis::Register;

impl<'ctx> LifterX86<'ctx> {
    /// Loads 2 operands and performs **zero** extension if they are not equal
    pub(super) fn load_2_operands(
        &self,
        operands: &[DecodedOperand],
    ) -> Result<[BasicValueEnum<'ctx>; 2], BuilderError> {
        let lhs_val = self.retdec_load_operand(&operands[0], &None, false)?;
        let rhs_val = self.retdec_load_operand(&operands[1], &None, false)?;

        if let (DecodedOperandKind::Reg(lhs_reg), DecodedOperandKind::Reg(rhs_reg)) =
            (&operands[0].kind, &operands[1].kind)
        {
            if rhs_reg.width(self.mode) > lhs_reg.width(self.mode) {
                let extended_reg_name = if cfg!(debug_assertions) {
                    &format!("extended_{rhs_reg:?}")
                } else {
                    ""
                };

                if let (BasicValueEnum::IntValue(lhs_int), BasicValueEnum::IntValue(rhs_int)) =
                    (lhs_val, rhs_val)
                {
                    let lhs_z_ext = self.builder.build_int_z_extend(
                        lhs_int,
                        rhs_int.get_type(),
                        extended_reg_name,
                    )?;

                    return Ok([lhs_z_ext.as_basic_value_enum(), rhs_val]);
                } else if let (BasicValueEnum::FloatValue(_), BasicValueEnum::FloatValue(_)) =
                    (lhs_val, rhs_val)
                {
                    todo!("Floating point numbers are not supported yet. Tried to load following operands: {operands:?}")
                } else {
                    unreachable!()
                }
            }
        }
        Ok([lhs_val, rhs_val])
    }

    /// Loads 2 operands and performs **sign** extension if they are not equal
    pub(super) fn load_2_operands_signed(
        &self,
        operands: &[DecodedOperand],
    ) -> Result<[BasicValueEnum<'ctx>; 2], BuilderError> {
        let lhs_val = self.retdec_load_operand(&operands[0], &None, false)?;
        let rhs_val = self.retdec_load_operand(&operands[1], &None, false)?;

        if let (DecodedOperandKind::Reg(lhs_reg), DecodedOperandKind::Reg(rhs_reg)) =
            (&operands[0].kind, &operands[1].kind)
        {
            if rhs_reg.width(self.mode) > lhs_reg.width(self.mode) {
                let extended_reg_name = if cfg!(debug_assertions) {
                    &format!("extended_{rhs_reg:?}")
                } else {
                    ""
                };

                if let (BasicValueEnum::IntValue(lhs_int), BasicValueEnum::IntValue(rhs_int)) =
                    (lhs_val, rhs_val)
                {
                    let lhs_z_ext = self.builder.build_int_s_extend(
                        lhs_int,
                        rhs_int.get_type(),
                        extended_reg_name,
                    )?;

                    return Ok([lhs_z_ext.as_basic_value_enum(), rhs_val]);
                } else if let (BasicValueEnum::FloatValue(_), BasicValueEnum::FloatValue(_)) =
                    (lhs_val, rhs_val)
                {
                    todo!("Floating point numbers are not supported yet. Tried to load following operands: {operands:?}")
                } else {
                    unreachable!()
                }
            }
        }
        Ok([lhs_val, rhs_val])
    }

    pub(super) fn load_operand(
        &self,
        operand: &DecodedOperand,
    ) -> Result<BasicValueEnum<'ctx>, BuilderError> {
        // dbg!("loading operand: {operand:?}");
        let op = match &operand.kind {
            DecodedOperandKind::Imm(imm_info) => self.load_imm(imm_info, operand.size),
            DecodedOperandKind::Reg(reg) => self.load_reg(reg)?, // Only this tested
            //DecodedOperandKind::Reg(reg) => self.retdec_load_reg(reg, None, None)?.unwrap(),
            DecodedOperandKind::Mem(mem_info) => self.load_mem(mem_info)?,
            DecodedOperandKind::Ptr(_) => unimplemented!("Pointer operands are not implemented"),
            DecodedOperandKind::Unused => unreachable!(),
        };

        Ok(op)
    }

    pub(super) fn load_stack_pointer_reg(&self) -> Register {
        Register::SP.largest_enclosing(self.mode)
    }

    //pub(crate) fn load_stack_pointer_value(&self) -> IntValue<'ctx> {
    //    let sp = self.load_stack_pointer_reg();
    //    self.load_reg(&sp).unwrap().into_int_value()
    //}

    pub(super) fn load_mem(
        &self,
        mem_info: &MemoryInfo,
    ) -> Result<BasicValueEnum<'ctx>, BuilderError> {
        //let addr = self.calc_mem_operand(mem_info)?;
        let addr = self.retdec_calc_mem_operand(mem_info)?;

        let i64_type = self.context.i64_type();
        let addr_to_ptr = self.builder.build_int_to_ptr(
            addr,
            self.context.ptr_type(AddressSpace::default()),
            "mem_",
        )?;
        let val = self.builder.build_load(i64_type, addr_to_ptr, "")?;
        Ok(val)
    }

    pub(super) fn load_imm(&self, imm: &ImmediateInfo, size: u16) -> BasicValueEnum<'ctx> {
        let ty = util::get_int_ty(self.context, size.into());
        ty.const_int(imm.value, imm.is_signed).as_basic_value_enum()
    }

    pub(super) fn load_reg(
        &self,
        register: &Register,
    ) -> Result<BasicValueEnum<'ctx>, BuilderError> {
        let largest_enclosing = register.largest_enclosing(self.mode);
        //let context = self.context;
        //let int_type = get_int_type(context, register, self.mode);

        // TODO: Bring it back
        // if register.class() == RegisterClass::IP {
        //     todo!()
        // }

        let regs_hashmap = self.regs_hashmap.borrow();
        let cached_reg = regs_hashmap.get(&largest_enclosing.into()).unwrap();

        // Special handling of 8 bit upper registers
        if [Register::AH, Register::BH, Register::CH, Register::DH].contains(register) {
            let cached_reg = cached_reg.into_int_value();
            let cached_reg_ty = cached_reg.get_type();
            let ret = self.builder.build_left_shift(
                cached_reg,
                cached_reg_ty.const_int(8, false),
                // "shifted_",
                &format!("shifted_{}", register),
            )?;
            Ok(ret.as_basic_value_enum())
        } else {
            // TODO: Understand why i even had that code block
            //let ret = self.builder.build_int_truncate(
            //    cached_reg.into_int_value(),
            //    int_type,
            //    // "truncated_",
            //    &format!("truncated_{}", register),
            //)?;
            //Ok(BasicValueEnum::from(ret))
            Ok(cached_reg.as_basic_value_enum())
        }
    }

    pub(super) fn load_flag(&self, cpu_flag: &ExtendedRegister) -> IntValue<'ctx> {
        let regs_hashmap = self.regs_hashmap.borrow();
        regs_hashmap.get(cpu_flag).unwrap().into_int_value()
    }
}
