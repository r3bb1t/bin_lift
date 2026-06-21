use super::Result;
use crate::lifter::{Error, LifterX86};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{IntDyn, IntValue};
use zydis::{Instruction, Mnemonic, Operands};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_bsr<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];
        let value = self.load_single_int_op(src, dest.size)?;
        let ty = value.ty();
        let is_zero =
            self.builder()?
                .build_icmp_eq::<IntDyn, _, _, _>(value, ty.const_zero(), "bsr_zf")?;

        let mut found = self.module.bool_type().const_zero().as_value().try_into()?;
        let mut result: IntValue<'ctx, IntDyn> = ty.const_zero().as_value().try_into()?;
        for index in (0..ty.bit_width()).rev() {
            let mask = self.builder()?.build_int_shl::<IntDyn, _, _, _>(
                ty.const_int_raw(1, false)?,
                ty.const_int_raw(u64::from(index), false)?,
                "bsr_mask",
            )?;
            let selected =
                self.builder()?
                    .build_int_and::<IntDyn, _, _, _>(value, mask, "bsr_selected")?;
            let bit_set = self.builder()?.build_icmp_ne::<IntDyn, _, _, _>(
                selected,
                ty.const_zero(),
                "bsr_bit_set",
            )?;
            let not_found = self.builder()?.build_icmp_eq::<bool, _, _, _>(
                found,
                self.module.bool_type().const_zero(),
                "bsr_not_found",
            )?;
            let should_set = self.builder()?.build_int_and::<bool, _, _, _>(
                not_found,
                bit_set,
                "bsr_should_set",
            )?;
            let index_value: IntValue<'ctx, IntDyn> = ty
                .const_int_raw(u64::from(index), false)?
                .as_value()
                .try_into()?;
            result = self
                .builder()?
                .build_select(should_set, index_value, result, "bsr_result")?;
            found = self
                .builder()?
                .build_int_or::<bool, _, _, _>(found, bit_set, "bsr_found")?;
        }

        let old_dest = self.load_single_int_op(dest, dest.size)?;
        let result = self
            .builder()?
            .build_select(is_zero, old_dest, result, "bsr_zero_result")?;
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, is_zero.as_dyn())?;
        self.store_op(dest, result)
    }

    pub(super) fn lift_bsf<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];
        let value = self.load_single_int_op(src, dest.size)?;
        let ty = value.ty();
        let is_zero =
            self.builder()?
                .build_icmp_eq::<IntDyn, _, _, _>(value, ty.const_zero(), "bsf_zf")?;

        let mut found = self.module.bool_type().const_zero().as_value().try_into()?;
        let mut result: IntValue<'ctx, IntDyn> = ty.const_zero().as_value().try_into()?;
        for index in 0..ty.bit_width() {
            let mask = self.builder()?.build_int_shl::<IntDyn, _, _, _>(
                ty.const_int_raw(1, false)?,
                ty.const_int_raw(u64::from(index), false)?,
                "bsf_mask",
            )?;
            let selected =
                self.builder()?
                    .build_int_and::<IntDyn, _, _, _>(value, mask, "bsf_selected")?;
            let bit_set = self.builder()?.build_icmp_ne::<IntDyn, _, _, _>(
                selected,
                ty.const_zero(),
                "bsf_bit_set",
            )?;
            let not_found = self.builder()?.build_icmp_eq::<bool, _, _, _>(
                found,
                self.module.bool_type().const_zero(),
                "bsf_not_found",
            )?;
            let should_set = self.builder()?.build_int_and::<bool, _, _, _>(
                not_found,
                bit_set,
                "bsf_should_set",
            )?;
            let index_value: IntValue<'ctx, IntDyn> = ty
                .const_int_raw(u64::from(index), false)?
                .as_value()
                .try_into()?;
            result = self
                .builder()?
                .build_select(should_set, index_value, result, "bsf_result")?;
            found = self
                .builder()?
                .build_int_or::<bool, _, _, _>(found, bit_set, "bsf_found")?;
        }

        let old_dest = self.load_single_int_op(dest, dest.size)?;
        let result = self
            .builder()?
            .build_select(is_zero, old_dest, result, "bsf_zero_result")?;
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, is_zero.as_dyn())?;
        self.store_op(dest, result)
    }

    pub(super) fn lift_bt<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        self.bitbyte_lift_bit_test(instr, Mnemonic::BT)
    }

    pub(super) fn lift_btc<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        self.bitbyte_lift_bit_test(instr, Mnemonic::BTC)
    }

    pub(super) fn lift_btr<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        self.bitbyte_lift_bit_test(instr, Mnemonic::BTR)
    }

    pub(super) fn lift_bts<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        self.bitbyte_lift_bit_test(instr, Mnemonic::BTS)
    }

    fn bitbyte_lift_bit_test<O: Operands>(
        &mut self,
        instr: &Instruction<O>,
        mnemonic: Mnemonic,
    ) -> Result<()> {
        let operands = instr.operands();
        let base = &operands[0];
        let offset = &operands[1];
        let base_value = self.load_single_int_op(base, base.size)?;
        let offset_value = self.load_single_int_op(offset, base.size)?;
        let ty = base_value.ty();
        let bit_width = u64::from(ty.bit_width());
        let masked_offset = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            offset_value,
            offset_value.ty().const_int_raw(bit_width - 1, false)?,
            "bit_offset",
        )?;
        let mask = self.builder()?.build_int_shl::<IntDyn, _, _, _>(
            ty.const_int_raw(1, false)?,
            masked_offset,
            "bit_mask",
        )?;
        let selected =
            self.builder()?
                .build_int_and::<IntDyn, _, _, _>(base_value, mask, "bit_selected")?;
        let cf = self.builder()?.build_icmp_ne::<IntDyn, _, _, _>(
            selected,
            ty.const_zero(),
            "bit_cf",
        )?;
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf.as_dyn())?;

        let result = match mnemonic {
            Mnemonic::BT => return Ok(()),
            Mnemonic::BTC => self
                .builder()?
                .build_int_xor::<IntDyn, _, _, _>(base_value, mask, "btc")?,
            Mnemonic::BTR => {
                let inverted_mask = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
                    mask,
                    ty.const_all_ones(),
                    "btr_mask",
                )?;
                self.builder()?.build_int_and::<IntDyn, _, _, _>(
                    base_value,
                    inverted_mask,
                    "btr",
                )?
            }
            Mnemonic::BTS => self
                .builder()?
                .build_int_or::<IntDyn, _, _, _>(base_value, mask, "bts")?,
            _ => return Err(Error::UnsupportedInstr("unsupported bit-test instruction")),
        };
        self.store_op(base, result)
    }
}
