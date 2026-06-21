use super::Result;
use crate::lifter::{Error, LifterX86};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{IntDyn, IntType, IntValue};
use zydis::{Instruction, Mnemonic, Operands};

struct ShiftLegacyCount<'ctx> {
    count: IntValue<'ctx, IntDyn>,
    shift_count: IntValue<'ctx, IntDyn>,
    count_nonzero: IntValue<'ctx, bool>,
    count_is_one: IntValue<'ctx, bool>,
    too_large: IntValue<'ctx, bool>,
}

struct ShiftBmiCount<'ctx> {
    shift_count: IntValue<'ctx, IntDyn>,
}

struct ShiftDoubleCount<'ctx> {
    safe_count: IntValue<'ctx, IntDyn>,
    count_nonzero: IntValue<'ctx, bool>,
    count_is_one: IntValue<'ctx, bool>,
}

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_sar<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let (dest, value_op, count_op, updates_flags) = match instr.mnemonic {
            Mnemonic::SAR => (&operands[0], &operands[0], &operands[1], true),
            Mnemonic::SARX => (&operands[0], &operands[1], &operands[2], false),
            _ => {
                return Err(Error::UnsupportedInstr(
                    "unsupported arithmetic shift instruction",
                ))
            }
        };

        let value = self.load_single_int_op(value_op, dest.size)?;
        let count = self.load_single_int_op(count_op, dest.size)?;
        let result = if updates_flags {
            let info = self.shift_legacy_count_info(value, count)?;
            let result = self.builder()?.build_int_ashr::<IntDyn, _, _, _>(
                value,
                info.shift_count,
                "sar",
            )?;

            let bit_width = value.ty().bit_width();
            let cf_count = self.shift_legacy_cf_count(&info, bit_width)?;
            let cf_pos = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
                cf_count,
                cf_count.ty().const_int_raw(1, false)?,
                "sar_cf_pos",
            )?;
            let cf_new = self.shift_low_bit_after_lshr(value, cf_pos)?;
            let cf = self.builder()?.build_select(
                info.count_nonzero,
                cf_new,
                self.load_flag(ExtendedRegisterEnum::CF)?,
                "sar_cf",
            )?;
            let of = self.builder()?.build_select(
                info.count_is_one,
                self.shift_bool_value(false)?,
                self.load_flag(ExtendedRegisterEnum::OF)?,
                "sar_of",
            )?;

            self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
            self.store_cpu_flag(ExtendedRegisterEnum::OF, of)?;
            self.shift_store_status_flags(result, info.count_nonzero)?;
            result
        } else {
            let info = self.shift_bmi_count_info(value, count)?;
            self.builder()?
                .build_int_ashr::<IntDyn, _, _, _>(value, info.shift_count, "sarx")?
        };

        self.store_op(dest, result)
    }

    pub(super) fn lift_shl<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let (dest, value_op, count_op, updates_flags) = match instr.mnemonic {
            Mnemonic::SHL => (&operands[0], &operands[0], &operands[1], true),
            Mnemonic::SHLX => (&operands[0], &operands[1], &operands[2], false),
            _ => {
                return Err(Error::UnsupportedInstr(
                    "unsupported left shift instruction",
                ))
            }
        };

        let value = self.load_single_int_op(value_op, dest.size)?;
        let count = self.load_single_int_op(count_op, dest.size)?;
        let result = if updates_flags {
            let info = self.shift_legacy_count_info(value, count)?;
            let shifted =
                self.builder()?
                    .build_int_shl::<IntDyn, _, _, _>(value, info.shift_count, "shl")?;
            let zero_value: IntValue<'ctx, IntDyn> =
                value.ty().const_zero().as_value().try_into()?;
            let result =
                self.builder()?
                    .build_select(info.too_large, zero_value, shifted, "shl_result")?;

            let bit_width = value.ty().bit_width();
            let cf_count = self.shift_legacy_cf_count(&info, bit_width)?;
            let width_value = self.shift_const_value(value.ty(), u64::from(bit_width))?;
            let cf_pos = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
                width_value,
                cf_count,
                "shl_cf_pos",
            )?;
            let cf_new = self.shift_low_bit_after_lshr(value, cf_pos)?;
            let cf_if_shifted = self.builder()?.build_select(
                info.count_nonzero,
                cf_new,
                self.load_flag(ExtendedRegisterEnum::CF)?,
                "shl_cf_shifted",
            )?;
            let cf = self.builder()?.build_select(
                info.too_large,
                self.shift_bool_value(false)?,
                cf_if_shifted,
                "shl_cf",
            )?;
            let of_one = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
                self.shift_msb_flag(result)?,
                cf_new,
                "shl_of_one",
            )?;
            let of = self.builder()?.build_select(
                info.count_is_one,
                of_one,
                self.load_flag(ExtendedRegisterEnum::OF)?,
                "shl_of",
            )?;

            self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
            self.store_cpu_flag(ExtendedRegisterEnum::OF, of)?;
            self.shift_store_status_flags(result, info.count_nonzero)?;
            result
        } else {
            let info = self.shift_bmi_count_info(value, count)?;
            self.builder()?
                .build_int_shl::<IntDyn, _, _, _>(value, info.shift_count, "shlx")?
        };

        self.store_op(dest, result)
    }

    pub(super) fn lift_shld<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        if instr.mnemonic != Mnemonic::SHLD {
            return Err(Error::UnsupportedInstr(
                "unsupported double-left shift instruction",
            ));
        }
        let dest = &operands[0];
        let source = &operands[1];
        let count = &operands[2];
        let value = self.load_single_int_op(dest, dest.size)?;
        let source_value = self.load_single_int_op(source, dest.size)?;
        let count_value = self.load_single_int_op(count, dest.size)?;
        let info = self.shift_double_count_info(value, count_value)?;
        let bit_width = value.ty().bit_width();
        let width_value = self.shift_const_value(value.ty(), u64::from(bit_width))?;
        let complement = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
            width_value,
            info.safe_count,
            "shld_complement",
        )?;

        let shifted_dest = self.builder()?.build_int_shl::<IntDyn, _, _, _>(
            value,
            info.safe_count,
            "shld_dest",
        )?;
        let shifted_source = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            source_value,
            complement,
            "shld_source",
        )?;
        let combined = self.builder()?.build_int_or::<IntDyn, _, _, _>(
            shifted_dest,
            shifted_source,
            "shld_combined",
        )?;
        let result =
            self.builder()?
                .build_select(info.count_nonzero, combined, value, "shld_result")?;

        let cf_pos = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
            width_value,
            info.safe_count,
            "shld_cf_pos",
        )?;
        let cf_new = self.shift_low_bit_after_lshr(value, cf_pos)?;
        let cf = self.builder()?.build_select(
            info.count_nonzero,
            cf_new,
            self.load_flag(ExtendedRegisterEnum::CF)?,
            "shld_cf",
        )?;
        let of_new = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
            self.shift_msb_flag(value)?,
            self.shift_msb_flag(result)?,
            "shld_of_new",
        )?;
        let of = self.builder()?.build_select(
            info.count_is_one,
            of_new,
            self.load_flag(ExtendedRegisterEnum::OF)?,
            "shld_of",
        )?;

        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of)?;
        self.shift_store_status_flags(result, info.count_nonzero)?;
        self.store_op(dest, result)
    }

    pub(super) fn lift_shr<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let (dest, value_op, count_op, updates_flags) = match instr.mnemonic {
            Mnemonic::SHR => (&operands[0], &operands[0], &operands[1], true),
            Mnemonic::SHRX => (&operands[0], &operands[1], &operands[2], false),
            _ => {
                return Err(Error::UnsupportedInstr(
                    "unsupported right shift instruction",
                ))
            }
        };

        let value = self.load_single_int_op(value_op, dest.size)?;
        let count = self.load_single_int_op(count_op, dest.size)?;
        let result = if updates_flags {
            let info = self.shift_legacy_count_info(value, count)?;
            let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
                value,
                info.shift_count,
                "shr",
            )?;
            let zero_value: IntValue<'ctx, IntDyn> =
                value.ty().const_zero().as_value().try_into()?;
            let result =
                self.builder()?
                    .build_select(info.too_large, zero_value, shifted, "shr_result")?;

            let bit_width = value.ty().bit_width();
            let cf_count = self.shift_legacy_cf_count(&info, bit_width)?;
            let cf_pos = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
                cf_count,
                cf_count.ty().const_int_raw(1, false)?,
                "shr_cf_pos",
            )?;
            let cf_new = self.shift_low_bit_after_lshr(value, cf_pos)?;
            let cf_if_shifted = self.builder()?.build_select(
                info.count_nonzero,
                cf_new,
                self.load_flag(ExtendedRegisterEnum::CF)?,
                "shr_cf_shifted",
            )?;
            let cf = self.builder()?.build_select(
                info.too_large,
                self.shift_bool_value(false)?,
                cf_if_shifted,
                "shr_cf",
            )?;
            let of = self.builder()?.build_select(
                info.count_is_one,
                self.shift_msb_flag(value)?,
                self.load_flag(ExtendedRegisterEnum::OF)?,
                "shr_of",
            )?;

            self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
            self.store_cpu_flag(ExtendedRegisterEnum::OF, of)?;
            self.shift_store_status_flags(result, info.count_nonzero)?;
            result
        } else {
            let info = self.shift_bmi_count_info(value, count)?;
            self.builder()?
                .build_int_lshr::<IntDyn, _, _, _>(value, info.shift_count, "shrx")?
        };

        self.store_op(dest, result)
    }

    pub(super) fn lift_shrd<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        if instr.mnemonic != Mnemonic::SHRD {
            return Err(Error::UnsupportedInstr(
                "unsupported double-right shift instruction",
            ));
        }
        let dest = &operands[0];
        let source = &operands[1];
        let count = &operands[2];
        let value = self.load_single_int_op(dest, dest.size)?;
        let source_value = self.load_single_int_op(source, dest.size)?;
        let count_value = self.load_single_int_op(count, dest.size)?;
        let info = self.shift_double_count_info(value, count_value)?;
        let bit_width = value.ty().bit_width();
        let width_value = self.shift_const_value(value.ty(), u64::from(bit_width))?;
        let complement = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
            width_value,
            info.safe_count,
            "shrd_complement",
        )?;

        let shifted_dest = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            value,
            info.safe_count,
            "shrd_dest",
        )?;
        let shifted_source = self.builder()?.build_int_shl::<IntDyn, _, _, _>(
            source_value,
            complement,
            "shrd_source",
        )?;
        let combined = self.builder()?.build_int_or::<IntDyn, _, _, _>(
            shifted_dest,
            shifted_source,
            "shrd_combined",
        )?;
        let result =
            self.builder()?
                .build_select(info.count_nonzero, combined, value, "shrd_result")?;

        let cf_pos = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
            info.safe_count,
            info.safe_count.ty().const_int_raw(1, false)?,
            "shrd_cf_pos",
        )?;
        let cf_new = self.shift_low_bit_after_lshr(value, cf_pos)?;
        let cf = self.builder()?.build_select(
            info.count_nonzero,
            cf_new,
            self.load_flag(ExtendedRegisterEnum::CF)?,
            "shrd_cf",
        )?;
        let of_new = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
            self.shift_msb_flag(value)?,
            self.shift_msb_flag(result)?,
            "shrd_of_new",
        )?;
        let of = self.builder()?.build_select(
            info.count_is_one,
            of_new,
            self.load_flag(ExtendedRegisterEnum::OF)?,
            "shrd_of",
        )?;

        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of)?;
        self.shift_store_status_flags(result, info.count_nonzero)?;
        self.store_op(dest, result)
    }

    fn shift_legacy_count_info(
        &self,
        value: IntValue<'ctx, IntDyn>,
        count: IntValue<'ctx, IntDyn>,
    ) -> Result<ShiftLegacyCount<'ctx>> {
        let bit_width = value.ty().bit_width();
        let mask = if bit_width == 64 { 0x3f } else { 0x1f };
        let count_ty = count.ty();
        let masked = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            count,
            count_ty.const_int_raw(mask, false)?,
            "shift_count_masked",
        )?;
        let count_nonzero = self.builder()?.build_icmp_ne::<IntDyn, _, _, _>(
            masked,
            count_ty.const_zero(),
            "shift_count_nonzero",
        )?;
        let count_is_one = self.builder()?.build_icmp_eq::<IntDyn, _, _, _>(
            masked,
            count_ty.const_int_raw(1, false)?,
            "shift_count_one",
        )?;
        let max_shift = count_ty.const_int_raw(u64::from(bit_width - 1), false)?;
        let too_large = self.builder()?.build_icmp_ugt::<IntDyn, _, _, _>(
            masked,
            max_shift,
            "shift_count_too_large",
        )?;
        let max_shift_value = self.shift_const_value(count_ty, u64::from(bit_width - 1))?;
        let shift_count =
            self.builder()?
                .build_select(too_large, max_shift_value, masked, "shift_count")?;
        Ok(ShiftLegacyCount {
            count: masked,
            shift_count,
            count_nonzero,
            count_is_one,
            too_large,
        })
    }

    fn shift_bmi_count_info(
        &self,
        value: IntValue<'ctx, IntDyn>,
        count: IntValue<'ctx, IntDyn>,
    ) -> Result<ShiftBmiCount<'ctx>> {
        let bit_width = value.ty().bit_width();
        let mask = if bit_width == 64 { 0x3f } else { 0x1f };
        let count_ty = count.ty();
        let shift_count = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            count,
            count_ty.const_int_raw(mask, false)?,
            "bmi_shift_count",
        )?;
        Ok(ShiftBmiCount { shift_count })
    }

    fn shift_double_count_info(
        &self,
        value: IntValue<'ctx, IntDyn>,
        count: IntValue<'ctx, IntDyn>,
    ) -> Result<ShiftDoubleCount<'ctx>> {
        let bit_width = value.ty().bit_width();
        let mask = if bit_width == 64 { 0x3f } else { 0x1f };
        let count_ty = count.ty();
        let masked = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            count,
            count_ty.const_int_raw(mask, false)?,
            "double_shift_count_masked",
        )?;
        let effective = self.builder()?.build_int_urem::<IntDyn, _, _, _>(
            masked,
            count_ty.const_int_raw(u64::from(bit_width), false)?,
            "double_shift_count",
        )?;
        let count_nonzero = self.builder()?.build_icmp_ne::<IntDyn, _, _, _>(
            effective,
            count_ty.const_zero(),
            "double_shift_nonzero",
        )?;
        let count_is_one = self.builder()?.build_icmp_eq::<IntDyn, _, _, _>(
            effective,
            count_ty.const_int_raw(1, false)?,
            "double_shift_one",
        )?;
        let one_value = self.shift_const_value(count_ty, 1)?;
        let safe_count = self.builder()?.build_select(
            count_nonzero,
            effective,
            one_value,
            "double_shift_safe_count",
        )?;
        Ok(ShiftDoubleCount {
            safe_count,
            count_nonzero,
            count_is_one,
        })
    }

    fn shift_legacy_cf_count(
        &self,
        info: &ShiftLegacyCount<'ctx>,
        bit_width: u32,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let count_ty = info.count.ty();
        let one = self.shift_const_value(count_ty, 1)?;
        let width = self.shift_const_value(count_ty, u64::from(bit_width))?;
        let limited = self.builder()?.build_select(
            info.too_large,
            width,
            info.count,
            "shift_cf_count_limited",
        )?;
        Ok(self
            .builder()?
            .build_select(info.count_nonzero, limited, one, "shift_cf_count")?)
    }

    fn shift_store_status_flags(
        &mut self,
        result: IntValue<'ctx, IntDyn>,
        update: IntValue<'ctx, bool>,
    ) -> Result<()> {
        let sf = self.builder()?.build_select(
            update,
            self.shift_sign_flag(result)?,
            self.load_flag(ExtendedRegisterEnum::SF)?,
            "shift_sf",
        )?;
        let zf = self.builder()?.build_select(
            update,
            self.shift_zero_flag(result)?,
            self.load_flag(ExtendedRegisterEnum::ZF)?,
            "shift_zf",
        )?;
        let pf = self.builder()?.build_select(
            update,
            self.shift_parity_flag(result)?,
            self.load_flag(ExtendedRegisterEnum::PF)?,
            "shift_pf",
        )?;
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf)
    }

    fn shift_sign_flag(&self, value: IntValue<'ctx, IntDyn>) -> Result<IntValue<'ctx, IntDyn>> {
        Ok(self
            .builder()?
            .build_icmp_slt::<IntDyn, _, _, _>(value, value.ty().const_zero(), "shift_sf_new")?
            .as_dyn())
    }

    fn shift_zero_flag(&self, value: IntValue<'ctx, IntDyn>) -> Result<IntValue<'ctx, IntDyn>> {
        Ok(self
            .builder()?
            .build_icmp_eq::<IntDyn, _, _, _>(value, value.ty().const_zero(), "shift_zf_new")?
            .as_dyn())
    }

    fn shift_parity_flag(&self, value: IntValue<'ctx, IntDyn>) -> Result<IntValue<'ctx, IntDyn>> {
        let mut folded = self.create_z_ext_or_trunc(value, self.module.i8_type().as_dyn())?;
        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            folded,
            folded.ty().const_int_raw(4, false)?,
            "shift_pf4",
        )?;
        folded = self
            .builder()?
            .build_int_xor::<IntDyn, _, _, _>(folded, shifted, "shift_pfx4")?;
        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            folded,
            folded.ty().const_int_raw(2, false)?,
            "shift_pf2",
        )?;
        folded = self
            .builder()?
            .build_int_xor::<IntDyn, _, _, _>(folded, shifted, "shift_pfx2")?;
        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            folded,
            folded.ty().const_int_raw(1, false)?,
            "shift_pf1",
        )?;
        folded = self
            .builder()?
            .build_int_xor::<IntDyn, _, _, _>(folded, shifted, "shift_pfx1")?;
        let low_bit = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            folded,
            folded.ty().const_int_raw(1, false)?,
            "shift_pf_bit",
        )?;
        Ok(self
            .builder()?
            .build_icmp_eq::<IntDyn, _, _, _>(low_bit, low_bit.ty().const_zero(), "shift_pf")?
            .as_dyn())
    }

    fn shift_msb_flag(&self, value: IntValue<'ctx, IntDyn>) -> Result<IntValue<'ctx, IntDyn>> {
        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            value,
            value
                .ty()
                .const_int_raw(u64::from(value.ty().bit_width() - 1), false)?,
            "shift_msb",
        )?;
        let bit = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            shifted,
            shifted.ty().const_int_raw(1, false)?,
            "shift_msb_bit",
        )?;
        self.create_z_ext_or_trunc(bit, self.module.bool_type().as_dyn())
    }

    fn shift_low_bit_after_lshr(
        &self,
        value: IntValue<'ctx, IntDyn>,
        shift: IntValue<'ctx, IntDyn>,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let shifted =
            self.builder()?
                .build_int_lshr::<IntDyn, _, _, _>(value, shift, "shift_bit_source")?;
        let bit = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            shifted,
            shifted.ty().const_int_raw(1, false)?,
            "shift_bit",
        )?;
        self.create_z_ext_or_trunc(bit, self.module.bool_type().as_dyn())
    }

    fn shift_bool_value(&self, value: bool) -> Result<IntValue<'ctx, IntDyn>> {
        let value = if value {
            self.module.bool_type().const_int(true)
        } else {
            self.module.bool_type().const_zero()
        };
        Ok(value.as_value().try_into()?)
    }

    fn shift_const_value(
        &self,
        ty: IntType<'ctx, IntDyn>,
        value: u64,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        Ok(ty.const_int_raw(value, false)?.as_value().try_into()?)
    }
}
