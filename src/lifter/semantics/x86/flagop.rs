use super::Result;
use crate::lifter::{Error, LifterX86};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{IntDyn, IntValue};
use zydis::{Mnemonic, Register};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_flagop(&mut self, mnemonic: Mnemonic) -> Result<()> {
        match mnemonic {
            Mnemonic::CLC => self.lift_clc(),
            Mnemonic::STC => self.lift_stc(),
            Mnemonic::CLD => self.lift_cld(),
            Mnemonic::STD => self.lift_std(),
            Mnemonic::CMC => self.lift_cmc(),
            Mnemonic::LAHF => self.lift_lahf(),
            Mnemonic::SAHF => self.lift_sahf(),
            Mnemonic::SALC => self.lift_salc(),
            _ => Err(Error::UnsupportedInstr("unsupported flag instruction")),
        }
    }

    pub(super) fn lift_stc(&mut self) -> Result<()> {
        self.store_cpu_flag_bool(ExtendedRegisterEnum::CF, true)
    }

    pub(super) fn lift_cmc(&mut self) -> Result<()> {
        let cf = self.load_flag(ExtendedRegisterEnum::CF)?;
        let inverted =
            self.builder()?
                .build_icmp_eq::<IntDyn, _, _, _>(cf, cf.ty().const_zero(), "cmc")?;
        self.store_cpu_flag(ExtendedRegisterEnum::CF, inverted.as_dyn())
    }

    pub(super) fn lift_clc(&mut self) -> Result<()> {
        self.store_cpu_flag_bool(ExtendedRegisterEnum::CF, false)
    }

    pub(super) fn lift_cld(&mut self) -> Result<()> {
        self.store_cpu_flag_bool(ExtendedRegisterEnum::DF, false)
    }

    pub(super) fn lift_std(&mut self) -> Result<()> {
        self.store_cpu_flag_bool(ExtendedRegisterEnum::DF, true)
    }

    pub(super) fn lift_salc(&mut self) -> Result<()> {
        let cf = self.load_flag(ExtendedRegisterEnum::CF)?;
        let cf_set = self.builder()?.build_icmp_ne::<IntDyn, _, _, _>(
            cf,
            cf.ty().const_zero(),
            "salc_cf",
        )?;
        let i8_ty = self.module.i8_type().as_dyn();
        let all_ones: IntValue<'ctx, IntDyn> = i8_ty.const_all_ones().as_value().try_into()?;
        let zero: IntValue<'ctx, IntDyn> = i8_ty.const_zero().as_value().try_into()?;
        let value = self
            .builder()?
            .build_select(cf_set, all_ones, zero, "salc")?;
        self.store_reg(Register::AL, value)
    }

    pub(super) fn lift_lahf(&mut self) -> Result<()> {
        let i8_ty = self.module.i8_type().as_dyn();
        let mut value: IntValue<'ctx, IntDyn> =
            i8_ty.const_int_raw(0x02, false)?.as_value().try_into()?;

        for (flag, bit) in [
            (ExtendedRegisterEnum::CF, 0_u64),
            (ExtendedRegisterEnum::PF, 2),
            (ExtendedRegisterEnum::AF, 4),
            (ExtendedRegisterEnum::ZF, 6),
            (ExtendedRegisterEnum::SF, 7),
        ] {
            let flag_bit = self.lahf_flag_bit(flag, bit)?;
            value = self
                .builder()?
                .build_int_or::<IntDyn, _, _, _>(value, flag_bit, "lahf")?;
        }

        self.store_reg(Register::AH, value)
    }

    pub(super) fn lift_sahf(&mut self) -> Result<()> {
        let ah: IntValue<'ctx, IntDyn> = self.load_register_value(&Register::AH)?.try_into()?;
        for (flag, bit) in [
            (ExtendedRegisterEnum::CF, 0_u64),
            (ExtendedRegisterEnum::PF, 2),
            (ExtendedRegisterEnum::AF, 4),
            (ExtendedRegisterEnum::ZF, 6),
            (ExtendedRegisterEnum::SF, 7),
        ] {
            let value = self.sahf_flag_bit(ah, bit)?;
            self.store_cpu_flag(flag, value)?;
        }
        Ok(())
    }

    fn lahf_flag_bit(
        &self,
        flag: ExtendedRegisterEnum,
        bit: u64,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let i8_ty = self.module.i8_type().as_dyn();
        let flag_value = self.create_z_ext_or_trunc(self.load_flag(flag)?, i8_ty)?;
        if bit == 0 {
            return Ok(flag_value);
        }
        Ok(self.builder()?.build_int_shl::<IntDyn, _, _, _>(
            flag_value,
            i8_ty.const_int_raw(bit, false)?,
            "lahf_bit",
        )?)
    }

    fn sahf_flag_bit(
        &self,
        ah: IntValue<'ctx, IntDyn>,
        bit: u64,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let i8_ty = self.module.i8_type().as_dyn();
        let shifted = if bit == 0 {
            ah
        } else {
            self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
                ah,
                i8_ty.const_int_raw(bit, false)?,
                "sahf_shift",
            )?
        };
        Ok(self.builder()?.build_int_and::<IntDyn, _, _, _>(
            shifted,
            i8_ty.const_int_raw(1, false)?,
            "sahf_bit",
        )?)
    }
}
