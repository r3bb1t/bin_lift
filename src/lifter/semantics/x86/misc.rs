use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::{Instruction, Operands};

impl<'ctx> LifterX86<'ctx> {
    // TODO: Consider using retdec's implementation
    pub(super) fn lift_lea<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        //let operands = instr.operands();
        //
        //// TODO: Make it more readable
        //if let DecodedOperandKind::Mem(ref to_calculate) = operands[0].kind {
        //    let addr = self.calc_mem_operand(to_calculate)?;
        //    self.store_op(&operands[1].kind, addr)?;
        //} else if let DecodedOperandKind::Mem(ref to_calculate) = operands[1].kind {
        //    let addr = self.calc_mem_operand(to_calculate)?;
        //    self.store_op(&operands[0].kind, addr)?;
        //} else {
        //    unreachable!("Error while working with LEA\n{:?}", operands);
        //}
        //Ok(())
        //

        let operands = instr.operands();
        let op1 = self.retdec_load_operand(&operands[1], &None, true)?;

        self.retdec_store_op(&operands[0], op1, None)?;

        Ok(())
    }
}
