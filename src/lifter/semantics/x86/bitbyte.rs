use super::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegister;

use inkwell::{values::IntValue, IntPredicate};
use zydis::{Instruction, Operands};

impl LifterX86<'_> {
    pub(super) fn lift_bsr<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = &instr.operands();

        let dst = &ops[0];
        let src = &ops[1];

        let r_value: IntValue<'_> = self.load_single_op(src, src.size)?.try_into()?;
        let r_value_ty = r_value.get_type();
        let is_zero =
            builder.build_int_compare(IntPredicate::EQ, r_value, r_value_ty.const_zero(), "")?;

        self.store_cpu_flag(ExtendedRegister::ZF, is_zero);

        let bit_width = r_value_ty.get_bit_width();

        let mut index = r_value_ty.const_int((bit_width - 1).into(), false);
        let zero_val = r_value_ty.const_zero();
        let one_val = r_value_ty.const_int(1, false);

        // Basically -1
        let mut bit_position = r_value_ty.const_int(u64::MAX, true);

        for _ in 0..bit_width {
            let mask = builder.build_left_shift(one_val, index, "")?;

            let test = builder.build_and(r_value, mask, "")?;
            let is_bit_set = builder.build_int_compare(IntPredicate::NE, test, zero_val, "")?;

            let tmp_position = builder
                .build_select(is_bit_set, index, bit_position, "")?
                .into_int_value();

            let is_position_unset = builder.build_int_compare(
                IntPredicate::EQ,
                bit_position,
                r_value_ty.const_int(u64::MAX, true),
                "",
            )?;

            bit_position = builder
                .build_select(is_position_unset, tmp_position, bit_position, "")?
                .into_int_value();

            index = builder.build_int_sub(index, one_val, "")?;
        }

        self.store_op(dst, index)?;
        Ok(())
    }

    pub(super) fn lift_bsf<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = &instr.operands();

        let dst = &ops[0];
        let src = &ops[1];

        let r_value: IntValue<'_> = self.load_single_op(src, src.size)?.try_into()?;
        let is_zero = builder.build_int_compare(
            IntPredicate::EQ,
            r_value,
            r_value.get_type().const_zero(),
            "",
        )?;

        self.store_cpu_flag(ExtendedRegister::ZF, is_zero);

        let int_type = r_value.get_type();
        let int_width = int_type.get_bit_width();

        let mut result = int_type.const_int(int_width.into(), false);
        let one = int_type.const_int(1, false);

        let mut continue_counting = self.context.custom_width_int_type(1).const_int(1, false);
        for i in 0..int_width {
            let bit_mask =
                builder.build_left_shift(one, int_type.const_int(i.into(), false), "")?;
            let bit_set = builder.build_and(r_value, bit_mask, "")?;
            let is_bit_zero =
                builder.build_int_compare(IntPredicate::EQ, bit_set, int_type.const_zero(), "")?;

            let possible_result = int_type.const_int(i.into(), false);
            let condition = builder.build_and(continue_counting, is_bit_zero, "")?;

            continue_counting = builder.build_not(is_bit_zero, "")?;

            result = builder
                .build_select(condition, result, possible_result, "")?
                .into_int_value();
        }

        self.store_op(dst, result)?;

        Ok(())
    }

    pub(super) fn lift_bt<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let [l_value, bit_index_value] = self.load_two_first_ints(ops)?;

        let l_value_bit_w = l_value.get_type().get_bit_width();

        let r_value = builder.build_and(
            bit_index_value,
            bit_index_value
                .get_type()
                .const_int((l_value_bit_w - 1).into(), false),
            "",
        )?;

        let shl = builder.build_left_shift(
            bit_index_value.get_type().const_int(1, false),
            r_value,
            "",
        )?;

        let and = builder.build_and(shl, l_value, "")?;
        let cf =
            builder.build_int_compare(IntPredicate::NE, and, and.get_type().const_zero(), "")?;

        self.store_cpu_flag(ExtendedRegister::CF, cf);

        Ok(())
    }

    // TODO: Check
    pub(super) fn lift_btc<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let bldr = &self.builder;
        let ops = instr.operands();
        let operands = instr.operands();

        let loaded_operands = self.load_two_first_ops(operands)?;

        //let operands = self.retdec_loadOpBinary(ops, eOpConv::SEXT_TRUNC_OR_BITCAST)?;
        let op0: IntValue<'_> = loaded_operands[0].try_into()?;

        let mut op1: IntValue<'_> = loaded_operands[1].try_into()?;
        let op0_bit_w = bldr
            .build_int_cast(op0, op0.get_type(), "")?
            .get_type()
            .get_bit_width();
        op1 = bldr.build_and(
            op1,
            op1.get_type().const_int((op0_bit_w - 1).into(), false),
            "",
        )?;

        let srl = bldr.build_right_shift(op0, op1, false, "")?;
        let and1 = bldr.build_and(srl, srl.get_type().const_int(1, false), "")?;
        let icmp = bldr.build_int_compare(
            inkwell::IntPredicate::NE,
            and1,
            and1.get_type().const_zero(),
            "",
        )?;
        self.store_cpu_flag(ExtendedRegister::CF, icmp);

        let shl = bldr.build_left_shift(op1.get_type().const_int(1, false), op1, "")?;
        // TODO: Check this. In original it has -1 and sign extend
        let xor1 = bldr.build_xor(shl, shl.get_type().const_int(u64::MAX, true), "")?;
        let and2 = bldr.build_int_add(op1, xor1, "")?;
        let xor2 = bldr.build_xor(and1, and1.get_type().const_int(1, false), "")?;
        let shl2 = bldr.build_left_shift(xor2, op1, "")?;
        let or1 = bldr.build_or(shl2, and2, "")?;

        //self.retdec_store_op(&ops[0], or1, None)?;
        self.store_op(&ops[0], or1)?;

        Ok(())
    }

    pub(super) fn lift_btr<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();
        let base = &ops[0];

        let base_bit_width: u64 = base.size.into();

        let [mut base_val, bit_offset] = self.load_two_first_ints(ops)?;
        //let base_val_ty = base_val.get_type();
        let bit_offset_ty = bit_offset.get_type();

        let bit_offset_masked = builder.build_and(
            bit_offset,
            bit_offset_ty.const_int(base_bit_width - 1, false),
            "",
        )?;

        let mut bit = builder.build_right_shift(base_val, bit_offset_masked, false, "")?;
        let one = bit.get_type().const_int(1, false);

        bit = builder.build_and(bit, one, "")?;

        self.store_cpu_flag(ExtendedRegister::CF, bit);

        let mut mask = builder.build_left_shift(
            base_val.get_type().const_int(1, false),
            bit_offset_masked,
            "",
        )?;

        mask = builder.build_not(mask, "")?;
        base_val = builder.build_and(base_val, mask, "")?;

        self.store_op(base, base_val)?;

        Ok(())
    }

    // TODO: Check this too
    pub(super) fn lift_bts<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let bldr = &self.builder;
        let ops = instr.operands();

        let operands = instr.operands();

        let [op0, mut op1] = self.load_two_first_ints(operands)?;

        let op0_bit_w = bldr
            .build_int_cast(op0, op0.get_type(), "")?
            .get_type()
            .get_bit_width();

        op1 = bldr.build_and(
            op1,
            op1.get_type().const_int((op0_bit_w - 1) as u64, false),
            "",
        )?;

        let shl = bldr.build_left_shift(op1.get_type().const_int(1, false), op1, "")?;
        let and = bldr.build_int_add(shl, op0, "")?;
        let icmp = bldr.build_int_compare(
            inkwell::IntPredicate::NE,
            and,
            and.get_type().const_zero(),
            "",
        )?;
        self.store_cpu_flag(ExtendedRegister::CF, icmp);

        let or1 = bldr.build_or(shl, op0, "")?;
        //self.store_op(&ops[0].kind, or1)?;
        self.store_op(&ops[0], or1)?;

        Ok(())
    }
}
