use super::Result;
use crate::lifter::{Error, LifterX86};

use llvmkit::ir::IntValue;
use zydis::{ffi::DecodedOperandKind, Instruction, Mnemonic, Operands};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    fn lift_setcc_with_condition<O: Operands>(
        &mut self,
        instr: &Instruction<O>,
        condition: IntValue<'ctx, bool>,
    ) -> Result<()> {
        let dest = instr
            .operands()
            .first()
            .ok_or(Error::UnsupportedInstr("setcc missing destination operand"))?;
        if !matches!(
            &dest.kind,
            DecodedOperandKind::Reg(_) | DecodedOperandKind::Mem(_)
        ) {
            return Err(Error::UnsupportedInstr(
                "setcc requires register or memory destination",
            ));
        }
        if dest.size != 8 {
            return Err(Error::UnsupportedInstr("setcc requires 8-bit destination"));
        }
        let byte = self.builder()?.build_zext_dyn(
            condition.as_dyn(),
            self.module.i8_type().as_dyn(),
            "setcc",
        )?;

        self.store_op(dest, byte)
    }

    pub(super) fn lift_setb<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETB)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setbe<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETBE)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setl<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETL)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setle<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETLE)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setnb<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETNB)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setnbe<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETNBE)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setnl<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETNL)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setnle<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETNLE)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setno<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETNO)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setnp<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETNP)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setns<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETNS)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setnz<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETNZ)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_seto<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETO)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setp<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETP)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_sets<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETS)?;
        self.lift_setcc_with_condition(instr, condition)
    }

    pub(super) fn lift_setz<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let condition = self.condition_for_mnemonic(Mnemonic::SETZ)?;
        self.lift_setcc_with_condition(instr, condition)
    }
}
