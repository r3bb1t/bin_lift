use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::{Instruction, Operands};

impl<'ctx> LifterX86<'ctx> {
    pub(in crate::semantics) fn lift_sets<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setnp<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setnz<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setl<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setle<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setz<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setp<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setno<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setnbe<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setnb<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setb<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setns<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setnle<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setbe<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setol<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }

    pub(in crate::semantics) fn lift_setnll<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }
}
