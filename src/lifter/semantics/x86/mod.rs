use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::{FullInstruction, Mnemonic};

use super::Lifter;

mod binary;
mod bitbyte;
mod convert;
mod dataxfer;
mod flagop; // done
mod logical;
mod misc;
mod nop;
mod pop;
mod push;
mod ret;
//mod setcc;

impl<'ctx> Lifter for LifterX86<'ctx> {
    fn lift_instr(&self, instruction: FullInstruction) -> Result<(), BuilderError> {
        #[cfg(debug_assertions)]
        self.set_nop(&instruction.mnemonic)?;

        let fmt = zydis::Formatter::intel();
        match fmt.format(None, &instruction) {
            Ok(formatted) => println!("{formatted}"),
            Err(e) => eprintln!("Error: {e}"),
        };

        // Ignore control transfer for now
        if [
            zydis::InstructionCategory::CALL,
            zydis::InstructionCategory::COND_BR,
            zydis::InstructionCategory::UNCOND_BR,
        ]
        .contains(&instruction.meta.category)
        {
            return Ok(());
        }
        match instruction.mnemonic {
            // XADD is actualy semaphore class, but placed in binary
            Mnemonic::ADD | Mnemonic::XADD => self.lift_add(instruction)?,
            Mnemonic::BTC => self.lift_btc(instruction)?,
            Mnemonic::BTS => self.lift_bts(instruction)?,
            Mnemonic::CLC => self.lift_clc(instruction)?,
            Mnemonic::CLD => self.lift_cld(instruction)?,
            Mnemonic::CMC => self.lift_cmc(instruction)?,
            Mnemonic::CWD => self.lift_cwd(instruction)?,
            Mnemonic::LAHF => self.lift_lahf(instruction)?,
            Mnemonic::LEA => self.lift_lea(instruction)?,
            Mnemonic::MOV | Mnemonic::MOVSX | Mnemonic::MOVSXD | Mnemonic::MOVZX => {
                self.lift_mov(instruction)?
            }
            Mnemonic::OR => self.lift_or(instruction)?,
            Mnemonic::POP => self.lift_pop(instruction)?,
            Mnemonic::PUSH => self.lift_push(instruction)?,
            Mnemonic::RET => self.lift_ret(instruction)?,
            Mnemonic::SAHF => self.lift_sahf(instruction)?,
            Mnemonic::SALC => self.lift_salc(instruction)?,
            Mnemonic::STC => self.lift_stc(instruction)?,
            Mnemonic::STD => self.lift_std(instruction)?,
            Mnemonic::SUB => self.lift_sub(instruction)?,
            Mnemonic::XOR => self.lift_xor(instruction)?,
            // TODO: Replace it with unimplemented!()
            //_ => self.lift_nop(instruction)?,
            Mnemonic::NOP => self.lift_nop(instruction)?,
            _ => unimplemented!(
                "Mnemonic: '{}' is not implemented yet!",
                instruction.mnemonic
            ),
        }

        Ok(())
    }
}
