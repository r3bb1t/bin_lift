use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::{Instruction, Mnemonic, Operands};

mod binary;
mod dataxfer;
mod misc;
mod pop;
mod push;
mod ret;

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub fn lift_instr<O: Operands>(
        &'b self,
        instruction: Instruction<O>,
    ) -> Result<(), BuilderError> {
        match instruction.mnemonic {
            Mnemonic::ADD => self.lift_add(instruction)?,
            Mnemonic::MOV => self.lift_mov(instruction)?,
            Mnemonic::POP => self.lift_pop(instruction)?,
            Mnemonic::PUSH => self.lift_push(instruction)?,
            Mnemonic::SUB => self.lift_sub(instruction)?,
            Mnemonic::RET => self.lift_ret(instruction)?,
            Mnemonic::LEA => self.lift_lea(instruction)?,
            _ => unimplemented!("{}", instruction.mnemonic),
        }

        Ok(())
    }
}
