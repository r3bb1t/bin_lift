use super::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegister;

use inkwell::IntPredicate;

impl LifterX86<'_> {
    pub(super) fn lift_stc(&self) -> Result<()> {
        self.store_cpu_flag_bool(ExtendedRegister::CF, true);
        Ok(())
    }

    pub(super) fn lift_cmc(&self) -> Result<()> {
        let cf = self.load_flag(ExtendedRegister::CF)?;
        let xor_op = self
            .builder
            .build_xor(cf, cf.get_type().const_int(1, false), "cmc_")?;
        self.store_cpu_flag(ExtendedRegister::CF, xor_op);
        Ok(())
    }

    pub(super) fn lift_clc(&self) -> Result<()> {
        self.store_cpu_flag_bool(ExtendedRegister::CF, false);
        Ok(())
    }

    pub(super) fn lift_cld(&self) -> Result<()> {
        self.store_cpu_flag_bool(ExtendedRegister::DF, false);
        Ok(())
    }

    pub(super) fn lift_std(&self) -> Result<()> {
        self.store_cpu_flag_bool(ExtendedRegister::DF, true);
        Ok(())
    }

    pub(super) fn lift_salc(&self) -> Result<()> {
        let builder = &self.builder;
        let cf = self.load_flag(ExtendedRegister::CF)?;
        let icmp = builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            cf,
            cf.get_type().const_zero(),
            "",
        )?;
        let i8_ty = self.context.i8_type();
        let v = builder.build_select(
            icmp,
            i8_ty.const_zero(),
            i8_ty.const_int(0xff, false),
            "salc_",
        )?;
        self.store_cpu_flag(ExtendedRegister::AL, v.into_int_value());
        Ok(())
    }

    //pub(super) fn lift_lahf(&self) -> Result<()> {
    //    let builder = &self.builder;
    //    let i8_ty = self.context.i8_type();
    //
    //    let cf =
    //        builder.build_int_z_extend(self.load_flag(ExtendedRegister::CF)?, i8_ty, "lahf_")?;
    //    let pf =
    //        builder.build_int_z_extend(self.load_flag(ExtendedRegister::PF)?, i8_ty, "lahf_")?;
    //    let af =
    //        builder.build_int_z_extend(self.load_flag(ExtendedRegister::AF)?, i8_ty, "lahf_")?;
    //    let zf =
    //        builder.build_int_z_extend(self.load_flag(ExtendedRegister::ZF)?, i8_ty, "lahf_")?;
    //    let sf =
    //        builder.build_int_z_extend(self.load_flag(ExtendedRegister::SF)?, i8_ty, "lahf_")?;
    //
    //    let zero = i8_ty.const_zero();
    //    let one = i8_ty.const_int(1, false);
    //
    //    let mut val = zero;
    //    val = builder.build_or(val, cf, "")?;
    //    val = builder.build_or(
    //        val,
    //        builder.build_left_shift(one, i8_ty.const_int(1, false), "lahf_")?,
    //        "",
    //    )?;
    //    val = builder.build_or(
    //        val,
    //        builder.build_left_shift(pf, i8_ty.const_int(2, false), "lahf_")?,
    //        "",
    //    )?;
    //    val = builder.build_or(
    //        val,
    //        builder.build_left_shift(af, i8_ty.const_int(4, false), "lahf_")?,
    //        "",
    //    )?;
    //    val = builder.build_or(
    //        val,
    //        builder.build_left_shift(zf, i8_ty.const_int(6, false), "lahf_")?,
    //        "",
    //    )?;
    //    val = builder.build_or(
    //        val,
    //        builder.build_left_shift(sf, i8_ty.const_int(7, false), "lahf_")?,
    //        "",
    //    )?;
    //
    //    self.store_cpu_flag(ExtendedRegister::AH, val);
    //    Ok(())
    //}

    pub(super) fn lift_lahf(&self) -> Result<()> {
        let builder = &self.builder;
        let i8_ty = self.context.i8_type();

        let mut sf = self.load_flag(ExtendedRegister::SF)?;
        let mut zf = self.load_flag(ExtendedRegister::ZF)?;
        let mut af = self.load_flag(ExtendedRegister::AF)?;
        let mut pf = self.load_flag(ExtendedRegister::PF)?;
        let mut cf = self.load_flag(ExtendedRegister::CF)?;

        cf = builder.build_int_z_extend(cf, i8_ty, "")?;
        pf = builder.build_left_shift(
            builder.build_int_z_extend(pf, i8_ty, "")?,
            i8_ty.const_int(2, false),
            "",
        )?;
        af = builder.build_left_shift(
            builder.build_int_z_extend(af, i8_ty, "")?,
            i8_ty.const_int(4, false),
            "",
        )?;
        zf = builder.build_left_shift(
            builder.build_int_z_extend(zf, i8_ty, "")?,
            i8_ty.const_int(6, false),
            "",
        )?;
        sf = builder.build_left_shift(
            builder.build_int_z_extend(sf, i8_ty, "")?,
            i8_ty.const_int(7, false),
            "",
        )?;

        let r_value = builder.build_int_add(
            builder.build_or(
                builder.build_or(
                    builder.build_or(cf, pf, "")?,
                    builder.build_or(af, sf, "")?,
                    "",
                )?,
                zf,
                "",
            )?,
            cf.get_type().const_int(2, false),
            "",
        )?;

        self.store_cpu_flag(ExtendedRegister::AF, r_value);
        Ok(())
    }

    //pub(super) fn lift_sahf(&self) -> Result<()> {
    //    let builder = &self.builder;
    //
    //    let ah = self.load_flag(ExtendedRegister::AH)?;
    //    let ah_ty = ah.get_type();
    //
    //    let zero = ah_ty.const_zero();
    //
    //    self.store_cpu_flag(
    //        ExtendedRegister::CF,
    //        builder.build_and(ah, ah_ty.const_int(1 << 0, false), "sahf_")?,
    //    );
    //    self.store_cpu_flag(
    //        ExtendedRegister::PF,
    //        builder.build_int_compare(
    //            IntPredicate::NE,
    //            builder.build_and(ah, ah_ty.const_int(1 << 2, false), "sahf_")?,
    //            zero,
    //            "",
    //        )?,
    //    );
    //    self.store_cpu_flag(
    //        ExtendedRegister::AF,
    //        builder.build_int_compare(
    //            IntPredicate::NE,
    //            builder.build_and(ah, ah_ty.const_int(1 << 4, false), "sahf_")?,
    //            zero,
    //            "",
    //        )?,
    //    );
    //    self.store_cpu_flag(
    //        ExtendedRegister::ZF,
    //        builder.build_int_compare(
    //            IntPredicate::NE,
    //            builder.build_and(ah, ah_ty.const_int(1 << 6, false), "sahf_")?,
    //            zero,
    //            "",
    //        )?,
    //    );
    //    self.store_cpu_flag(
    //        ExtendedRegister::SF,
    //        builder.build_int_compare(
    //            IntPredicate::NE,
    //            builder.build_and(ah, ah_ty.const_int(1 << 7, false), "sahf_")?,
    //            zero,
    //            "",
    //        )?,
    //    );
    //    Ok(())
    //}
    pub(super) fn lift_sahf(&self) -> Result<()> {
        let builder = &self.builder;

        let ah = self.load_flag(ExtendedRegister::AH)?;
        let ah_ty = ah.get_type();

        let one = ah_ty.const_int(1, false);

        let cf = builder.build_and(
            builder.build_right_shift(ah, ah_ty.const_int(0, false), false, "")?,
            one,
            "sahf_cf_",
        )?;
        let pf = builder.build_and(
            builder.build_right_shift(ah, ah_ty.const_int(2, false), false, "")?,
            one,
            "sahf_pf_",
        )?;
        let af = builder.build_and(
            builder.build_right_shift(ah, ah_ty.const_int(4, false), false, "")?,
            one,
            "sahf_af_",
        )?;
        let zf = builder.build_and(
            builder.build_right_shift(ah, ah_ty.const_int(6, false), false, "")?,
            one,
            "sahf_zf_",
        )?;
        let sf = builder.build_and(
            builder.build_right_shift(ah, ah_ty.const_int(7, false), false, "")?,
            one,
            "sahf_sf_",
        )?;

        self.store_cpu_flag(ExtendedRegister::CF, cf);
        self.store_cpu_flag(ExtendedRegister::PF, pf);
        self.store_cpu_flag(ExtendedRegister::AF, af);
        self.store_cpu_flag(ExtendedRegister::ZF, zf);
        self.store_cpu_flag(ExtendedRegister::SF, sf);
        Ok(())
    }
}
