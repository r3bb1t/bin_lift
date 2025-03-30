use super::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use inkwell::{values::IntValue, IntPredicate};
use zydis::{Instruction, Operands};

impl LifterX86<'_> {
    // NOTE: checked
    pub(super) fn lift_rcl<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let dest = &ops[0];
        let count = &ops[1];
        let dest_size: u32 = ops[0].size.into();

        let l_value = IntValue::try_from(self.load_single_op(dest, dest.size)?)?;
        let count_value = IntValue::try_from(self.load_single_op(count, dest.size)?)?;

        let count_value_ty = count_value.get_type();

        let cf = self.load_flag(ExtendedRegisterEnum::CF)?;
        let bit_width: u64 = l_value.get_type().get_bit_width().into();
        let mask_c = if bit_width == 64 { 0x3f } else { 0x1f };

        let mut actual_count = builder.build_and(
            count_value,
            count_value_ty.const_int(mask_c, false),
            "rcl_actual_count",
        )?;

        actual_count = if bit_width < 16 {
            builder.build_int_unsigned_rem(
                actual_count,
                count_value_ty.const_int(bit_width + 1, false),
                "rcl_actual_count2",
            )?
        } else {
            actual_count
        };

        let wide_type = self.context.custom_width_int_type(dest_size * 2);
        let wide_l_value = builder.build_int_z_extend(l_value, wide_type, "rcl_wide_ty_val")?;
        let cf_extended = builder.build_int_z_extend(cf, wide_type, "rcl_cf_extended")?;
        let shifted_in_cf = cf_extended;

        actual_count = builder.build_int_z_extend(actual_count, wide_type, "rcl_actual_count")?;

        let left_shifted = builder.build_left_shift(
            wide_l_value,
            builder.build_int_z_extend(actual_count, wide_type, "rcl_actual_count_extended")?,
            "rcl_left_shifted",
        )?;

        let right_shift_amount = builder.build_int_sub(
            wide_type.const_int(dest_size.into(), false),
            actual_count,
            "rcl_right_shift_amount",
        )?;

        // left shifted actually
        let right_shifted = builder.build_right_shift(
            wide_l_value,
            builder.build_int_z_extend(
                right_shift_amount,
                wide_type,
                "rcl_actual_count_extended",
            )?,
            false,
            "",
        )?;

        let rotated = {
            let rotated = builder.build_or(
                left_shifted,
                builder.build_int_z_extend(right_shifted, wide_type, "")?,
                "",
            )?;

            let rotated = builder.build_right_shift(rotated, actual_count, false, "")?;
            let rotated = builder.build_left_shift(rotated, actual_count, "")?;
            builder.build_or(rotated, shifted_in_cf, "")?
        };

        let result = self.create_z_ext_or_trunc(rotated, l_value.get_type())?;

        let int_1_ty = self.context.custom_width_int_type(1);
        let new_cf_bit_position = rotated.get_type().const_int(dest_size.into(), false);
        let new_cf = self.create_z_ext_or_trunc(
            builder.build_right_shift(rotated, new_cf_bit_position, false, "")?,
            int_1_ty,
        )?;

        let msb_after_rotate = self.create_z_ext_or_trunc(
            builder.build_right_shift(
                result,
                result.get_type().const_int(dest_size.into(), false),
                false,
                "rclmsbafterrotate",
            )?,
            int_1_ty,
        )?;

        let is_count_one = builder.build_int_compare(
            IntPredicate::EQ,
            actual_count,
            actual_count.get_type().const_int(1, false),
            "",
        )?;

        let new_of =
            self.create_z_ext_or_trunc(builder.build_xor(new_cf, msb_after_rotate, "")?, int_1_ty)?;

        let new_of = builder
            .build_select(
                is_count_one,
                new_of,
                self.load_flag(ExtendedRegisterEnum::OF)?,
                "new_of",
            )?
            .into_int_value();

        let is_count_zero = builder.build_int_compare(
            IntPredicate::EQ,
            actual_count,
            actual_count.get_type().const_zero(),
            "rcl_is_count_zero",
        )?;

        let result = builder
            .build_select(is_count_zero, l_value, result, "")?
            .into_int_value();
        let new_cf = builder
            .build_select(is_count_zero, cf, new_cf, "")?
            .into_int_value();
        let new_of = builder
            .build_select(
                is_count_zero,
                self.load_flag(ExtendedRegisterEnum::OF)?,
                new_of,
                "",
            )?
            .into_int_value();

        self.store_op(dest, result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::CF, new_cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, new_of);
        Ok(())
    }

    // NOTE: checked
    pub(super) fn lift_rcr<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ctx = self.context;

        let ops = instr.operands();
        let dest = &ops[0];
        let count = &ops[1];

        let dest_size: u32 = dest.size.into();

        //let [l_value, count_value] = self.load_two_first_ints(ops)?;
        let l_value: IntValue<'_> = self.load_single_op(dest, dest.size)?.try_into()?;
        let count_value: IntValue<'_> = self.load_single_op(count, dest.size)?.try_into()?;
        let carry_flag = self.load_flag(ExtendedRegisterEnum::CF)?;

        let l_value_ty = l_value.get_type();
        let count_value_ty = count_value.get_type();

        let count_mask = count_value_ty.const_int(if dest_size == 64 { 0x3f } else { 0x1f }, false);
        let mut actual_count = builder.build_and(count_value, count_mask, "")?;

        let bit_width = l_value_ty.const_int(dest_size.into(), false);
        let bit_width_plus_one = l_value_ty.const_int((dest_size + 1).into(), false);
        let bit_width_minus_one = l_value_ty.const_int((dest_size - 1).into(), false);
        let one = l_value_ty.const_int(1, false);
        let zero = l_value_ty.const_zero();

        actual_count = builder.build_int_unsigned_rem(actual_count, bit_width_plus_one, "")?;

        let wide_type = ctx.custom_width_int_type((dest.size * 2).into());
        let wide_l_value = builder.build_int_z_extend(l_value, wide_type, "")?;
        let wide_cf = builder.build_int_z_extend(carry_flag, wide_type, "")?;

        let shifted_cf =
            builder.build_left_shift(wide_cf, wide_type.const_int(dest_size.into(), false), "")?;

        let combined_value = builder.build_or(wide_l_value, shifted_cf, "")?;

        let right_shifted = builder.build_right_shift(
            combined_value,
            builder.build_int_z_extend(actual_count, wide_type, "")?,
            false,
            "",
        )?;

        let left_shifted = builder.build_left_shift(
            combined_value,
            builder.build_int_sub(
                builder.build_int_z_extend(bit_width_plus_one, wide_type, "")?,
                builder.build_int_z_extend(actual_count, wide_type, "")?,
                "",
            )?,
            "",
        )?;

        let rotated = builder.build_or(right_shifted, left_shifted, "")?;

        let mut result = builder.build_int_truncate(rotated, l_value_ty, "")?;

        let mut new_cf = builder.build_int_truncate(
            builder.build_right_shift(
                rotated,
                wide_type.const_int(dest_size.into(), false),
                false,
                "",
            )?,
            ctx.bool_type(),
            "",
        )?;

        let msb_pos = l_value_ty.const_int((dest_size - 1).into(), false);
        let second_msb_pos = l_value_ty.const_int((dest_size - 2).into(), false);

        let msb = self.create_z_ext_or_trunc(
            builder.build_right_shift(result, msb_pos, false, "")?,
            ctx.bool_type(),
        )?;

        let second_msb = self.create_z_ext_or_trunc(
            builder.build_right_shift(result, second_msb_pos, false, "")?,
            ctx.bool_type(),
        )?;

        let of_defined =
            self.create_z_ext_or_trunc(builder.build_xor(msb, second_msb, "")?, ctx.bool_type())?;

        let is_count_one = builder.build_int_compare(IntPredicate::EQ, actual_count, one, "")?;

        let mut new_of = builder
            .build_select(
                is_count_one,
                of_defined,
                self.load_flag(ExtendedRegisterEnum::CF)?,
                "",
            )?
            .into_int_value();

        let is_count_zero = builder.build_int_compare(IntPredicate::EQ, actual_count, zero, "")?;
        result = builder
            .build_select(is_count_zero, l_value, result, "")?
            .into_int_value();

        new_cf = builder
            .build_select(is_count_zero, carry_flag, new_cf, "")?
            .into_int_value();
        new_of = builder
            .build_select(
                is_count_zero,
                self.load_flag(ExtendedRegisterEnum::OF)?,
                new_of,
                "",
            )?
            .into_int_value();

        self.store_op(dest, result)?;
        self.store_cpu_flag(ExtendedRegisterEnum::CF, new_cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, new_of);

        Ok(())
    }

    pub(super) fn lift_rol<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let ops = instr.operands();
        let dest = &ops[0];
        let src = &ops[1];

        let int_1_ty = self.context.bool_type();

        let dest_size: u64 = dest.size.into();

        let l_value: IntValue<'_> = self.load_single_op(dest, dest.size)?.try_into()?;
        let mut r_value: IntValue<'_> = self.load_single_op(src, dest.size)?.try_into()?;

        let l_value_ty = l_value.get_type();

        let bit_width = l_value_ty.const_int(dest_size, false);
        let bit_width_plus_one = l_value_ty.const_int(dest_size + 1, false);
        let count_mask = l_value_ty.const_int(if dest_size == 64 { 0x3f } else { 0x1f }, false);

        let one = l_value_ty.const_int(1, false);
        let zero = l_value_ty.const_zero();

        let msb_pos = l_value_ty.const_int(dest_size - 1, false);
        r_value = builder.build_int_unsigned_rem(
            builder.build_int_add(r_value, count_mask, "make_r_value_")?,
            bit_width,
            "",
        )?;

        let shifted_left = builder.build_left_shift(l_value, r_value, "")?;
        let shifted_right = builder.build_right_shift(
            l_value,
            builder.build_int_sub(bit_width, r_value, "")?,
            false,
            "rol_",
        )?;
        let mut result = builder.build_or(shifted_left, shifted_right, "")?;
        let mut cf = self.create_z_ext_or_trunc(shifted_right, int_1_ty)?;

        let is_zero_bit_rotation =
            builder.build_int_compare(IntPredicate::EQ, r_value, zero, "is_zero_bit_rotation_")?;
        let old_cf = self.load_flag(ExtendedRegisterEnum::CF)?;

        cf = builder
            .build_select(is_zero_bit_rotation, old_cf, cf, "")?
            .into_int_value();

        result = builder
            .build_select(is_zero_bit_rotation, l_value, result, "result")?
            .into_int_value();

        let new_msb = builder.build_right_shift(result, msb_pos, false, "")?;
        let of1 = self.create_z_ext_or_trunc(new_msb, int_1_ty)?;

        let mut of = builder.build_xor(cf, of1, "")?;

        let is_one_bit_rotation =
            builder.build_int_compare(IntPredicate::EQ, r_value, one, "is_one_bit_rotation")?;
        let of_current = self.load_flag(ExtendedRegisterEnum::OF)?;
        of = builder
            .build_select(is_one_bit_rotation, of, of_current, "")?
            .into_int_value();
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);

        self.store_op(dest, result)?;

        Ok(())
    }

    pub(super) fn lift_ror<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let ops = instr.operands();
        let dest = &ops[0];
        let src = &ops[1];

        let int_1_ty = self.context.bool_type();

        let dest_size: u64 = dest.size.into();

        let l_value: IntValue<'_> = self.load_single_op(dest, dest.size)?.try_into()?;
        let mut r_value: IntValue<'_> = self.load_single_op(src, dest.size)?.try_into()?;

        let l_value_ty = l_value.get_type();

        let bit_width = l_value_ty.const_int(dest_size, false);
        let bit_width_plus_one = l_value_ty.const_int(dest_size + 1, false);
        let count_mask = l_value_ty.const_int(if dest_size == 64 { 0x3f } else { 0x1f }, false);

        let one = l_value_ty.const_int(1, false);
        let zero = l_value_ty.const_zero();

        let msb_pos = l_value_ty.const_int(dest_size - 1, false);
        let second_msb_pos = l_value_ty.const_int(dest_size - 2, false);
        r_value = builder.build_int_unsigned_rem(
            builder.build_int_add(r_value, count_mask, "mask_r_value_")?,
            bit_width,
            "",
        )?;

        let right_shifted = builder.build_right_shift(l_value, r_value, false, "")?;
        let left_shifted = builder.build_left_shift(
            l_value,
            builder.build_int_sub(bit_width, r_value, "")?,
            "rol_",
        )?;
        let mut result = builder.build_or(right_shifted, left_shifted, "")?;
        let msb = builder.build_right_shift(result, msb_pos, false, "")?;
        let mut cf = self.create_z_ext_or_trunc(msb, int_1_ty)?;
        let second_msb = builder.build_right_shift(result, second_msb_pos, false, "")?;

        let of_defined =
            self.create_z_ext_or_trunc(builder.build_xor(msb, second_msb, "")?, cf.get_type())?;

        let is_one_bit_rotation = builder.build_int_compare(IntPredicate::EQ, r_value, one, "")?;

        let is_zero_bit_rotation =
            builder.build_int_compare(IntPredicate::EQ, r_value, zero, "is_zero_bit_rotation_")?;
        let of_current = self.load_flag(ExtendedRegisterEnum::OF)?;
        let of = builder
            .build_select(is_one_bit_rotation, of_defined, of_current, "ror-of")?
            .into_int_value();

        cf = builder
            .build_select(
                is_zero_bit_rotation,
                self.load_flag(ExtendedRegisterEnum::CF)?,
                cf,
                "",
            )?
            .into_int_value();

        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);

        result = builder
            .build_select(is_zero_bit_rotation, l_value, result, "ror-result")?
            .into_int_value();

        self.store_op(dest, result)?;
        Ok(())
    }
}
