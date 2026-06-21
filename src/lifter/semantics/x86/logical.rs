use super::Result;
use crate::lifter::{Error, LifterX86};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{IntDyn, IntValue};
use zydis::{Instruction, Mnemonic, Operands};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_and_andn<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let dest = &operands[0];

        let (lhs, rhs) = match instr.mnemonic {
            Mnemonic::AND => (
                self.load_single_int_op(dest, dest.size)?,
                self.load_single_int_op(&operands[1], dest.size)?,
            ),
            Mnemonic::ANDN if operands.len() >= 3 => (
                self.load_single_int_op(&operands[1], dest.size)?,
                self.load_single_int_op(&operands[2], dest.size)?,
            ),
            Mnemonic::ANDN => return Err(Error::UnsupportedInstr("andn requires three operands")),
            _ => {
                return Err(Error::UnsupportedInstr(
                    "unsupported logical and instruction",
                ))
            }
        };

        let lhs = if instr.mnemonic == Mnemonic::ANDN {
            self.builder()?.build_int_xor::<IntDyn, _, _, _>(
                lhs,
                lhs.ty().const_all_ones(),
                "andn_not",
            )?
        } else {
            lhs
        };

        let value = self
            .builder()?
            .build_int_and::<IntDyn, _, _, _>(lhs, rhs, "and")?;

        self.logical_store_status_flags(value)?;
        self.store_cpu_flag_bool(ExtendedRegisterEnum::OF, false)?;
        self.store_cpu_flag_bool(ExtendedRegisterEnum::CF, false)?;
        self.store_op(dest, value)
    }

    pub(super) fn lift_not<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let dest = &instr.operands()[0];
        let value = self.load_single_int_op(dest, dest.size)?;
        let result = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
            value,
            value.ty().const_all_ones(),
            "not",
        )?;
        self.store_op(dest, result)
    }

    pub(super) fn lift_or<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let dest = &operands[0];
        let lhs = self.load_single_int_op(dest, dest.size)?;
        let rhs = self.load_single_int_op(&operands[1], dest.size)?;
        let result = self
            .builder()?
            .build_int_or::<IntDyn, _, _, _>(lhs, rhs, "or")?;

        self.logical_store_status_flags(result)?;
        self.store_cpu_flag_bool(ExtendedRegisterEnum::CF, false)?;
        self.store_cpu_flag_bool(ExtendedRegisterEnum::OF, false)?;
        self.store_op(dest, result)
    }

    pub(super) fn lift_test<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let lhs = self.load_single_int_op(&operands[0], operands[0].size)?;
        let rhs = self.load_single_int_op(&operands[1], operands[0].size)?;
        let result = self
            .builder()?
            .build_int_and::<IntDyn, _, _, _>(lhs, rhs, "test")?;

        self.logical_store_status_flags(result)?;
        self.store_cpu_flag_bool(ExtendedRegisterEnum::OF, false)?;
        self.store_cpu_flag_bool(ExtendedRegisterEnum::CF, false)
    }

    pub(super) fn lift_xor<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let dest = &operands[0];
        let lhs = self.load_single_int_op(dest, dest.size)?;
        let rhs = self.load_single_int_op(&operands[1], dest.size)?;
        let result = self
            .builder()?
            .build_int_xor::<IntDyn, _, _, _>(lhs, rhs, "xor")?;

        self.logical_store_status_flags(result)?;
        self.store_cpu_flag_bool(ExtendedRegisterEnum::CF, false)?;
        self.store_cpu_flag_bool(ExtendedRegisterEnum::OF, false)?;
        self.store_op(dest, result)
    }

    fn logical_store_status_flags(&mut self, value: IntValue<'ctx, IntDyn>) -> Result<()> {
        let zero = value.ty().const_zero();
        let sf = self
            .builder()?
            .build_icmp_slt::<IntDyn, _, _, _>(value, zero, "logical_sf")?
            .as_dyn();
        let zf = self
            .builder()?
            .build_icmp_eq::<IntDyn, _, _, _>(value, zero, "logical_zf")?
            .as_dyn();
        let pf = self.logical_parity_flag(value)?;

        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf)
    }

    fn logical_parity_flag(&self, value: IntValue<'ctx, IntDyn>) -> Result<IntValue<'ctx, IntDyn>> {
        let mut folded = self.create_z_ext_or_trunc(value, self.module.i8_type().as_dyn())?;
        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            folded,
            folded.ty().const_int_raw(4, false)?,
            "logical_pf4",
        )?;
        folded =
            self.builder()?
                .build_int_xor::<IntDyn, _, _, _>(folded, shifted, "logical_pfx4")?;
        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            folded,
            folded.ty().const_int_raw(2, false)?,
            "logical_pf2",
        )?;
        folded =
            self.builder()?
                .build_int_xor::<IntDyn, _, _, _>(folded, shifted, "logical_pfx2")?;
        let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            folded,
            folded.ty().const_int_raw(1, false)?,
            "logical_pf1",
        )?;
        folded =
            self.builder()?
                .build_int_xor::<IntDyn, _, _, _>(folded, shifted, "logical_pfx1")?;
        let low_bit = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            folded,
            folded.ty().const_int_raw(1, false)?,
            "logical_pf_bit",
        )?;
        Ok(self
            .builder()?
            .build_icmp_eq::<IntDyn, _, _, _>(low_bit, low_bit.ty().const_zero(), "logical_pf")?
            .as_dyn())
    }
}
