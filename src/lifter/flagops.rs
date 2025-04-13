use super::{Error, ExtendedRegisterEnum, LifterX86, Result};

use inkwell::{values::IntValue, IntPredicate};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn compute_aux_flag(
        &self,
        lhs: IntValue<'ctx>,
        rhs: IntValue<'ctx>,
        result: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let aux_c = result.get_type().const_int(0x10, false);

        let aux_1 = builder.build_xor(result, builder.build_xor(lhs, rhs, "")?, "aux_1_")?;
        let aux_2 = builder.build_and(aux_c, aux_1, "aux_2_")?;
        let af = builder.build_int_compare(IntPredicate::EQ, aux_2, aux_1, "computed_af_")?;
        Ok(af)
    }

    pub(super) fn compute_sign_flag(&self, value: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
        let sf = self.builder.build_int_compare(
            IntPredicate::SLT,
            value,
            value.get_type().const_zero(),
            "sign_flag_",
        )?;
        Ok(sf)
    }

    pub(super) fn compute_zero_flag(&self, value: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
        let zf = self.builder.build_int_compare(
            IntPredicate::EQ,
            value,
            value.get_type().const_zero(),
            "sign_flag_",
        )?;
        Ok(zf)
    }

    pub(super) fn compute_parity_flag(&self, value: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;

        let lsb = builder.build_int_z_extend(
            builder.build_and(value, value.get_type().const_int(0xff, false), "")?,
            self.context.i64_type(),
            "",
        )?;

        let lsb_ty = lsb.get_type();

        let mut parity = builder.build_and(
            builder.build_int_unsigned_rem(
                builder.build_and(
                    builder.build_int_mul(
                        lsb,
                        lsb_ty.const_int(0x0101010101010101, false),
                        "pf1",
                    )?,
                    lsb_ty.const_int(0x8040201008040201, false),
                    "pf2",
                )?,
                lsb_ty.const_int(0x1ff, false),
                "pf3",
            )?,
            lsb_ty.const_int(1, false),
            "pf3",
        )?;

        parity = builder.build_int_compare(IntPredicate::EQ, lsb_ty.const_zero(), parity, "pf5")?;

        Ok(parity)
    }

    pub(super) fn set_rflags_value(&self, value: IntValue<'ctx>) -> Result<()> {
        let builder = &self.builder;
        for flag in 0..12 {
            let shift_amount = flag;
            let shifted_flag_value = builder.build_right_shift(
                value,
                value.get_type().const_int(shift_amount, false),
                false,
                "",
            )?;
            let flag_value =
                builder.build_int_truncate(shifted_flag_value, self.context.bool_type(), "")?;
            let resolved_flag = Self::resolve_flag_from_range(flag)?;
            self.store_cpu_flag(resolved_flag, flag_value);
        }
        Ok(())
    }

    pub(super) fn get_rflags_value(&self) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let biggest_int_type = self.get_max_int_type();
        let mut rflags = biggest_int_type.const_zero();
        for flag in 0..12 {
            let flag_value = self.load_flag(Self::resolve_flag_from_range(flag)?)?;
            let shift_amount = flag;
            let shifted_flag_value = builder.build_left_shift(
                builder.build_int_z_extend(flag_value, biggest_int_type, "")?,
                biggest_int_type.const_int(shift_amount, false),
                "",
            )?;
            rflags = builder.build_or(rflags, shifted_flag_value, "")?;
        }
        Ok(rflags)
    }

    fn resolve_flag_from_range(range_int: u64) -> Result<ExtendedRegisterEnum> {
        match range_int {
            0 => Ok(ExtendedRegisterEnum::CF),
            1 => Ok(ExtendedRegisterEnum::Reserved1),
            2 => Ok(ExtendedRegisterEnum::PF),
            3 => Ok(ExtendedRegisterEnum::Reserved3),
            4 => Ok(ExtendedRegisterEnum::AF),
            5 => Ok(ExtendedRegisterEnum::Reserved5),
            6 => Ok(ExtendedRegisterEnum::ZF),
            7 => Ok(ExtendedRegisterEnum::SF),
            8 => Ok(ExtendedRegisterEnum::TF),
            9 => Ok(ExtendedRegisterEnum::IF),
            10 => Ok(ExtendedRegisterEnum::DF),
            11 => Ok(ExtendedRegisterEnum::OF),
            12 => Ok(ExtendedRegisterEnum::IOPL), // prob the unnecesarry one
            _ => Err(Error::FlagResolveError(range_int)),
        }
    }
}
