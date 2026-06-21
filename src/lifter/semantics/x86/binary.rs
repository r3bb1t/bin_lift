use crate::lifter::{Error, LifterX86};
use crate::miscellaneous::ExtendedRegisterEnum;

use super::Result;
use llvmkit::ir::IntDyn;
use zydis::{Instruction, Mnemonic, Operands};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_adc<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::ADC {
            return Err(Error::UnsupportedInstr("unsupported adc instruction"));
        }

        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];

        let lhs = self.load_single_int_op(dest, dest.size)?;
        let rhs = self.load_single_int_op(src, dest.size)?;
        let carry_in =
            self.create_z_ext_or_trunc(self.load_flag(ExtendedRegisterEnum::CF)?, lhs.ty())?;

        let partial = self
            .builder()?
            .build_int_add::<IntDyn, _, _, _>(lhs, rhs, "adc_partial")?;
        let result =
            self.builder()?
                .build_int_add::<IntDyn, _, _, _>(partial, carry_in, "adc_result")?;

        let carry_from_add = self.unsigned_less_than_flag(partial, lhs, "adc_cf_add")?;
        let carry_from_carry = self.unsigned_less_than_flag(result, partial, "adc_cf_carry")?;
        let cf = self.or_flag(carry_from_add, carry_from_carry, "adc_cf")?;
        let af = self.compute_aux_flag(lhs, rhs, result)?;
        let of = self.compute_overflow_flag_add(lhs, rhs, result)?;

        self.store_arithmetic_flags(result, af, Some(cf), of)?;
        self.store_op(dest, result)
    }

    pub(super) fn lift_add_sub<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];

        let lhs = self.load_single_int_op(dest, dest.size)?;
        let rhs = self.load_single_int_op(src, dest.size)?;

        let (result, cf, of) = match instr.mnemonic {
            Mnemonic::ADD => {
                let result =
                    self.builder()?
                        .build_int_add::<IntDyn, _, _, _>(lhs, rhs, "add_result")?;
                let cf = self.unsigned_less_than_flag(result, lhs, "add_cf")?;
                let of = self.compute_overflow_flag_add(lhs, rhs, result)?;
                (result, cf, of)
            }
            Mnemonic::SUB => {
                let result =
                    self.builder()?
                        .build_int_sub::<IntDyn, _, _, _>(lhs, rhs, "sub_result")?;
                let cf = self.unsigned_less_than_flag(lhs, rhs, "sub_cf")?;
                let of = self.compute_overflow_flag_sub(lhs, rhs, result)?;
                (result, cf, of)
            }
            _ => {
                return Err(Error::UnsupportedInstr(
                    "unsupported arithmetic instruction",
                ))
            }
        };

        let af = self.compute_aux_flag(lhs, rhs, result)?;
        self.store_arithmetic_flags(result, af, Some(cf), of)?;
        self.store_op(dest, result)
    }

    pub(super) fn lift_cmp<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::CMP {
            return Err(Error::UnsupportedInstr("unsupported compare instruction"));
        }

        let operands = instr.operands();
        let lhs = self.load_single_int_op(&operands[0], operands[0].size)?;
        let rhs = self.load_single_int_op(&operands[1], operands[0].size)?;

        let result = self
            .builder()?
            .build_int_sub::<IntDyn, _, _, _>(lhs, rhs, "cmp_result")?;
        let af = self.compute_aux_flag(lhs, rhs, result)?;
        let cf = self.unsigned_less_than_flag(lhs, rhs, "cmp_cf")?;
        let of = self.compute_overflow_flag_sub(lhs, rhs, result)?;

        self.store_arithmetic_flags(result, af, Some(cf), of)
    }

    pub(super) fn lift_dec<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::DEC {
            return Err(Error::UnsupportedInstr("unsupported dec instruction"));
        }

        let op = &instr.operands()[0];
        let lhs = self.load_single_int_op(op, op.size)?;
        let one = lhs.ty().const_int_raw(1, false)?;
        let result = self
            .builder()?
            .build_int_sub::<IntDyn, _, _, _>(lhs, one, "dec_result")?;

        let af = self.compute_aux_flag(lhs, one.as_value().try_into()?, result)?;
        let of = self.compute_overflow_flag_sub(lhs, one.as_value().try_into()?, result)?;

        self.store_arithmetic_flags(result, af, None, of)?;
        self.store_op(op, result)
    }

    pub(super) fn lift_inc<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::INC {
            return Err(Error::UnsupportedInstr("unsupported inc instruction"));
        }

        let op = &instr.operands()[0];
        let lhs = self.load_single_int_op(op, op.size)?;
        let one = lhs.ty().const_int_raw(1, false)?;
        let result = self
            .builder()?
            .build_int_add::<IntDyn, _, _, _>(lhs, one, "inc_result")?;

        let af = self.compute_aux_flag(lhs, one.as_value().try_into()?, result)?;
        let of = self.compute_overflow_flag_add(lhs, one.as_value().try_into()?, result)?;

        self.store_arithmetic_flags(result, af, None, of)?;
        self.store_op(op, result)
    }

    pub(super) fn lift_neg<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::NEG {
            return Err(Error::UnsupportedInstr("unsupported neg instruction"));
        }

        let dest = &instr.operands()[0];
        let value = self.load_single_int_op(dest, dest.size)?;
        let zero = value.ty().const_zero();
        let result = self
            .builder()?
            .build_int_sub::<IntDyn, _, _, _>(zero, value, "neg_result")?;

        let low_nibble = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            value,
            value.ty().const_int_raw(0x0f, false)?,
            "neg_low_nibble",
        )?;
        let af = self.non_zero_flag(low_nibble, "neg_af")?;
        let cf = self.non_zero_flag(value, "neg_cf")?;
        let pf = self.compute_parity_flag(result)?;
        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        let result_equals_value = self.equal_flag(result, value, "neg_of_eq")?;
        let of = self.and_flag(cf, result_equals_value, "neg_of")?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af)?;
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of)?;
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf)?;
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf)?;

        self.store_op(dest, result)
    }

    pub(super) fn lift_sbb<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::SBB {
            return Err(Error::UnsupportedInstr("unsupported sbb instruction"));
        }

        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];

        let lhs = self.load_single_int_op(dest, dest.size)?;
        let rhs = self.load_single_int_op(src, dest.size)?;
        let borrow_in =
            self.create_z_ext_or_trunc(self.load_flag(ExtendedRegisterEnum::CF)?, lhs.ty())?;

        let partial = self
            .builder()?
            .build_int_sub::<IntDyn, _, _, _>(lhs, rhs, "sbb_partial")?;
        let result =
            self.builder()?
                .build_int_sub::<IntDyn, _, _, _>(partial, borrow_in, "sbb_result")?;

        let borrow_from_sub = self.unsigned_less_than_flag(lhs, rhs, "sbb_cf_sub")?;
        let borrow_from_carry = self.unsigned_less_than_flag(partial, borrow_in, "sbb_cf_carry")?;
        let cf = self.or_flag(borrow_from_sub, borrow_from_carry, "sbb_cf")?;
        let af = self.compute_aux_flag(lhs, rhs, result)?;
        let of = self.compute_overflow_flag_sub(lhs, rhs, result)?;

        self.store_arithmetic_flags(result, af, Some(cf), of)?;
        self.store_op(dest, result)
    }
}
