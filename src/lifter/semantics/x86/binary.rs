use super::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use inkwell::{values::IntValue, IntPredicate};
use zydis::{Instruction, Operands};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn lift_adc<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let [lhs, rhs] = self.load_two_first_ints(ops)?;

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
        let of = self.generate_overflow_add(&lhs, &rhs, &result)?;
        let pf = self.compute_parity_flag(result)?;
        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf_final);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        self.store_op(&ops[0], result)?;
        Ok(())
    }

    pub(super) fn lift_add<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let ops = instr.operands();

        let [lhs, rhs] = self.load_two_first_ints(ops)?;

        let result = if lhs.is_null() {
            rhs
        } else if rhs.is_null() {
            lhs
        } else {
            self.builder.build_int_add(lhs, rhs, "add_result_")?
        };

        //let result = self.builder.build_int_add(lhs, rhs, "add_result_")?;

        let af_val = self.generate_carry_add_int4(&lhs, &rhs)?;
        let cf_val = self.generate_carry_add(&result, &lhs)?;
        let of_val = self.generate_overflow_add(&result, &lhs, &rhs)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af_val);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf_val);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of_val);

        self.calculate_and_store_common_flags_for_add_sub(result)?;

        self.store_op(&ops[0], result)?;
        if instr.mnemonic == zydis::Mnemonic::XADD {
            //self.retdec_store_op(&operands[1], lhs, None)?;
            self.store_op(&ops[1], result)?;
        }
        Ok(())
    }

    pub(super) fn lift_cmp<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let [lhs, rhs] = self.load_two_first_ints(ops)?;

        let cmp_result = builder.build_int_sub(lhs, rhs, "cmp_result")?;

        let sign_l = builder.build_int_compare(
            IntPredicate::SLT,
            lhs,
            lhs.get_type().const_zero(),
            "cmp_sign_l",
        )?;
        let sign_r = builder.build_int_compare(
            IntPredicate::SLT,
            rhs,
            rhs.get_type().const_zero(),
            "cmp_sign_l",
        )?;

        let sign_result = builder.build_int_compare(
            IntPredicate::SLT,
            cmp_result,
            cmp_result.get_type().const_zero(),
            "cmp_sign_result",
        )?;

        let af = {
            let lower_nibble_mask = lhs.get_type().const_int(0xf, false);
            let r_value_lower_nibble =
                builder.build_and(lhs, lower_nibble_mask, "lvalLowerNibble")?;
            let op2_lower_nibble = builder.build_and(rhs, lower_nibble_mask, "rvalLowerNibble")?;
            builder.build_int_compare(
                IntPredicate::ULT,
                r_value_lower_nibble,
                op2_lower_nibble,
                "sub_af",
            )?
        };

        let of = builder.build_or(
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
        )?;

        let cf = builder.build_int_compare(IntPredicate::ULT, lhs, rhs, "cmp_cf_")?;
        let zf = builder.build_int_compare(
            IntPredicate::EQ,
            cmp_result,
            cmp_result.get_type().const_zero(),
            "cmp_zf_",
        )?;
        let sf = builder.build_int_compare(
            IntPredicate::SLT,
            cmp_result,
            cmp_result.get_type().const_zero(),
            "cmp_sf_",
        )?;

        let pf = self.compute_parity_flag(cmp_result)?;

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
        let one = lhs.get_type().const_int(1, true);

        let result = builder.build_int_sub(lhs, one, "dec_result")?;

        let af = {
            let lower_nibble_mask = lhs.get_type().const_int(0xf, false);
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
        let of = self.generate_overflow_sub(&result, &lhs, &one)?;
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
            let r_value_lower_nibble =
                builder.build_and(lhs, lower_nibble_mask, "lvalLowerNibble")?;
            let op2_lower_nibble = one;
            builder.build_int_compare(
                IntPredicate::ULT,
                r_value_lower_nibble,
                op2_lower_nibble,
                "inc_af",
            )?
        };
        let of = self.generate_overflow_add(&result, &lhs, &one)?;
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

    // TODO: Steal from bochs instead
    pub(super) fn lift_neg<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let op = &(instr.operands())[0];

        let r_value: IntValue<'_> = self.load_single_op(op, op.size)?.try_into()?;
        let r_value_ty = r_value.get_type();

        let result = builder.build_int_sub(
            self.context
                .custom_width_int_type(r_value_ty.get_bit_width())
                .const_zero(),
            r_value,
            "neg",
        )?;
        self.store_op(op, result)?;

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

        // Store OF flag
        if op.size > 32 {
            self.store_cpu_flag_bool(ExtendedRegisterEnum::OF, false);
        } else {
            let is_zero = builder.build_int_compare(
                IntPredicate::NE,
                r_value,
                r_value_ty.const_zero(),
                "",
            )?;
            let of = builder.build_int_compare(IntPredicate::EQ, result, r_value, "")?;
            let of = builder.build_select(is_zero, of, of.get_type().const_zero(), "neg_of")?;
            self.store_cpu_flag(ExtendedRegisterEnum::OF, of.into_int_value());
        };

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        Ok(())
    }

    pub(super) fn lift_sbb<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let [l_value, r_value] = self.load_two_first_ints(ops)?;
        let cf = self.create_z_ext_or_trunc(
            self.load_flag(ExtendedRegisterEnum::CF)?,
            r_value.get_type(),
        )?;

        let tmp_result = builder.build_int_add(r_value, cf, "")?;
        let result = builder.build_int_sub(l_value, tmp_result, "")?;
        self.store_op(&ops[0], result)?;

        let new_cf = builder.build_int_compare(IntPredicate::ULT, l_value, tmp_result, "")?;

        let af = self.compute_aux_flag(l_value, r_value, result)?;
        let of = self.compute_overflow_flag_sbb(l_value, r_value, cf, result)?;
        let pf = self.compute_parity_flag(result)?;
        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, new_cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        Ok(())
    }

    pub(super) fn lift_sub<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let ops = instr.operands();

        let [lhs, rhs] = self.load_two_first_ints(ops)?;

        let result = if lhs.is_null() {
            rhs
        } else if rhs.is_null() {
            lhs
        } else {
            self.builder.build_int_sub(lhs, rhs, "add_result_")?
        };
        //let result = self
        //    .builder
        //    .build_int_sub(l_value, r_value, "sub_result_")?;

        let af_val = self.generate_borrow_sub_int4(&lhs, &rhs)?;
        let cf_val = self.generate_borrow_sub(&result, &lhs)?;
        let of_val = self.generate_overflow_sub(&result, &lhs, &rhs)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af_val);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf_val);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of_val);

        self.calculate_and_store_common_flags_for_add_sub(result)?;

        self.store_op(&ops[0], result)?;
        //self.retdec_store_op(&operands[0], result, None)?;
        Ok(())
    }

    //pub(super) fn lift_sub<O: Operands>(&self, instr: Instruction<O>) -> Result<()> {
    //    let operands = instr.operands();
    //    let loaded = self.experimental_load_two_ops(operands)?;
    //
    //    let lhs: IntValue<'ctx> = loaded[0].try_into()?;
    //    let rhs: IntValue<'ctx> = loaded[1].try_into()?;
    //
    //    let result = self.builder.build_int_sub(lhs, rhs, "sub_result_")?;
    //
    //    let af_val = self.generate_borrow_sub_int4(&lhs, &rhs)?;
    //    let cf_val = self.generate_borrow_sub(&result, &lhs)?;
    //    let of_val = self.generate_overflow_sub(&result, &lhs, &rhs)?;
    //
    //    self.store_cpu_flag(ExtendedRegister::AF, af_val);
    //    self.store_cpu_flag(ExtendedRegister::CF, cf_val);
    //    self.store_cpu_flag(ExtendedRegister::OF, of_val);
    //
    //    self.calculate_and_store_common_flags_for_add_sub(result)?;
    //
    //    self.experimental_store_op(&operands[0], result)?;
    //    //self.retdec_store_op(&operands[0], result, None)?;
    //    Ok(())
    //}

    /// Calculate the following flags: SF, ZF, PF
    fn calculate_and_store_common_flags_for_add_sub(&self, result: IntValue<'ctx>) -> Result<()> {
        self.store_cpu_flag(ExtendedRegisterEnum::SF, self.compute_sign_flag(result)?);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, self.compute_zero_flag(result)?);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, self.compute_parity_flag(result)?);

        Ok(())
    }

    // region:   Stolen from retdec
    fn generate_overflow_add(
        &self,
        add: &IntValue<'ctx>,
        op0: &IntValue<'ctx>,
        op1: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let xor0 = builder.build_xor(*op0, *add, "")?;
        let xor1 = builder.build_xor(*op1, *add, "")?;

        let of_and = builder.build_and(xor0, xor1, "")?;

        let r = builder.build_int_compare(
            IntPredicate::SLT,
            of_and,
            of_and.get_type().const_zero(),
            "",
        )?;
        Ok(r)
    }

    fn generate_carry_add(
        &self,
        add: &IntValue<'ctx>,
        op0: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let r = self
            .builder
            .build_int_compare(IntPredicate::ULT, *add, *op0, "")?;
        Ok(r)
    }

    fn generate_carry_add_int4(
        &self,
        op0: &IntValue<'ctx>,
        op1: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let ci15 = op0.get_type().const_int(0xf, false);
        let and0 = self.builder.build_and(*op0, ci15, "")?;
        let and1 = builder.build_and(*op1, ci15, "")?;
        let add = builder.build_int_add(and0, and1, "")?;

        let r = builder.build_int_compare(IntPredicate::UGT, add, ci15, "")?;
        Ok(r)
    }

    fn generate_borrow_sub_int4(
        &self,
        op0: &IntValue<'ctx>,
        op1: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let ci15 = op0.get_type().const_int(0xf, false);
        let and0 = builder.build_and(*op0, ci15, "")?;
        let and1 = builder.build_and(*op1, ci15, "")?;
        let af_sub = builder.build_int_sub(and0, and1, "")?;

        let r = builder.build_int_compare(IntPredicate::UGT, af_sub, ci15, "")?;
        Ok(r)
    }

    fn generate_borrow_sub(
        &self,
        op0: &IntValue<'ctx>,
        op1: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let r = self
            .builder
            .build_int_compare(IntPredicate::ULT, *op0, *op1, "")?;
        Ok(r)
    }

    fn generate_overflow_sub(
        &self,
        sub: &IntValue<'ctx>,
        op0: &IntValue<'ctx>,
        op1: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let xor0 = builder.build_xor(*op0, *op1, "")?;
        let xor1 = builder.build_xor(*op0, *sub, "")?;

        let of_and = builder.build_and(xor0, xor1, "")?;

        let r = builder.build_int_compare(
            IntPredicate::SLT,
            of_and,
            of_and.get_type().const_zero(),
            "",
        )?;

        Ok(r)
    }

    fn compute_overflow_flag_sbb(
        &self,
        l_value: IntValue<'ctx>,
        r_value: IntValue<'ctx>,
        cf: IntValue<'ctx>,
        sub: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let cfc = self.create_z_ext_or_trunc(cf, sub.get_type())?;
        let of_sub = builder.build_int_sub(sub, cfc, "")?;
        let xor_0 = builder.build_xor(l_value, r_value, "")?;
        let xor_1 = builder.build_xor(l_value, of_sub, "")?;
        let of_and = builder.build_and(xor_0, xor_1, "")?;

        let of = builder.build_int_compare(
            IntPredicate::SLT,
            of_and,
            of_and.get_type().const_zero(),
            "",
        )?;
        Ok(of)
    }
    // endregion:   Stolen from retdec
}
