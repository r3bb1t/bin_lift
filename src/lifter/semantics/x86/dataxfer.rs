use crate::lifter::{eOpConv, LifterX86};
use inkwell::builder::BuilderError;
use zydis::{Instruction, Mnemonic, Operands};

// mov
impl<'ctx> LifterX86<'ctx> {
    //pub(super) fn lift_mov<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
    //    let operands = instr.operands();
    //    //let rhs = self.load_operand(&operands[1])?;
    //    let rhs = self.retdec_load_operand(&operands[1], &None, false)?;
    //    //self.store_op(&operands[0].kind, rhs)?;
    //    self.retdec_store_op(&operands[0], rhs, None)?;
    //
    //    Ok(())
    //}
    pub(super) fn lift_mov<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        let operands = instr.operands();
        let rhs = self.retdec_load_operand(&operands[1], &None, false)?;

        match instr.mnemonic {
            Mnemonic::MOV | Mnemonic::MOVZX => {
                self.retdec_store_op(&operands[0], rhs, Some(eOpConv::ZEXT_TRUNC_OR_BITCAST))?
            }
            Mnemonic::MOVSX | Mnemonic::MOVSXD => {
                self.retdec_store_op(&operands[0], rhs, Some(eOpConv::SEXT_TRUNC_OR_BITCAST))?
            }
            _ => unreachable!(),
        };

        Ok(())
    }
}
