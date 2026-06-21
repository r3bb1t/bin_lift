use super::Result;
use crate::lifter::{Error, LifterX86};

use llvmkit::ir::IntValue;
use zydis::{ffi::DecodedOperandKind, Instruction, Mnemonic, Operands};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    fn lift_cmov_with_condition<O: Operands>(
        &mut self,
        instr: &Instruction<O>,
        condition: IntValue<'ctx, bool>,
    ) -> Result<()> {
        let operands = instr.operands();
        let dest = operands
            .first()
            .ok_or(Error::UnsupportedInstr("cmov missing destination operand"))?;
        let src = operands
            .get(1)
            .ok_or(Error::UnsupportedInstr("cmov missing source operand"))?;
        if !matches!(&dest.kind, DecodedOperandKind::Reg(_)) {
            return Err(Error::UnsupportedInstr(
                "cmov requires register destination",
            ));
        }
        if !matches!(
            &src.kind,
            DecodedOperandKind::Reg(_) | DecodedOperandKind::Mem(_)
        ) {
            return Err(Error::UnsupportedInstr(
                "cmov requires register or memory source",
            ));
        }
        if !matches!(dest.size, 16 | 32 | 64) {
            return Err(Error::UnsupportedInstr(
                "cmov requires 16/32/64-bit operands",
            ));
        }
        let lhs = self.load_single_int_op(dest, dest.size)?;
        let rhs = self.load_single_int_op(src, dest.size)?;
        let selected = self.builder()?.build_select(condition, rhs, lhs, "cmov")?;

        self.store_op(dest, selected)
    }

    pub(super) fn lift_cmovb<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVB)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovbe<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVBE)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovl<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVL)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovle<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVLE)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovnb<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVNB)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovnbe<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVNBE)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovnl<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVNL)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovnle<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVNLE)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovno<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVNO)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovnp<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVNP)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovns<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVNS)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovnz<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVNZ)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovo<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVO)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovp<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVP)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovs<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVS)?;
        self.lift_cmov_with_condition(instr, condition)
    }

    pub(super) fn lift_cmovz<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::CMOVZ)?;
        self.lift_cmov_with_condition(instr, condition)
    }
}
