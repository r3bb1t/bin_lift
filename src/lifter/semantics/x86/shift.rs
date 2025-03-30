use super::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use inkwell::{values::IntValue, IntPredicate};
use zydis::{Instruction, Mnemonic, Operands};

impl LifterX86<'_> {
    //pub(super) fn lift_sar<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
    //    let builder = &self.builder;
    //    let ops = instr.operands();
    //
    //    let [l_value, count_value] = self.load_two_first_ints(ops)?;
    //    let l_value_ty = l_value.get_type();
    //    let count_value_ty = count_value.get_type();
    //
    //    let zero = count_value.get_type().const_zero();
    //    let bit_width: u64 = l_value_ty.get_bit_width().into();
    //    let mask_c: u64 = if bit_width == 64 { 0x3f } else { 0x1f };
    //
    //    let clamped_count =
    //        builder.build_and(count_value, count_value_ty.const_int(mask_c, false), "")?;
    //
    //    let mut result = builder.build_right_shift(l_value, clamped_count, false, "")?;
    //    let is_zeroed = builder.build_int_compare(
    //        IntPredicate::ULT,
    //        clamped_count,
    //        clamped_count.get_type().const_int(bit_width - 1, false),
    //        "",
    //    )?;
    //
    //    result = builder
    //        .build_select(is_zeroed, zero, result, "aaa")?
    //        .into_int_value();
    //
    //    let cf_r_value = builder.build_int_sub(
    //        clamped_count,
    //        clamped_count.get_type().const_int(1, false),
    //        "",
    //    )?;
    //    let cf_shl =
    //        builder.build_left_shift(cf_r_value.get_type().const_int(1, false), cf_r_value, "")?;
    //    let cf_and = builder.build_and(cf_shl, l_value, "")?;
    //    let mut cf_value = builder.build_int_compare(
    //        IntPredicate::NE,
    //        cf_and,
    //        cf_and.get_type().const_zero(),
    //        "",
    //    )?;
    //
    //    let is_count_one = builder.build_int_compare(
    //        IntPredicate::EQ,
    //        clamped_count,
    //        clamped_count.get_type().const_int(1, false),
    //        "",
    //    )?;
    //
    //    let int_1_ty = self.context.custom_width_int_type(1);
    //    let of = builder
    //        .build_select(
    //            is_count_one,
    //            int_1_ty.const_int(1, false),
    //            self.load_flag(ExtendedRegister::OF)?,
    //            "bbb",
    //        )?
    //        .into_int_value();
    //
    //    let is_not_zero = builder.build_int_compare(IntPredicate::NE, clamped_count, zero, "")?;
    //    let old_cf = self.load_flag(ExtendedRegister::CF)?;
    //    cf_value = builder
    //        .build_select(
    //            is_not_zero,
    //            cf_value,
    //            self.create_z_ext_or_trunc(old_cf, int_1_ty)?,
    //            "ccc",
    //        )?
    //        .into_int_value();
    //    cf_value = builder
    //        .build_select(
    //            is_zeroed,
    //            builder.build_int_truncate(zero, int_1_ty, "ddd")?,
    //            cf_value,
    //            "",
    //        )?
    //        .into_int_value();
    //
    //    let sf = builder
    //        .build_select(
    //            is_not_zero,
    //            self.compute_sign_flag(result)?,
    //            self.load_flag(ExtendedRegister::SF)?,
    //            "eee",
    //        )?
    //        .into_int_value();
    //
    //    let zf = builder
    //        .build_select(
    //            is_not_zero,
    //            self.compute_zero_flag(result)?,
    //            self.load_flag(ExtendedRegister::ZF)?,
    //            "fff",
    //        )?
    //        .into_int_value();
    //
    //    let pf = builder
    //        .build_select(
    //            is_not_zero,
    //            self.compute_parity_flag(result)?,
    //            self.load_flag(ExtendedRegister::PF)?,
    //            "",
    //        )?
    //        .into_int_value();
    //
    //    self.store_cpu_flag(ExtendedRegister::CF, cf_value);
    //    self.store_cpu_flag(ExtendedRegister::OF, of);
    //    self.store_cpu_flag(ExtendedRegister::SF, sf);
    //    self.store_cpu_flag(ExtendedRegister::ZF, zf);
    //    self.store_cpu_flag(ExtendedRegister::PF, pf);
    //
    //    self.store_op(&ops[0], result)?;
    //    Ok(())
    //}

    // NOTE: reimplemented and checked
    pub(super) fn lift_sar<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let int_1_ty = self.context.bool_type();
        let ops = instr.operands();

        let [dest, count] = match instr.mnemonic {
            Mnemonic::SAR => [&ops[0], &ops[1]],
            Mnemonic::SARX => [&ops[1], &ops[2]],
            _ => unreachable!(),
        };

        let l_value = IntValue::try_from(self.load_single_op(dest, dest.size)?)?;
        let count_value = IntValue::try_from(self.load_single_op(count, dest.size)?)?;

        let l_value_ty = l_value.get_type();
        let count_value_ty = count_value.get_type();

        let bit_width: u64 = l_value_ty.get_bit_width().into();
        let mask_c: u64 = if bit_width == 64 { 0x3f } else { 0x1f };

        let mut clamped_count = builder.build_and(
            count_value,
            count_value_ty.const_int(mask_c, false),
            "sarclamp",
        )?;
        let clamped_count_ty = clamped_count.get_type();

        let zero = clamped_count_ty.const_zero();
        let max_shift = clamped_count_ty.const_int(bit_width - 1, false);

        let is_zeroed =
            builder.build_int_compare(IntPredicate::UGT, clamped_count, max_shift, "")?;

        clamped_count = builder
            .build_select(is_zeroed, max_shift, clamped_count, "")?
            .into_int_value();

        let result = builder.build_right_shift(l_value, clamped_count, false, "")?;

        let last_shift = builder.build_right_shift(
            l_value,
            builder.build_int_sub(clamped_count, clamped_count_ty.const_int(1, false), "")?,
            false,
            "",
        )?;

        let mut cf_value = builder.build_int_truncate(last_shift, int_1_ty, "")?;
        let is_count_zero = builder.build_int_compare(
            IntPredicate::EQ,
            clamped_count,
            clamped_count_ty.const_zero(),
            "",
        )?;

        let ol_cf = self.load_flag(ExtendedRegisterEnum::CF)?;

        cf_value = builder
            .build_select(is_count_zero, ol_cf, cf_value, "cf_value")?
            .into_int_value();

        let of = int_1_ty.const_zero();

        let is_not_zero = builder.build_int_compare(IntPredicate::NE, clamped_count, zero, "")?;
        let old_sf = self.load_flag(ExtendedRegisterEnum::SF)?;
        let old_zf = self.load_flag(ExtendedRegisterEnum::ZF)?;
        let old_pf = self.load_flag(ExtendedRegisterEnum::PF)?;

        if instr.mnemonic != Mnemonic::SARX {
            let pf = builder
                .build_select(is_not_zero, self.compute_parity_flag(result)?, old_pf, "")?
                .into_int_value();
            let sf = builder
                .build_select(is_not_zero, self.compute_sign_flag(result)?, old_sf, "")?
                .into_int_value();
            let zf = builder
                .build_select(is_not_zero, self.compute_zero_flag(result)?, old_zf, "")?
                .into_int_value();

            self.store_cpu_flag(ExtendedRegisterEnum::CF, cf_value);
            self.store_cpu_flag(ExtendedRegisterEnum::OF, of);
            self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
            self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
            self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);
        }

        self.store_op(dest, result)?;
        Ok(())
    }

    // NOTE: Checked
    // TODO: Mergen might be fixed due to not read variables, so in future, check
    pub(super) fn lift_shl<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let int_1_ty = self.context.custom_width_int_type(1);

        let [dest, count] = match instr.mnemonic {
            Mnemonic::SHL => [&ops[0], &ops[1]],
            Mnemonic::SHLX => [&ops[1], &ops[2]],
            _ => unreachable!(),
        };

        let l_value = IntValue::try_from(self.load_single_op(dest, dest.size)?)?;
        let count_value = IntValue::try_from(self.load_single_op(count, dest.size)?)?;

        //let [l_value, count_value] = self.load_two_first_ints(ops)?;
        let l_value_ty = l_value.get_type();

        let count_value_ty = count_value.get_type();
        let bit_width: u64 = l_value_ty.get_bit_width().into();
        let mask_c: u64 = if bit_width == 64 { 0x3f } else { 0x1f };

        let bit_width_value = count_value_ty.const_int(bit_width, false);

        let clamped_count_value = builder.build_and(
            count_value,
            count_value_ty.const_int(mask_c, false),
            "shlclamp",
        )?;

        let mut result = builder.build_left_shift(l_value, clamped_count_value, "shl-shift")?;
        let zero = count_value_ty.const_zero();
        let is_zeroed = builder.build_int_compare(
            IntPredicate::UGT,
            clamped_count_value,
            clamped_count_value
                .get_type()
                .const_int(bit_width - 1, false),
            "",
        )?;

        result = builder
            .build_select(is_zeroed, zero, result, "")?
            .into_int_value();

        let mut cf_value = builder.build_right_shift(
            l_value,
            builder.build_int_sub(bit_width_value, clamped_count_value, "")?,
            false,
            "",
        )?;
        let one = cf_value.get_type().const_int(1, false);

        cf_value = builder.build_and(cf_value, one, "shlcf")?;
        cf_value = self.create_z_ext_or_trunc(cf_value, int_1_ty)?;

        let count_is_not_zero = builder.build_int_compare(
            IntPredicate::NE,
            clamped_count_value,
            clamped_count_value.get_type().const_zero(),
            "",
        )?;

        let cf_rvalue = builder.build_int_sub(
            clamped_count_value,
            clamped_count_value.get_type().const_int(1, false),
            "",
        )?;
        let cf_shl = builder.build_left_shift(l_value, cf_rvalue, "")?;
        let cf_int_t = cf_shl.get_type();
        let cf_right_count = cf_int_t.const_int((cf_int_t.get_bit_width() - 1).into(), false);
        let cf_low = builder.build_left_shift(cf_shl, cf_right_count, "")?;
        cf_value = builder
            .build_select(
                count_is_not_zero,
                self.create_z_ext_or_trunc(cf_low, int_1_ty)?,
                self.load_flag(ExtendedRegisterEnum::CF)?,
                "",
            )?
            .into_int_value();

        cf_value = builder
            .build_select(
                is_zeroed,
                self.create_z_ext_or_trunc(zero, int_1_ty)?,
                cf_value,
                "",
            )?
            .into_int_value();

        let is_count_one = builder.build_int_compare(
            IntPredicate::EQ,
            clamped_count_value,
            clamped_count_value.get_type().const_int(1, false),
            "",
        )?;

        let mut original_msb = builder.build_right_shift(
            l_value,
            l_value_ty.const_int(bit_width - 1, false),
            false,
            "",
        )?;
        original_msb =
            builder.build_and(original_msb, l_value.get_type().const_int(1, false), "")?;
        original_msb = self.create_z_ext_or_trunc(original_msb, int_1_ty)?;

        let cf_as_msb = self.create_z_ext_or_trunc(
            builder.build_right_shift(
                l_value,
                l_value_ty.const_int(bit_width - 1, false),
                false,
                "",
            )?,
            int_1_ty,
        )?;

        let result_msb = self.create_z_ext_or_trunc(
            builder.build_right_shift(
                result,
                result.get_type().const_int(bit_width - 1, false),
                false,
                "",
            )?,
            int_1_ty,
        )?;

        let of_value = builder
            .build_select(
                is_count_one,
                builder.build_xor(result_msb, cf_as_msb, "")?,
                self.load_flag(ExtendedRegisterEnum::OF)?,
                "",
            )?
            .into_int_value();

        // TODO: re-check
        if instr.mnemonic != Mnemonic::SHLX {
            self.store_cpu_flag(ExtendedRegisterEnum::CF, cf_value);
            self.store_cpu_flag(ExtendedRegisterEnum::OF, of_value);

            let sf = builder
                .build_select(
                    count_is_not_zero,
                    self.compute_sign_flag(result)?,
                    self.load_flag(ExtendedRegisterEnum::SF)?,
                    "",
                )?
                .into_int_value();

            self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
            // TODO: in mergen its lazy flag
            let old_zf = self.load_flag(ExtendedRegisterEnum::ZF)?;
            let zf = builder
                .build_select(
                    count_is_not_zero,
                    self.compute_zero_flag(result)?,
                    old_zf,
                    "",
                )?
                .into_int_value();

            let old_pf = self.load_flag(ExtendedRegisterEnum::PF)?;
            let pf = builder
                .build_select(
                    count_is_not_zero,
                    self.compute_parity_flag(result)?,
                    old_pf,
                    "",
                )?
                .into_int_value();

            self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
            self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);
        }
        self.store_op(dest, result)?;

        Ok(())
    }

    // NOTE: checked
    pub(super) fn lift_shld<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let dest = &ops[0];
        let source = &ops[1];
        let count = &ops[2];

        let l_value = IntValue::try_from(self.load_single_op(dest, dest.size)?)?;
        let source_value = IntValue::try_from(self.load_single_op(source, dest.size)?)?;
        let count_value = IntValue::try_from(self.load_single_op(count, dest.size)?)?;

        //let [l_value, count_value] = self.load_two_first_ints(ops)?;
        let l_value_ty = l_value.get_type();
        let count_value_ty = count_value.get_type();

        let bit_width = l_value_ty.get_bit_width().into();
        let effective_count_value = builder.build_int_unsigned_rem(
            count_value,
            count_value_ty.const_int(bit_width, false),
            "",
        )?;
        let effective_count_value_ty = effective_count_value.get_type();

        let shifted_dest =
            builder.build_left_shift(l_value, effective_count_value, "shiftedDest")?;
        let complement_count = builder.build_int_sub(
            count_value_ty.const_int(bit_width, false),
            effective_count_value,
            "complementCount",
        )?;

        let shifted_source =
            builder.build_right_shift(source_value, complement_count, false, "shiftedSource")?;

        let mut result_value = builder.build_or(shifted_dest, shifted_source, "shldResult")?;

        let count_is_not_zero = builder.build_int_compare(
            IntPredicate::NE,
            effective_count_value,
            effective_count_value_ty.const_zero(),
            "",
        )?;

        let last_shifted_bit_position = builder.build_int_sub(
            effective_count_value,
            effective_count_value_ty.const_int(1, false),
            "",
        )?;

        let last_shifted_bit = builder.build_and(
            builder.build_right_shift(l_value, last_shifted_bit_position, false, "")?,
            l_value_ty.const_int(1, false),
            "",
        )?;

        let cf = builder
            .build_select(
                count_is_not_zero,
                self.create_z_ext_or_trunc(last_shifted_bit, self.context.bool_type())?,
                self.load_flag(ExtendedRegisterEnum::CF)?,
                "",
            )?
            .into_int_value();

        result_value = builder
            .build_select(count_is_not_zero, result_value, l_value, "")?
            .into_int_value();

        let is_one = builder.build_int_compare(
            IntPredicate::EQ,
            effective_count_value,
            effective_count_value_ty.const_int(1, false),
            "",
        )?;

        let new_of = builder.build_xor(
            builder.build_right_shift(
                l_value,
                l_value_ty.const_int(bit_width - 1, false),
                false,
                "sub_of",
            )?,
            builder.build_right_shift(
                result_value,
                result_value.get_type().const_int(bit_width - 1, false),
                false,
                "",
            )?,
            "",
        )?;

        let of = builder
            .build_select(
                is_one,
                self.create_z_ext_or_trunc(new_of, self.context.bool_type())?,
                self.load_flag(ExtendedRegisterEnum::OF)?,
                "",
            )?
            .into_int_value();

        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);

        self.store_cpu_flag(
            ExtendedRegisterEnum::SF,
            self.compute_sign_flag(result_value)?,
        );
        self.store_cpu_flag(
            ExtendedRegisterEnum::ZF,
            self.compute_zero_flag(result_value)?,
        );
        self.store_cpu_flag(
            ExtendedRegisterEnum::PF,
            self.compute_parity_flag(result_value)?,
        );

        self.store_op(dest, result_value)?;

        Ok(())
    }

    // NOTE: checked
    pub(super) fn lift_shr<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let mnemonic = &instr.mnemonic;

        let [dest, count] = match mnemonic {
            Mnemonic::SHR => [&ops[0], &ops[1]],
            Mnemonic::SHRX => [&ops[1], &ops[2]],
            _ => unreachable!(),
        };

        let l_value = IntValue::try_from(self.load_single_op(dest, dest.size)?)?;
        let count_value = IntValue::try_from(self.load_single_op(count, dest.size)?)?;

        let l_value_ty = l_value.get_type();
        let count_value_ty = count_value.get_type();

        let bit_width: u64 = l_value_ty.get_bit_width().into();
        let mask_c: u64 = if bit_width == 64 { 0x3f } else { 0x1f };

        let clamped_count =
            builder.build_and(count_value, count_value_ty.const_int(mask_c, false), "")?;
        let mut result = builder.build_right_shift(l_value, clamped_count, false, "")?;
        let zero = count_value_ty.const_zero();
        let is_zeroed = builder.build_int_compare(
            IntPredicate::UGT,
            clamped_count,
            clamped_count.get_type().const_int(bit_width - 1, false),
            "",
        )?;

        result = builder
            .build_select(is_zeroed, zero, result, "")?
            .into_int_value();

        let mut cf_value = builder.build_int_truncate(
            builder.build_right_shift(
                l_value,
                builder.build_int_sub(
                    clamped_count,
                    clamped_count.get_type().const_int(1, false),
                    "",
                )?,
                false,
                "",
            )?,
            self.context.bool_type(),
            "",
        )?;

        let is_count_one = builder.build_int_compare(
            IntPredicate::EQ,
            clamped_count,
            clamped_count.get_type().const_int(1, false),
            "",
        )?;

        let mut of =
            builder.build_int_compare(IntPredicate::EQ, l_value, l_value_ty.const_zero(), "")?;

        of = builder
            .build_select(
                is_count_one,
                of,
                self.load_flag(ExtendedRegisterEnum::OF)?,
                "",
            )?
            .into_int_value();

        let is_not_zero = builder.build_int_compare(IntPredicate::NE, clamped_count, zero, "")?;
        let old_cf = self.load_flag(ExtendedRegisterEnum::CF)?;

        cf_value = builder
            .build_select(is_not_zero, cf_value, old_cf, "cf_value_1_")?
            .into_int_value();

        cf_value = builder
            .build_select(
                is_zeroed,
                builder.build_int_truncate(zero, self.context.bool_type(), "")?,
                cf_value,
                "",
            )?
            .into_int_value();

        let sf = builder
            .build_select(
                is_not_zero,
                self.compute_sign_flag(result)?,
                self.load_flag(ExtendedRegisterEnum::SF)?,
                "",
            )?
            .into_int_value();

        let zf = builder
            .build_select(
                is_not_zero,
                self.compute_zero_flag(result)?,
                self.load_flag(ExtendedRegisterEnum::ZF)?,
                "",
            )?
            .into_int_value();

        let pf = builder
            .build_select(
                is_not_zero,
                self.compute_parity_flag(result)?,
                self.load_flag(ExtendedRegisterEnum::PF)?,
                "",
            )?
            .into_int_value();

        if mnemonic != &Mnemonic::SHRX {
            self.store_cpu_flag(ExtendedRegisterEnum::CF, cf_value);
            self.store_cpu_flag(ExtendedRegisterEnum::OF, of);
            self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
            self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);
            self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        }
        self.store_op(dest, result)?;

        Ok(())
    }

    // NOTE: checked
    pub(super) fn lift_shrd<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let int_1_ty = self.context.bool_type();

        let dest = &ops[0];
        let source = &ops[1];
        let count = &ops[2];

        let l_value = IntValue::try_from(self.load_single_op(dest, dest.size)?)?;
        let source_value = IntValue::try_from(self.load_single_op(source, dest.size)?)?;
        let count_value = IntValue::try_from(self.load_single_op(count, dest.size)?)?;

        let l_value_ty = l_value.get_type();
        let count_value_ty = count_value.get_type();

        let bit_width = l_value_ty.get_bit_width().into();
        let effective_count_value = builder.build_int_unsigned_rem(
            count_value,
            count_value_ty.const_int(bit_width, false),
            "effectiveShiftCount",
        )?;
        let effective_count_value_ty = effective_count_value.get_type();

        let shifted_dest =
            builder.build_right_shift(l_value, effective_count_value, false, "shiftedDest")?;
        let complement_count = builder.build_int_sub(
            count_value_ty.const_int(bit_width, false),
            effective_count_value,
            "complementCount",
        )?;

        let shifted_source =
            builder.build_left_shift(source_value, complement_count, "shiftedSource")?;
        let result_value = builder.build_or(shifted_dest, shifted_source, "shrdResult")?;
        let result_value_ty = result_value.get_type();

        let cf_bit_position = builder.build_int_sub(
            effective_count_value,
            effective_count_value_ty.const_int(1, false),
            "",
        )?;
        let mut cf = builder.build_right_shift(l_value, cf_bit_position, false, "")?;
        cf = builder.build_and(cf, cf.get_type().const_int(1, false), "")?;
        cf = self.create_z_ext_or_trunc(cf, self.context.bool_type())?;

        let is_count_one = builder.build_int_compare(
            IntPredicate::EQ,
            effective_count_value,
            effective_count_value_ty.const_int(1, false),
            "",
        )?;
        let mut most_significant_bits_of_dest = builder.build_right_shift(
            l_value,
            l_value_ty.const_int(bit_width - 1, false),
            false,
            "name",
        )?;
        let most_significant_bits_of_dest_ty = most_significant_bits_of_dest.get_type();
        most_significant_bits_of_dest = builder.build_and(
            most_significant_bits_of_dest,
            most_significant_bits_of_dest_ty.const_int(1, false),
            "shrdmsb2",
        )?;
        let mut most_significant_bits_of_result = builder.build_right_shift(
            result_value,
            result_value_ty.const_int(bit_width - 1, false),
            false,
            "shlmsbresult",
        )?;
        let most_significant_bits_of_result_ty = most_significant_bits_of_result.get_type();
        most_significant_bits_of_result = builder.build_and(
            most_significant_bits_of_result,
            most_significant_bits_of_result_ty.const_int(1, false),
            "shrdmsb2",
        )?;

        let mut of = builder.build_xor(
            most_significant_bits_of_dest,
            most_significant_bits_of_result,
            "",
        )?;
        of = self.create_z_ext_or_trunc(of, int_1_ty)?;
        of = builder
            .build_select(is_count_one, of, int_1_ty.const_zero(), "")?
            .into_int_value();
        of = builder.build_int_z_extend(of, int_1_ty, "")?;

        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);

        self.store_cpu_flag(
            ExtendedRegisterEnum::PF,
            self.compute_parity_flag(result_value)?,
        );
        self.store_cpu_flag(
            ExtendedRegisterEnum::SF,
            self.compute_sign_flag(result_value)?,
        );
        self.store_cpu_flag(
            ExtendedRegisterEnum::ZF,
            self.compute_zero_flag(result_value)?,
        );

        self.store_op(dest, result_value)?;

        Ok(())
    }
}
