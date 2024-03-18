use crate::lifter::{eOpConv, LifterX86};
use crate::util::get_int_type;
use inkwell::builder::BuilderError;
use inkwell::values::{BasicValue, BasicValueEnum};
use inkwell::AddressSpace;
use zydis::ffi::{DecodedOperand, DecodedOperandKind, MemoryInfo};
use zydis::Register;

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub fn load_2_operands(
        &self,
        operands: &[DecodedOperand],
        if_not_equal: eOpConv,
    ) -> Result<(BasicValueEnum, BasicValueEnum), BuilderError> {
        let lhs = self.load_operand(&operands[0])?;

        let rhs = self.load_operand(&operands[1])?;

        let rhs_final = match if_not_equal {
            eOpConv::Nothing => rhs,
            eOpConv::SEXT_TRUNC_OR_BITCAST => self
                .create_s_ext_or_trunc(rhs.into_int_value(), lhs.get_type().into_int_type())?
                .as_basic_value_enum(),
            _ => unimplemented!(),
        };

        Ok((lhs, rhs_final))
    }

    pub fn load_operand(&self, operand: &DecodedOperand) -> Result<BasicValueEnum, BuilderError> {
        // dbg!("loading operand: {operand:?}");
        Ok(match &operand.kind {
            DecodedOperandKind::Imm(imm_info) => self.load_imm(imm_info.value, imm_info.is_signed),
            DecodedOperandKind::Reg(reg) => self.load_reg(reg)?, // Only this tested
            DecodedOperandKind::Mem(mem_info) => self.load_mem(mem_info)?,
            DecodedOperandKind::Ptr(_) => unimplemented!("Pointer operands are not implemented"),
            DecodedOperandKind::Unused => unreachable!(),
        })
    }

    pub fn load_stack_pointer(&self) -> BasicValueEnum {
        let sp = Register::SP.largest_enclosing(*self.mode);
        self.load_reg(&sp).unwrap()
    }

    fn load_mem(&self, mem_info: &MemoryInfo) -> Result<BasicValueEnum, BuilderError> {
        let addr = self.calc_mem_operand(mem_info)?;

        let i64_type = self.context.i64_type();
        let addr_to_ptr = self.builder.build_int_to_ptr(
            addr.into_int_value(),
            i64_type.ptr_type(AddressSpace::default()),
            "mem_",
        )?;
        let val = self.builder.build_load(i64_type, addr_to_ptr, "")?;
        Ok(val)
    }
    fn load_imm(&self, value: u64, is_signed: bool) -> BasicValueEnum<'ctx> {
        let value = self.context.i64_type().const_int(value, is_signed);
        BasicValueEnum::from(value)
    }
    pub(in crate::lifter) fn load_reg(
        &self,
        register: &Register,
    ) -> Result<BasicValueEnum, BuilderError> {
        let largest_enclosing = register.largest_enclosing(*self.mode);
        let context = self.context;
        let int_type = get_int_type(context, register, self.mode);

        // TODO: Bring it back
        // if register.class() == RegisterClass::IP {
        //     todo!()
        // }

        let regs_hashmap = self.regs_hashmap.borrow();
        let cached_reg = regs_hashmap.get(&largest_enclosing).unwrap();

        // Special handling of 8 bit upper registers
        if [Register::AH, Register::BH, Register::CH, Register::DH].contains(register) {
            let ret = self.builder.build_left_shift(
                cached_reg.into_int_value(),
                self.context.i8_type().const_int(8, false),
                // "shifted_",
                &format!("shifted_{}", register),
            )?;
            Ok(BasicValueEnum::from(ret))
        } else {
            let ret = self.builder.build_int_truncate(
                cached_reg.into_int_value(),
                int_type,
                // "truncated_",
                &format!("truncated_{}", register),
            )?;
            Ok(BasicValueEnum::from(ret))
        }
    }
}
