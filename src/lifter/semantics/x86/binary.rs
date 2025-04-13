use super::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use inkwell::{values::IntValue, IntPredicate};
use zydis::{Instruction, Mnemonic, Operands};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn lift_adc<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];

        let lhs = self.load_single_int_op(dest, dest.size)?;
        let rhs = self.load_single_int_op(src, dest.size)?;

        let cf = self.load_flag(ExtendedRegisterEnum::CF)?;
        let cf_extended = builder.build_int_z_extend(cf, lhs.get_type(), "cf_extended")?;

        let temp_result = builder.build_int_add(lhs, rhs, "adc_temp")?;
        let result = builder.build_int_add(temp_result, cf_extended, "adc_result")?;

        let cf_after_first_add = builder.build_or(
            builder.build_int_compare(IntPredicate::ULT, temp_result, lhs, "")?,
            builder.build_int_compare(IntPredicate::ULT, temp_result, rhs, "")?,
            "",
        )?;

        let cf_final = builder.build_or(
            cf_after_first_add,
            builder.build_int_compare(IntPredicate::ULT, result, cf_extended, "")?,
            "",
        )?;
        let af = self.compute_aux_flag(lhs, rhs, result)?;
        let of = self.compute_overflow_flag_add(lhs, rhs, result)?;
        let pf = self.compute_parity_flag(result)?;
        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf_final);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        self.store_op(dest, result)?;
        Ok(())
    }

    pub(super) fn lift_add_sub<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];

        let lhs = self.load_single_int_op(dest, dest.size)?;
        let rhs = self.load_single_int_op(src, dest.size)?;

        let l_value_ty = lhs.get_type();

        let [result, af, cf, of] = if instr.mnemonic == Mnemonic::ADD {
            let result = builder.build_int_add(lhs, rhs, "real_add_")?;

            let af = {
                let lower_nibble_mask = l_value_ty.const_int(0xF, false);
                let r_value_lower_nibble = builder.build_and(lhs, lower_nibble_mask, "")?;
                let op_2_lower_nibble = builder.build_and(rhs, lower_nibble_mask, "")?;
                let sum_lower_nibble =
                    builder.build_int_add(r_value_lower_nibble, op_2_lower_nibble, "")?;
                builder.build_int_compare(
                    IntPredicate::UGT,
                    sum_lower_nibble,
                    lower_nibble_mask,
                    "",
                )?
            };
            let cf = builder.build_or(
                builder.build_int_compare(IntPredicate::ULT, result, lhs, "")?,
                builder.build_int_compare(IntPredicate::ULT, result, rhs, "")?,
                "",
            )?;
            let of = self.compute_overflow_flag_add(lhs, rhs, result)?;
            [result, af, cf, of]
        } else if instr.mnemonic == Mnemonic::SUB {
            let result = builder.build_int_sub(lhs, rhs, "real_sub_")?;

            let af = {
                let lower_nibble_mask = l_value_ty.const_int(0xF, false);
                let r_value_lower_nibble = builder.build_and(lhs, lower_nibble_mask, "")?;
                let op_2_lower_nibble = builder.build_and(rhs, lower_nibble_mask, "")?;
                builder.build_int_compare(
                    IntPredicate::ULT,
                    r_value_lower_nibble,
                    op_2_lower_nibble,
                    "",
                )?
            };
            let cf = builder.build_int_compare(IntPredicate::ULT, rhs, lhs, "")?;
            let of = self.compute_overflow_flag_sub(lhs, rhs, result)?;

            [result, af, cf, of]
        } else {
            unreachable!()
        };

        let pf = self.compute_parity_flag(result)?;
        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);

        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        self.store_op(dest, result)?;
        Ok(())
    }

    pub(super) fn lift_cmp<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let l_value: IntValue<'_> = self.load_single_op(&ops[0], ops[0].size)?.try_into()?;
        let r_value: IntValue<'_> = self.load_single_op(&ops[1], ops[0].size)?.try_into()?;

        let cmp_result = builder.build_int_sub(l_value, r_value, "cmp_result")?;

        let of = {
            let sign_l = builder.build_int_compare(
                IntPredicate::SLT,
                l_value,
                l_value.get_type().const_zero(),
                "cmp_sign_l",
            )?;
            let sign_r = builder.build_int_compare(
                IntPredicate::SLT,
                r_value,
                r_value.get_type().const_zero(),
                "cmp_sign_l",
            )?;

            let sign_result = builder.build_int_compare(
                IntPredicate::SLT,
                cmp_result,
                cmp_result.get_type().const_zero(),
                "cmp_sign_result",
            )?;
            builder.build_or(
                builder.build_and(
                    sign_l,
                    builder.build_and(
                        builder.build_not(sign_r, "cmp_not_sign_r_")?,
                        builder.build_not(sign_result, "cmp_not_sign_r")?,
                        "cmp_and_1_",
                    )?,
                    "",
                )?,
                builder.build_and(
                    builder.build_not(sign_l, "")?,
                    builder.build_and(sign_r, sign_result, "")?,
                    "cmp_and_2_",
                )?,
                "",
            )?
        };

        let af = {
            let lower_nibble_mask = l_value.get_type().const_int(0xF, false);
            let r_value_lower_nibble =
                builder.build_and(l_value, lower_nibble_mask, "lvalLowerNibble")?;
            let op2_lower_nibble =
                builder.build_and(r_value, lower_nibble_mask, "rvalLowerNibble")?;
            builder.build_int_compare(
                IntPredicate::ULT,
                r_value_lower_nibble,
                op2_lower_nibble,
                "sub_af",
            )?
        };

        let cf = builder.build_int_compare(IntPredicate::ULT, l_value, r_value, "cmp_cf_")?;
        let pf = self.compute_parity_flag(cmp_result)?;
        let sf = self.compute_sign_flag(cmp_result)?;
        let zf = self.compute_zero_flag(cmp_result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        Ok(())
    }

    pub(super) fn lift_dec<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let op = &(instr.operands())[0];

        let lhs: IntValue<'_> = self.load_single_op(op, op.size)?.try_into()?;
        let lhs_ty = lhs.get_type();
        let one = lhs_ty.const_int(1, true);

        let result = builder.build_int_sub(lhs, one, "dec_result")?;

        let af = {
            let lower_nibble_mask = lhs_ty.const_int(0xf, false);
            let r_value_lower_nibble =
                builder.build_and(lhs, lower_nibble_mask, "lvalLowerNibble")?;
            let op2_lower_nibble = one;
            builder.build_int_compare(
                IntPredicate::ULT,
                r_value_lower_nibble,
                op2_lower_nibble,
                "dec_af",
            )?
        };
        let of = self.compute_overflow_flag_sub(lhs, one, result)?;
        let pf = self.compute_parity_flag(result)?;
        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        self.store_op(op, result)?;

        Ok(())
    }

    pub(super) fn lift_inc<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let op = &(instr.operands())[0];

        let lhs: IntValue<'_> = self.load_single_op(op, op.size)?.try_into()?;
        let one = lhs.get_type().const_int(1, true);

        let result = builder.build_int_add(lhs, one, "dec_result")?;

        let af = {
            let lower_nibble_mask = lhs.get_type().const_int(0xf, false);
            let dest_lower_nibble = builder.build_and(lhs, lower_nibble_mask, "lvalLowerNibble")?;
            let src_lower_nibble = one;
            let sum_lower_nibble =
                builder.build_int_add(dest_lower_nibble, src_lower_nibble, "")?;
            builder.build_int_compare(
                IntPredicate::UGT,
                sum_lower_nibble,
                lower_nibble_mask,
                "inc_af",
            )?
        };

        let of = self.compute_overflow_flag_add(lhs, one, result)?;
        let pf = self.compute_parity_flag(result)?;
        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        self.store_op(op, result)?;

        Ok(())
    }

    pub(super) fn lift_neg<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();
        let dest = &ops[0];

        let r_value: IntValue<'_> = self.load_single_op(dest, dest.size)?.try_into()?;
        let r_value_ty = r_value.get_type();

        let result = builder.build_int_sub(
            self.context
                .custom_width_int_type(r_value_ty.get_bit_width())
                .const_zero(),
            r_value,
            "neg",
        )?;
        self.store_op(dest, result)?;

        let af = {
            let fifteen = r_value_ty.const_int(0xf, false);
            builder.build_int_compare(
                IntPredicate::NE,
                builder.build_and(r_value, fifteen, "")?,
                r_value_ty.const_zero(),
                "af",
            )?
        };
        let cf = builder.build_int_compare(
            IntPredicate::NE,
            r_value,
            r_value_ty.const_zero(),
            "neg_cf",
        )?;
        let pf = self.compute_parity_flag(result)?;
        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        let is_zero =
            builder.build_int_compare(IntPredicate::NE, r_value, r_value_ty.const_zero(), "")?;

        let mut of = builder.build_int_compare(IntPredicate::EQ, result, r_value, "")?;
        of = builder
            .build_select(is_zero, of, of.get_type().const_zero(), "")?
            .into_int_value();

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        Ok(())
    }

    pub(super) fn lift_sbb<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];

        let l_value = self.load_single_int_op(dest, dest.size)?;
        let r_value = self.load_single_int_op(src, dest.size)?;

        let cf = self.create_z_ext_or_trunc(
            self.load_flag(ExtendedRegisterEnum::CF)?,
            r_value.get_type(),
        )?;

        let tmp_result = builder.build_int_add(l_value, r_value, "")?;
        let result = builder.build_int_sub(tmp_result, cf, "")?;
        self.store_op(dest, result)?;

        let new_cf = builder.build_or(
            builder.build_int_compare(IntPredicate::ULT, l_value, r_value, "")?,
            builder.build_int_compare(IntPredicate::ULT, tmp_result, cf, "")?,
            "",
        )?;

        let af = self.compute_aux_flag(l_value, r_value, result)?;
        let pf = self.compute_parity_flag(result)?;
        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        let of = {
            let bit_width = l_value.get_type().get_bit_width();
            let sign_bit = self
                .context
                .custom_width_int_type(bit_width)
                .const_int((bit_width - 1).into(), false);

            let lhs_sign = builder.build_right_shift(l_value, sign_bit, false, "")?;
            let rhs_sign = builder.build_right_shift(r_value, sign_bit, false, "")?;
            let result_sign = builder.build_right_shift(result, sign_bit, false, "")?;

            let result_idk = builder.build_xor(lhs_sign, rhs_sign, "")?;
            let result_idk_2 = builder.build_xor(lhs_sign, result_sign, "")?;
            let result_idk_3 = builder.build_xor(result_idk, result_idk_2, "")?;

            builder.build_int_compare(
                IntPredicate::EQ,
                result_idk_3,
                result_idk_3.get_type().const_int(2, false),
                "",
            )?
        };

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, new_cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        Ok(())
    }

    // Region: stolen from Mergen
    fn compute_overflow_flag_add(
        &self,
        l_value: IntValue<'ctx>,
        r_value: IntValue<'ctx>,
        add: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let xor0 = builder.build_xor(l_value, add, "")?;
        let xor1 = builder.build_xor(r_value, add, "")?;

        let of_and = builder.build_and(xor0, xor1, "")?;

        let r = builder.build_int_compare(
            IntPredicate::SLT,
            of_and,
            of_and.get_type().const_zero(),
            "",
        )?;
        Ok(r)
    }

    fn compute_overflow_flag_sub(
        &self,
        l_value: IntValue<'ctx>,
        r_value: IntValue<'ctx>,
        sub: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;

        let xor0 = builder.build_xor(l_value, r_value, "")?;
        let xor1 = builder.build_xor(l_value, sub, "")?;
        let of_and = builder.build_and(xor0, xor1, "")?;

        let ret = builder.build_int_compare(
            IntPredicate::SLT,
            of_and,
            of_and.get_type().const_zero(),
            "",
        )?;

        Ok(ret)
    }
}
