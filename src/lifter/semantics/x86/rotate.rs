use super::Result;
use crate::lifter::{Error, LifterX86};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{IntDyn, IntType, IntValue};
use zydis::{Instruction, Mnemonic, Operands};

struct RotateCount<'ctx> {
    safe_count: IntValue<'ctx, IntDyn>,
    count_nonzero: IntValue<'ctx, bool>,
    count_is_one: IntValue<'ctx, bool>,
}

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_rcl<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::RCL {
            return Err(Error::UnsupportedInstr(
                "unsupported rotate-through-carry-left instruction",
            ));
        }
        let operands = instr.operands();
        let dest = &operands[0];
        let count_op = &operands[1];
        let value = self.load_single_int_op(dest, dest.size)?;
        let count = self.load_single_int_op(count_op, dest.size)?;
        let info = self.rotate_carry_count_info(value, count)?;
        let bit_width = value.ty().bit_width();
        let ring_ty = self.module.custom_width_int_type(bit_width + 1)?;
        let combined = self.rotate_carry_combined_value(value, ring_ty)?;
        let safe_count = self.create_z_ext_or_trunc(info.safe_count, ring_ty)?;
        let ring_width = self.rotate_const_value(ring_ty, u64::from(bit_width + 1))?;
        let complement = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
            ring_width,
            safe_count,
            "rcl_complement",
        )?;
        let left = self
            .builder()?
            .build_int_shl::<IntDyn, _, _, _>(combined, safe_count, "rcl_left")?;
        let right =
            self.builder()?
                .build_int_lshr::<IntDyn, _, _, _>(combined, complement, "rcl_right")?;
        let rotated =
            self.builder()?
                .build_int_or::<IntDyn, _, _, _>(left, right, "rcl_rotated")?;
        let raw_result = self.create_z_ext_or_trunc(rotated, value.ty())?;
        let result =
            self.builder()?
                .build_select(info.count_nonzero, raw_result, value, "rcl_result")?;
        let cf_new = self.rotate_low_bit_after_lshr(
            rotated,
            self.rotate_const_value(ring_ty, u64::from(bit_width))?,
        )?;
        let cf = self.builder()?.build_select(
            info.count_nonzero,
            cf_new,
            self.load_flag(ExtendedRegisterEnum::CF)?,
            "rcl_cf",
        )?;
        let of_new = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
            self.rotate_msb_flag(result)?,
            cf_new,
            "rcl_of_new",
        )?;
        let of = self.builder()?.build_select(
            info.count_is_one,
            of_new,
            self.load_flag(ExtendedRegisterEnum::OF)?,
            "rcl_of",
        )?;

        self.store_op(dest, result)?;
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of)
    }

    pub(super) fn lift_rcr<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::RCR {
            return Err(Error::UnsupportedInstr(
                "unsupported rotate-through-carry-right instruction",
            ));
        }
        let operands = instr.operands();
        let dest = &operands[0];
        let count_op = &operands[1];
        let value = self.load_single_int_op(dest, dest.size)?;
        let count = self.load_single_int_op(count_op, dest.size)?;
        let info = self.rotate_carry_count_info(value, count)?;
        let bit_width = value.ty().bit_width();
        let ring_ty = self.module.custom_width_int_type(bit_width + 1)?;
        let combined = self.rotate_carry_combined_value(value, ring_ty)?;
        let safe_count = self.create_z_ext_or_trunc(info.safe_count, ring_ty)?;
        let ring_width = self.rotate_const_value(ring_ty, u64::from(bit_width + 1))?;
        let complement = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
            ring_width,
            safe_count,
            "rcr_complement",
        )?;
        let right =
            self.builder()?
                .build_int_lshr::<IntDyn, _, _, _>(combined, safe_count, "rcr_right")?;
        let left = self
            .builder()?
            .build_int_shl::<IntDyn, _, _, _>(combined, complement, "rcr_left")?;
        let rotated =
            self.builder()?
                .build_int_or::<IntDyn, _, _, _>(right, left, "rcr_rotated")?;
        let raw_result = self.create_z_ext_or_trunc(rotated, value.ty())?;
        let result =
            self.builder()?
                .build_select(info.count_nonzero, raw_result, value, "rcr_result")?;
        let cf_new = self.rotate_low_bit_after_lshr(
            rotated,
            self.rotate_const_value(ring_ty, u64::from(bit_width))?,
        )?;
        let cf = self.builder()?.build_select(
            info.count_nonzero,
            cf_new,
            self.load_flag(ExtendedRegisterEnum::CF)?,
            "rcr_cf",
        )?;
        let of_new = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
            self.rotate_msb_flag(result)?,
            self.rotate_second_msb_flag(result)?,
            "rcr_of_new",
        )?;
        let of = self.builder()?.build_select(
            info.count_is_one,
            of_new,
            self.load_flag(ExtendedRegisterEnum::OF)?,
            "rcr_of",
        )?;

        self.store_op(dest, result)?;
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of)
    }

    pub(super) fn lift_rol<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::ROL {
            return Err(Error::UnsupportedInstr(
                "unsupported rotate-left instruction",
            ));
        }
        let operands = instr.operands();
        let dest = &operands[0];
        let count_op = &operands[1];
        let value = self.load_single_int_op(dest, dest.size)?;
        let count = self.load_single_int_op(count_op, dest.size)?;
        let info = self.rotate_plain_count_info(value, count)?;
        let bit_width = value.ty().bit_width();
        let width = self.rotate_const_value(value.ty(), u64::from(bit_width))?;
        let complement = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
            width,
            info.safe_count,
            "rol_complement",
        )?;
        let left =
            self.builder()?
                .build_int_shl::<IntDyn, _, _, _>(value, info.safe_count, "rol_left")?;
        let right =
            self.builder()?
                .build_int_lshr::<IntDyn, _, _, _>(value, complement, "rol_right")?;
        let rotated =
            self.builder()?
                .build_int_or::<IntDyn, _, _, _>(left, right, "rol_rotated")?;
        let result =
            self.builder()?
                .build_select(info.count_nonzero, rotated, value, "rol_result")?;
        let cf_new = self.rotate_lsb_flag(result)?;
        let cf = self.builder()?.build_select(
            info.count_nonzero,
            cf_new,
            self.load_flag(ExtendedRegisterEnum::CF)?,
            "rol_cf",
        )?;
        let of_new = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
            self.rotate_msb_flag(result)?,
            cf_new,
            "rol_of_new",
        )?;
        let of = self.builder()?.build_select(
            info.count_is_one,
            of_new,
            self.load_flag(ExtendedRegisterEnum::OF)?,
            "rol_of",
        )?;

        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of)?;
        self.store_op(dest, result)
    }

    pub(super) fn lift_ror<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::ROR {
            return Err(Error::UnsupportedInstr(
                "unsupported rotate-right instruction",
            ));
        }
        let operands = instr.operands();
        let dest = &operands[0];
        let count_op = &operands[1];
        let value = self.load_single_int_op(dest, dest.size)?;
        let count = self.load_single_int_op(count_op, dest.size)?;
        let info = self.rotate_plain_count_info(value, count)?;
        let bit_width = value.ty().bit_width();
        let width = self.rotate_const_value(value.ty(), u64::from(bit_width))?;
        let complement = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
            width,
            info.safe_count,
            "ror_complement",
        )?;
        let right = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            value,
            info.safe_count,
            "ror_right",
        )?;
        let left = self
            .builder()?
            .build_int_shl::<IntDyn, _, _, _>(value, complement, "ror_left")?;
        let rotated =
            self.builder()?
                .build_int_or::<IntDyn, _, _, _>(right, left, "ror_rotated")?;
        let result =
            self.builder()?
                .build_select(info.count_nonzero, rotated, value, "ror_result")?;
        let cf_new = self.rotate_msb_flag(result)?;
        let cf = self.builder()?.build_select(
            info.count_nonzero,
            cf_new,
            self.load_flag(ExtendedRegisterEnum::CF)?,
            "ror_cf",
        )?;
        let of_new = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
            self.rotate_msb_flag(result)?,
            self.rotate_second_msb_flag(result)?,
            "ror_of_new",
        )?;
        let of = self.builder()?.build_select(
            info.count_is_one,
            of_new,
            self.load_flag(ExtendedRegisterEnum::OF)?,
            "ror_of",
        )?;

        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of)?;
        self.store_op(dest, result)
    }

    fn rotate_plain_count_info(
        &self,
        value: IntValue<'ctx, IntDyn>,
        count: IntValue<'ctx, IntDyn>,
    ) -> Result<RotateCount<'ctx>> {
        let bit_width = value.ty().bit_width();
        let mask = if bit_width == 64 { 0x3f } else { 0x1f };
        let count_ty = count.ty();
        let masked = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            count,
            count_ty.const_int_raw(mask, false)?,
            "rotate_count_masked",
        )?;
        let effective = self.builder()?.build_int_urem::<IntDyn, _, _, _>(
            masked,
            count_ty.const_int_raw(u64::from(bit_width), false)?,
            "rotate_count",
        )?;
        self.rotate_count_from_effective(effective)
    }

    fn rotate_carry_count_info(
        &self,
        value: IntValue<'ctx, IntDyn>,
        count: IntValue<'ctx, IntDyn>,
    ) -> Result<RotateCount<'ctx>> {
        let bit_width = value.ty().bit_width();
        let mask = if bit_width == 64 { 0x3f } else { 0x1f };
        let count_ty = count.ty();
        let masked = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            count,
            count_ty.const_int_raw(mask, false)?,
            "rotate_carry_count_masked",
        )?;
        let effective = if bit_width == 8 || bit_width == 16 {
            self.builder()?.build_int_urem::<IntDyn, _, _, _>(
                masked,
                count_ty.const_int_raw(u64::from(bit_width + 1), false)?,
                "rotate_carry_count",
            )?
        } else {
            masked
        };
        self.rotate_count_from_effective(effective)
    }

    fn rotate_count_from_effective(
        &self,
        count: IntValue<'ctx, IntDyn>,
    ) -> Result<RotateCount<'ctx>> {
        let count_ty = count.ty();
        let count_nonzero = self.builder()?.build_icmp_ne::<IntDyn, _, _, _>(
            count,
            count_ty.const_zero(),
            "rotate_count_nonzero",
        )?;
        let count_is_one = self.builder()?.build_icmp_eq::<IntDyn, _, _, _>(
            count,
            count_ty.const_int_raw(1, false)?,
            "rotate_count_one",
        )?;
        let one = self.rotate_const_value(count_ty, 1)?;
        let safe_count =
            self.builder()?
                .build_select(count_nonzero, count, one, "rotate_safe_count")?;
        Ok(RotateCount {
            safe_count,
            count_nonzero,
            count_is_one,
        })
    }

    fn rotate_carry_combined_value(
        &self,
        value: IntValue<'ctx, IntDyn>,
        ring_ty: IntType<'ctx, IntDyn>,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let bit_width = value.ty().bit_width();
        let wide_value = self.create_z_ext_or_trunc(value, ring_ty)?;
        let cf = self.create_z_ext_or_trunc(self.load_flag(ExtendedRegisterEnum::CF)?, ring_ty)?;
        let shifted_cf = self.builder()?.build_int_shl::<IntDyn, _, _, _>(
            cf,
            ring_ty.const_int_raw(u64::from(bit_width), false)?,
            "rotate_carry_cf",
        )?;
        Ok(self.builder()?.build_int_or::<IntDyn, _, _, _>(
            wide_value,
            shifted_cf,
            "rotate_carry_combined",
        )?)
    }

    fn rotate_lsb_flag(&self, value: IntValue<'ctx, IntDyn>) -> Result<IntValue<'ctx, IntDyn>> {
        let bit = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            value,
            value.ty().const_int_raw(1, false)?,
            "rotate_lsb",
        )?;
        self.create_z_ext_or_trunc(bit, self.module.bool_type().as_dyn())
    }

    fn rotate_msb_flag(&self, value: IntValue<'ctx, IntDyn>) -> Result<IntValue<'ctx, IntDyn>> {
        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            value,
            value
                .ty()
                .const_int_raw(u64::from(value.ty().bit_width() - 1), false)?,
            "rotate_msb_shifted",
        )?;
        self.rotate_lsb_flag(shifted)
    }

    fn rotate_second_msb_flag(
        &self,
        value: IntValue<'ctx, IntDyn>,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            value,
            value
                .ty()
                .const_int_raw(u64::from(value.ty().bit_width() - 2), false)?,
            "rotate_second_msb_shifted",
        )?;
        self.rotate_lsb_flag(shifted)
    }

    fn rotate_low_bit_after_lshr(
        &self,
        value: IntValue<'ctx, IntDyn>,
        shift: IntValue<'ctx, IntDyn>,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let shifted =
            self.builder()?
                .build_int_lshr::<IntDyn, _, _, _>(value, shift, "rotate_bit_source")?;
        self.rotate_lsb_flag(shifted)
    }

    fn rotate_const_value(
        &self,
        ty: IntType<'ctx, IntDyn>,
        value: u64,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        Ok(ty.const_int_raw(value, false)?.as_value().try_into()?)
    }
}
