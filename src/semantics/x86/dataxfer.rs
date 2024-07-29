use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::{Instruction, Operands};

// mov
impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub(in crate::semantics) fn lift_mov<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let operands = instr.operands();
        let rhs = self.load_operand(&operands[1])?;
        self.store_op(&operands[0].kind, rhs)?;

        Ok(())
    }
}
