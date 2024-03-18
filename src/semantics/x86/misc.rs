use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::ffi::DecodedOperandKind;
use zydis::{Instruction, Operands};

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub(in crate::semantics) fn lift_lea<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let operands = instr.operands();

        // TODO: Make it more readable
        if let DecodedOperandKind::Mem(ref to_calculate) = operands[0].kind {
            let addr = self.calc_mem_operand(to_calculate)?;
            self.store_op(operands[1].clone().kind, addr);
        } else if let DecodedOperandKind::Mem(ref to_calculate) = operands[1].kind {
            let addr = self.calc_mem_operand(to_calculate)?;
            self.store_op(operands[0].clone().kind, addr);
        } else {
            unreachable!("Error while working with LEA\n{:?}", operands);
        }
        Ok(())
    }
}
