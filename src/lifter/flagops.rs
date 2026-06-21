use crate::lifter::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{IntDyn, IntPredicate, IntValue};

pub(super) type DynInt<'ctx> = IntValue<'ctx, IntDyn>;

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn store_arithmetic_flags(
        &mut self,
        result: DynInt<'ctx>,
        af: DynInt<'ctx>,
        cf: Option<DynInt<'ctx>>,
        of: DynInt<'ctx>,
    ) -> Result<()> {
        let pf = self.compute_parity_flag(result)?;
        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af)?;
        if let Some(cf) = cf {
            self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
        }
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of)?;
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf)?;
        Ok(())
    }

    pub(super) fn compute_zero_flag(&self, value: DynInt<'ctx>) -> Result<DynInt<'ctx>> {
        Ok(self
            .builder()?
            .build_int_cmp::<IntDyn, _, _, _>(
                IntPredicate::Eq,
                value,
                value.ty().const_zero(),
                "computed_zf",
            )?
            .as_dyn())
    }

    pub(super) fn compute_sign_flag(&self, value: DynInt<'ctx>) -> Result<DynInt<'ctx>> {
        self.high_bit_flag(value, "computed_sf")
    }

    pub(super) fn compute_parity_flag(&self, value: DynInt<'ctx>) -> Result<DynInt<'ctx>> {
        let byte_ty = self.module.i8_type().as_dyn();
        let mut parity = self.create_z_ext_or_trunc(value, byte_ty)?;

        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            parity,
            byte_ty.const_int_raw(4, false)?,
            "pf_shift4",
        )?;
        parity = self
            .builder()?
            .build_int_xor::<IntDyn, _, _, _>(parity, shifted, "pf_xor4")?;

        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            parity,
            byte_ty.const_int_raw(2, false)?,
            "pf_shift2",
        )?;
        parity = self
            .builder()?
            .build_int_xor::<IntDyn, _, _, _>(parity, shifted, "pf_xor2")?;

        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            parity,
            byte_ty.const_int_raw(1, false)?,
            "pf_shift1",
        )?;
        parity = self
            .builder()?
            .build_int_xor::<IntDyn, _, _, _>(parity, shifted, "pf_xor1")?;

        let low_bit = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            parity,
            byte_ty.const_int_raw(1, false)?,
            "pf_low_bit",
        )?;
        Ok(self
            .builder()?
            .build_int_cmp::<IntDyn, _, _, _>(
                IntPredicate::Eq,
                low_bit,
                byte_ty.const_zero(),
                "computed_pf",
            )?
            .as_dyn())
    }

    pub(super) fn compute_aux_flag(
        &self,
        lhs: DynInt<'ctx>,
        rhs: DynInt<'ctx>,
        result: DynInt<'ctx>,
    ) -> Result<DynInt<'ctx>> {
        let lhs_xor_rhs =
            self.builder()?
                .build_int_xor::<IntDyn, _, _, _>(lhs, rhs, "af_lhs_rhs")?;
        let changed =
            self.builder()?
                .build_int_xor::<IntDyn, _, _, _>(lhs_xor_rhs, result, "af_changed")?;
        let masked = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            changed,
            lhs.ty().const_int_raw(0x10, false)?,
            "af_masked",
        )?;
        self.non_zero_flag(masked, "computed_af")
    }

    pub(super) fn compute_overflow_flag_add(
        &self,
        lhs: DynInt<'ctx>,
        rhs: DynInt<'ctx>,
        result: DynInt<'ctx>,
    ) -> Result<DynInt<'ctx>> {
        let lhs_xor_result =
            self.builder()?
                .build_int_xor::<IntDyn, _, _, _>(lhs, result, "add_of_lhs_result")?;
        let rhs_xor_result =
            self.builder()?
                .build_int_xor::<IntDyn, _, _, _>(rhs, result, "add_of_rhs_result")?;
        let overflow_bits = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            lhs_xor_result,
            rhs_xor_result,
            "add_of_bits",
        )?;
        self.high_bit_flag(overflow_bits, "computed_add_of")
    }

    pub(super) fn compute_overflow_flag_sub(
        &self,
        lhs: DynInt<'ctx>,
        rhs: DynInt<'ctx>,
        result: DynInt<'ctx>,
    ) -> Result<DynInt<'ctx>> {
        let lhs_xor_rhs =
            self.builder()?
                .build_int_xor::<IntDyn, _, _, _>(lhs, rhs, "sub_of_lhs_rhs")?;
        let lhs_xor_result =
            self.builder()?
                .build_int_xor::<IntDyn, _, _, _>(lhs, result, "sub_of_lhs_result")?;
        let overflow_bits = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            lhs_xor_rhs,
            lhs_xor_result,
            "sub_of_bits",
        )?;
        self.high_bit_flag(overflow_bits, "computed_sub_of")
    }

    pub(super) fn equal_flag(
        &self,
        lhs: DynInt<'ctx>,
        rhs: DynInt<'ctx>,
        name: &str,
    ) -> Result<DynInt<'ctx>> {
        Ok(self
            .builder()?
            .build_int_cmp::<IntDyn, _, _, _>(IntPredicate::Eq, lhs, rhs, name)?
            .as_dyn())
    }

    pub(super) fn non_zero_flag(&self, value: DynInt<'ctx>, name: &str) -> Result<DynInt<'ctx>> {
        Ok(self
            .builder()?
            .build_int_cmp::<IntDyn, _, _, _>(
                IntPredicate::Ne,
                value,
                value.ty().const_zero(),
                name,
            )?
            .as_dyn())
    }

    pub(super) fn unsigned_less_than_flag(
        &self,
        lhs: DynInt<'ctx>,
        rhs: DynInt<'ctx>,
        name: &str,
    ) -> Result<DynInt<'ctx>> {
        Ok(self
            .builder()?
            .build_int_cmp::<IntDyn, _, _, _>(IntPredicate::Ult, lhs, rhs, name)?
            .as_dyn())
    }

    pub(super) fn and_flag(
        &self,
        lhs: DynInt<'ctx>,
        rhs: DynInt<'ctx>,
        name: &str,
    ) -> Result<DynInt<'ctx>> {
        Ok(self
            .builder()?
            .build_int_and::<IntDyn, _, _, _>(lhs, rhs, name)?)
    }

    pub(super) fn or_flag(
        &self,
        lhs: DynInt<'ctx>,
        rhs: DynInt<'ctx>,
        name: &str,
    ) -> Result<DynInt<'ctx>> {
        Ok(self
            .builder()?
            .build_int_or::<IntDyn, _, _, _>(lhs, rhs, name)?)
    }

    fn high_bit_flag(&self, value: DynInt<'ctx>, name: &str) -> Result<DynInt<'ctx>> {
        Ok(self
            .builder()?
            .build_int_cmp::<IntDyn, _, _, _>(
                IntPredicate::Slt,
                value,
                value.ty().const_zero(),
                name,
            )?
            .as_dyn())
    }
}
