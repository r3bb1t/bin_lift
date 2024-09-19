use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::{Instruction, Operands};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn lift_nop<O: Operands>(&self, _instr: Instruction<O>) -> Result<(), BuilderError> {
        Ok(())
    }
}
