use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::{Instruction, Mnemonic, Operands};

mod binary;
mod convert;
mod dataxfer;
mod flagop;
mod logical;
mod misc;
mod pop;
mod push;
mod ret;

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub(crate) fn lift_instr<O: Operands>(
        &'b self,
        instruction: Instruction<O>,
    ) -> Result<(), BuilderError> {
        match instruction.mnemonic {
            // XADD is actualy semaphore class, but placed in binary
            Mnemonic::ADD | Mnemonic::XADD => self.lift_add(instruction)?,
            Mnemonic::CLC => self.lift_clc(instruction)?,
            Mnemonic::CLD => self.lift_cld(instruction)?,
            Mnemonic::CMC => self.lift_cmc(instruction)?,
            Mnemonic::CWD => self.lift_cwd(instruction)?,
            Mnemonic::LAHF => self.lift_lahf(instruction)?,
            Mnemonic::LEA => self.lift_lea(instruction)?,
            Mnemonic::MOV => self.lift_mov(instruction)?,
            Mnemonic::POP => self.lift_pop(instruction)?,
            Mnemonic::PUSH => self.lift_push(instruction)?,
            Mnemonic::RET => self.lift_ret(instruction)?,
            Mnemonic::SAHF => self.lift_sahf(instruction)?,
            Mnemonic::SALC => self.lift_salc(instruction)?,
            Mnemonic::STC => self.lift_stc(instruction)?,
            Mnemonic::STD => self.lift_std(instruction)?,
            Mnemonic::SUB => self.lift_sub(instruction)?,
            Mnemonic::XOR => self.lift_xor(instruction)?,
            _ => unimplemented!("Mnemonic: {} is not implemented yet!", instruction.mnemonic),
        }

        Ok(())
    }
}
