use crate::miscellaneous::ExtendedRegisterEnum;

use super::{LifterX86, Result};

use inkwell::{values::IntValue, IntPredicate};
use zydis::{ffi::DecodedOperand, Instruction, Operands};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn lift_cmovb<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let cf = self.load_flag(ExtendedRegisterEnum::CF)?;

        let condition = builder.build_int_compare(
            IntPredicate::EQ,
            cf,
            cf.get_type().const_int(1, false),
            "cmvob_condition",
        )?;

        self.cmov_helper(ops, condition, "cmovb_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovbe<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let zf = self.load_flag(ExtendedRegisterEnum::ZF)?;
        let cf = self.load_flag(ExtendedRegisterEnum::CF)?;

        let condition = builder.build_or(zf, cf, "cmovbe_condition")?;

        self.cmov_helper(ops, condition, "cmovl_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovl<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let sf = self.load_flag(ExtendedRegisterEnum::SF)?;
        let of = self.load_flag(ExtendedRegisterEnum::OF)?;

        let condition = builder.build_int_compare(
            IntPredicate::NE,
            sf,
            of.get_type().const_int(1, false),
            "cmovl_condition",
        )?;

        self.cmov_helper(ops, condition, "cmovl_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovle<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let zf = self.load_flag(ExtendedRegisterEnum::ZF)?;
        let sf = self.load_flag(ExtendedRegisterEnum::SF)?;
        let of = self.load_flag(ExtendedRegisterEnum::OF)?;

        let sf_neg_of = builder.build_int_compare(IntPredicate::NE, sf, of, "sf_neg_of_cmovle")?;
        let condition = builder.build_or(zf, sf_neg_of, "cmovnle_condition")?;

        self.cmov_helper(ops, condition, "cmovle_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovnb<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let cf = self.load_flag(ExtendedRegisterEnum::CF)?;
        let not_cf = builder.build_not(cf, "cmovnb_not_cf")?;

        self.cmov_helper(ops, not_cf, "cmovnb_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovnbe<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let cf = self.load_flag(ExtendedRegisterEnum::CF)?;
        let zf = self.load_flag(ExtendedRegisterEnum::ZF)?;

        let condition = builder.build_and(
            builder.build_not(cf, "cmovnbe_not_cf")?,
            builder.build_not(zf, "cmovnbe_not_zf")?,
            "cmovnbe_condition",
        )?;

        self.cmov_helper(ops, condition, "cmovl_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovnl<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let sf = self.load_flag(ExtendedRegisterEnum::SF)?;
        let of = self.load_flag(ExtendedRegisterEnum::OF)?;

        let condition = builder.build_int_compare(IntPredicate::EQ, sf, of, "cmovl_compare")?;

        self.cmov_helper(ops, condition, "cmovnl_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovnle<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let zf = self.load_flag(ExtendedRegisterEnum::ZF)?;
        let sf = self.load_flag(ExtendedRegisterEnum::SF)?;
        let of = self.load_flag(ExtendedRegisterEnum::OF)?;

        let condition = builder.build_and(
            builder.build_not(zf, "not_zf_cmovnle")?,
            builder.build_int_compare(IntPredicate::EQ, sf, of, "sf_eq_of")?,
            "cmovnle_condition",
        )?;

        self.cmov_helper(ops, condition, "cmovnle_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovno<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let of = self.load_flag(ExtendedRegisterEnum::OF)?;
        let not_of = builder.build_not(of, "not_of_")?;

        self.cmov_helper(ops, not_of, "cmovno_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovnp<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let ops = instr.operands();

        let pf = self.load_flag(ExtendedRegisterEnum::PF)?;
        let not_pf = self.builder.build_not(pf, "cmovnp_not_pf")?;

        self.cmov_helper(ops, not_pf, "cmovp_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovns<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let sf = self.load_flag(ExtendedRegisterEnum::SF)?;
        let condition = builder.build_int_compare(
            IntPredicate::EQ,
            sf,
            sf.get_type().const_zero(),
            "cmovns_condition",
        )?;

        self.cmov_helper(ops, condition, "cmovns_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovnz<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let zf = self.load_flag(ExtendedRegisterEnum::ZF)?;

        let condition = builder.build_int_compare(
            IntPredicate::EQ,
            zf,
            zf.get_type().const_int(1, false),
            "cmovnz_condition",
        )?;

        self.cmov_helper(ops, condition, "cmovnz_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovo<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let ops = instr.operands();

        let of = self.load_flag(ExtendedRegisterEnum::OF)?;

        self.cmov_helper(ops, of, "cmovo_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovp<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let ops = instr.operands();

        let pf = self.load_flag(ExtendedRegisterEnum::PF)?;

        self.cmov_helper(ops, pf, "cmovp_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovs<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let ops = instr.operands();

        let sf = self.load_flag(ExtendedRegisterEnum::SF)?;

        self.cmov_helper(ops, sf, "cmovp_compare")?;
        Ok(())
    }

    pub(super) fn lift_cmovz<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let ops = instr.operands();

        let zf = self.load_flag(ExtendedRegisterEnum::ZF)?;

        self.cmov_helper(ops, zf, "cmovp_compare")?;
        Ok(())
    }

    /// Get lvalue, rvalue, build select value. Also does the store of op
    fn cmov_helper(
        &self,
        ops: &[DecodedOperand],
        condition: IntValue<'ctx>,
        select_text: &'static str,
    ) -> Result<()> {
        let loaded_ops = self.load_two_first_ops(ops)?;
        let lhs: IntValue<'_> = loaded_ops[0].try_into()?;
        let rhs: IntValue<'_> = loaded_ops[1].try_into()?;

        let result = self
            .builder
            .build_select(condition, rhs, lhs, select_text)?
            .into_int_value();

        self.store_op(&ops[0], result)?;
        Ok(())
    }
}
