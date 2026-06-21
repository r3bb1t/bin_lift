use super::Result;
use crate::lifter::{Error, LifterX86};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{Constant, ConstantIntValue, IntDyn, IntValue};
use zydis::{
    ffi::{DecodedOperand, DecodedOperandKind},
    Instruction, InstructionAttributes, Mnemonic, Operands, Register,
};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_movs(&mut self, mnemonic: Mnemonic) -> Result<()> {
        let size_bits = Self::movs_size_bits(mnemonic)?;
        let src_reg = Register::SI.largest_enclosing(self.mode);
        let dst_reg = Register::DI.largest_enclosing(self.mode);
        let src_addr: IntValue<'ctx, IntDyn> = self.load_register_value(&src_reg)?.try_into()?;
        let dst_addr: IntValue<'ctx, IntDyn> = self.load_register_value(&dst_reg)?.try_into()?;
        let value_ty = self.module.custom_width_int_type(size_bits)?;

        let src_ptr = self.builder()?.build_gep(
            self.module.i8_type(),
            self.stackmemory,
            [src_addr],
            "movs_src",
        )?;
        let value = self
            .builder()?
            .build_int_load_dyn(value_ty, src_ptr, "movs_load")?;
        let dst_ptr = self.builder()?.build_gep(
            self.module.i8_type(),
            self.stackmemory,
            [dst_addr],
            "movs_dst",
        )?;
        self.builder()?.build_store(value, dst_ptr)?;
        self.update_movs_index_registers(size_bits)
    }

    pub(super) fn lift_movs_x<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let size_bits = Self::movs_size_bits(instr.mnemonic)?;
        let operands = instr.operands();
        let has_rep = instr.attributes.contains(InstructionAttributes::HAS_REP)
            || instr.attributes.contains(InstructionAttributes::HAS_REPE)
            || instr.attributes.contains(InstructionAttributes::HAS_REPNE);
        if operands.len() >= 2
            && matches!(&operands[0].kind, DecodedOperandKind::Mem(_))
            && matches!(&operands[1].kind, DecodedOperandKind::Mem(_))
        {
            if has_rep {
                return self.lift_rep_movs_x(operands, size_bits);
            }
            let operand_size = u16::try_from(size_bits)
                .map_err(|_| Error::UnsupportedInstr("unsupported string move size"))?;
            let value = self.load_single_int_op(&operands[1], operand_size)?;
            self.store_op(&operands[0], value)?;
            self.update_movs_index_registers(size_bits)
        } else if has_rep {
            Err(Error::UnsupportedInstr("rep movs operand form"))
        } else {
            self.lift_movs(instr.mnemonic)
        }
    }

    fn lift_rep_movs_x(&mut self, operands: &[DecodedOperand], size_bits: u32) -> Result<()> {
        let count_op = operands
            .get(2)
            .ok_or(Error::UnsupportedInstr("rep movs missing count"))?;
        let count = self.load_single_int_op(count_op, count_op.size)?;
        let count_constant = Constant::try_from(count.as_value())
            .map_err(|_| Error::UnsupportedInstr("rep movs dynamic count"))?;
        let loop_count = ConstantIntValue::<IntDyn>::try_from(count_constant)?
            .value_zext_u128()
            .ok_or(Error::UnsupportedInstr("rep movs count too large"))?;
        let loop_count = usize::try_from(loop_count)
            .map_err(|_| Error::UnsupportedInstr("rep movs count too large"))?;
        let operand_size = u16::try_from(size_bits)
            .map_err(|_| Error::UnsupportedInstr("unsupported string move size"))?;

        for _ in 0..loop_count {
            let value = self.load_single_int_op(&operands[1], operand_size)?;
            self.store_op(&operands[0], value)?;
            self.update_movs_index_registers(size_bits)?;
        }

        self.store_op(count_op, count.ty().const_zero())
    }

    fn movs_size_bits(mnemonic: Mnemonic) -> Result<u32> {
        match mnemonic {
            Mnemonic::MOVSB => Ok(8),
            Mnemonic::MOVSW => Ok(16),
            Mnemonic::MOVSD => Ok(32),
            Mnemonic::MOVSQ => Ok(64),
            _ => Err(Error::UnsupportedInstr(
                "unsupported string move instruction",
            )),
        }
    }

    fn update_movs_index_registers(&mut self, size_bits: u32) -> Result<()> {
        let src_reg = Register::SI.largest_enclosing(self.mode);
        let dst_reg = Register::DI.largest_enclosing(self.mode);
        let src_addr: IntValue<'ctx, IntDyn> = self.load_register_value(&src_reg)?.try_into()?;
        let dst_addr: IntValue<'ctx, IntDyn> = self.load_register_value(&dst_reg)?.try_into()?;
        let step_bytes = u64::from(size_bits / 8);
        let df = self.load_flag(ExtendedRegisterEnum::DF)?;
        let decrement = self.builder()?.build_icmp_ne::<IntDyn, _, _, _>(
            df,
            df.ty().const_zero(),
            "movs_df",
        )?;

        let src_step = src_addr.ty().const_int_raw(step_bytes, false)?;
        let src_inc =
            self.builder()?
                .build_int_add::<IntDyn, _, _, _>(src_addr, src_step, "movs_si_inc")?;
        let src_dec =
            self.builder()?
                .build_int_sub::<IntDyn, _, _, _>(src_addr, src_step, "movs_si_dec")?;
        let next_src = self
            .builder()?
            .build_select(decrement, src_dec, src_inc, "movs_si")?;

        let dst_step = dst_addr.ty().const_int_raw(step_bytes, false)?;
        let dst_inc =
            self.builder()?
                .build_int_add::<IntDyn, _, _, _>(dst_addr, dst_step, "movs_di_inc")?;
        let dst_dec =
            self.builder()?
                .build_int_sub::<IntDyn, _, _, _>(dst_addr, dst_step, "movs_di_dec")?;
        let next_dst = self
            .builder()?
            .build_select(decrement, dst_dec, dst_inc, "movs_di")?;

        self.store_reg(src_reg, next_src)?;
        self.store_reg(dst_reg, next_dst)
    }
}
