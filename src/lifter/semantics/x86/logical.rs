use super::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use inkwell::{values::IntValue, IntPredicate};
use zydis::{Instruction, Operands};

impl LifterX86<'_> {
    pub(super) fn lift_and_andn<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();
        let [lhs, rhs] = self.load_two_first_ops(ops)?;

        let lhs_int: IntValue<'_> = lhs.try_into()?;
        let rhs_int: IntValue<'_> = rhs.try_into()?;

        let lhs_final = match instr.mnemonic {
            zydis::Mnemonic::AND => lhs_int,
            zydis::Mnemonic::ANDN => builder.build_not(lhs_int, "andn_not")?,
            _ => unreachable!(),
        };

        let value = builder.build_and(lhs_final, rhs_int, "and_op")?;

        let sf = self.compute_sign_flag(value)?;
        let zf = self.compute_zero_flag(value)?;
        let pf = self.compute_parity_flag(value)?;

        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);

        self.store_cpu_flag_bool(ExtendedRegisterEnum::OF, false);
        self.store_cpu_flag_bool(ExtendedRegisterEnum::CF, false);

        self.store_op(&ops[0], value)?;
        Ok(())
    }

    // NOTE: checked
    pub(super) fn lift_not<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let dest = &ops[0];

        let mut r_value: IntValue<'_> = self.load_single_op(dest, dest.size)?.try_into()?;

        r_value = builder.build_xor(r_value, r_value.get_type().const_all_ones(), "")?;

        self.store_op(dest, r_value)?;
        Ok(())
    }

    // NOTE: checked
    pub(super) fn lift_or<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];

        let rhs = self.load_single_int_op(src, dest.size)?;
        let lhs = self.load_single_int_op(dest, dest.size)?;

        let result = self.builder.build_or(lhs, rhs, "")?;
        let bool_ty = self.context.bool_type();

        let sf = self.compute_sign_flag(result)?;
        let zf = self.compute_zero_flag(result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        self.retdec_store_registers_plus_sflags(
            result,
            &[
                (ExtendedRegisterEnum::AF, bool_ty.const_int(0, false)),
                (ExtendedRegisterEnum::CF, bool_ty.const_int(0, false)),
                (ExtendedRegisterEnum::OF, bool_ty.const_int(0, false)),
            ],
        )?;

        self.store_op(dest, result)?;
        Ok(())
    }

    pub(super) fn lift_test<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let ops = instr.operands();
        let [lhs, rhs] = self.load_two_first_ops(ops)?;
        // What if test will recieve float ?
        let lhs_int: IntValue<'_> = lhs.try_into()?;
        let rhs_int: IntValue<'_> = rhs.try_into()?;

        let test_result = builder.build_and(lhs_int, rhs_int, "test_and")?;

        let zero = test_result.get_type().const_zero();

        let sf = builder.build_int_compare(IntPredicate::SLT, test_result, zero, "sf")?;
        let zf = builder.build_int_compare(IntPredicate::EQ, test_result, zero, "zf")?;
        let pf = self.compute_parity_flag(test_result)?;

        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);

        self.store_cpu_flag_bool(ExtendedRegisterEnum::OF, false);
        self.store_cpu_flag_bool(ExtendedRegisterEnum::CF, false);

        Ok(())
    }

    pub(super) fn lift_xor<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();

        let operand_values = self.load_two_first_ops(operands)?;

        let lhs: IntValue<'_> = operand_values[0].try_into()?;
        let rhs: IntValue<'_> = operand_values[1].try_into()?;

        let result = self.builder.build_xor(lhs, rhs, "xor_")?;

        let bool_ty = self.context.bool_type();

        self.retdec_store_registers_plus_sflags(
            result,
            &[
                (ExtendedRegisterEnum::AF, bool_ty.const_zero()),
                (ExtendedRegisterEnum::CF, bool_ty.const_zero()),
                (ExtendedRegisterEnum::OF, bool_ty.const_zero()),
            ],
        )?;

        self.store_op(&operands[0], result)?;
        Ok(())
    }
}
