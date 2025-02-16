use super::{Error, Lifter, Result};
use crate::lifter::LifterX86;

use zydis::{FullInstruction, Mnemonic};

mod binary;
mod bitbyte;
mod call;
mod cmov;
mod convert;
mod dataxfer;
mod flagop;
mod logical;
mod misc;
mod pop;
mod push;
mod ret;
mod rotate;
mod settcc;
mod shift;
mod stringop;
mod system;

impl Lifter for LifterX86<'_> {
    fn lift_instr(&self, instr: &FullInstruction) -> Result<()> {
        //// Ignore control transfer for now
        //if [
        //    zydis::InstructionCategory::CALL,
        //    zydis::InstructionCategory::COND_BR,
        //    zydis::InstructionCategory::UNCOND_BR,
        //    //InstructionCategory::RET,
        //]
        //.contains(&instr.meta.category)
        //{
        //    //eprintln!("Skipping {}", instruction.mnemonic);
        //    return Ok(());
        //}

        #[cfg(debug_assertions)]
        self.builder.build_alloca(
            self.context.i128_type(),
            &format!("debug_{}_", instr.mnemonic),
        )?;

        match instr.mnemonic {
            // binary
            Mnemonic::ADC => self.lift_adc(instr),
            Mnemonic::ADD | Mnemonic::XADD => self.lift_add(instr),
            Mnemonic::CMP => self.lift_cmp(instr),
            Mnemonic::DEC => self.lift_dec(instr),
            Mnemonic::INC => self.lift_inc(instr),
            Mnemonic::NEG => self.lift_neg(instr),
            Mnemonic::SBB => self.lift_sbb(instr),
            Mnemonic::SUB => self.lift_sub(instr),

            // bitbyte
            Mnemonic::BSR => self.lift_bsr(instr),
            Mnemonic::BSF => self.lift_bsf(instr),
            Mnemonic::BT => self.lift_bt(instr),
            Mnemonic::BTC => self.lift_btc(instr),
            Mnemonic::BTR => self.lift_btr(instr),
            Mnemonic::BTS => self.lift_bts(instr),

            // call
            Mnemonic::CALL => self.lift_call(instr),

            // cmov
            Mnemonic::CMOVB => self.lift_cmovb(instr),
            Mnemonic::CMOVBE => self.lift_cmovbe(instr),
            Mnemonic::CMOVL => self.lift_cmovl(instr),
            Mnemonic::CMOVLE => self.lift_cmovle(instr),
            Mnemonic::CMOVNB => self.lift_cmovnb(instr),
            Mnemonic::CMOVNBE => self.lift_cmovnbe(instr),
            Mnemonic::CMOVNL => self.lift_cmovnl(instr),
            Mnemonic::CMOVNLE => self.lift_cmovnle(instr),
            Mnemonic::CMOVNO => self.lift_cmovno(instr),
            Mnemonic::CMOVNP => self.lift_cmovnp(instr),
            Mnemonic::CMOVNS => self.lift_cmovns(instr),
            Mnemonic::CMOVNZ => self.lift_cmovnz(instr),
            Mnemonic::CMOVO => self.lift_cmovo(instr),
            Mnemonic::CMOVP => self.lift_cmovp(instr),
            Mnemonic::CMOVS => self.lift_cmovs(instr),
            Mnemonic::CMOVZ => self.lift_cmovz(instr),

            // convert
            Mnemonic::CBW => self.lift_cbw(),
            Mnemonic::CDQ => self.lift_cdq(),
            Mnemonic::CDQE => self.lift_cdqe(),
            Mnemonic::CQO => self.lift_cqo(),
            Mnemonic::CWD => self.lift_cwd(),
            Mnemonic::CWDE => self.lift_cwde(),

            // dataxfer
            Mnemonic::BSWAP => self.lift_bswap(instr),
            Mnemonic::MOVZX | Mnemonic::MOVSX | Mnemonic::MOVSXD | Mnemonic::MOV => {
                self.lift_mov(instr)
            }
            Mnemonic::XCHG => self.lift_xchg(instr),

            // logical
            Mnemonic::AND | Mnemonic::ANDN => self.lift_and_andn(instr),
            Mnemonic::NOT => self.lift_not(instr),
            Mnemonic::OR => self.lift_or(instr),
            Mnemonic::TEST => self.lift_test(instr),
            Mnemonic::XOR => self.lift_xor(instr),

            // flagop
            Mnemonic::CMC => self.lift_cmc(),
            Mnemonic::CLC => self.lift_clc(),
            Mnemonic::CLD => self.lift_cld(),
            Mnemonic::STC => self.lift_stc(),
            Mnemonic::STD => self.lift_std(),
            Mnemonic::SALC => self.lift_salc(),
            Mnemonic::LAHF => self.lift_lahf(),
            Mnemonic::SAHF => self.lift_sahf(),

            // misc
            Mnemonic::LEA => self.lift_lea(instr),

            // nop
            Mnemonic::NOP => Ok(()),

            // pop
            Mnemonic::POP => self.lift_pop(instr),
            Mnemonic::POPFQ => self.lift_popfq(instr),

            // push
            Mnemonic::PUSH => self.lift_push(instr),
            Mnemonic::PUSHFQ => self.lift_pushfq(instr),

            // ret
            Mnemonic::RET => self.lift_ret(instr),

            // rotate
            Mnemonic::RCL => self.lift_rcl(instr),
            Mnemonic::RCR => self.lift_rcr(instr),
            Mnemonic::ROL => self.lift_rol(instr),
            Mnemonic::ROR => self.lift_ror(instr),

            // setcc
            Mnemonic::SETB => self.lift_setb(instr),
            Mnemonic::SETBE => self.lift_setbe(instr),
            Mnemonic::SETL => self.lift_setl(instr),
            Mnemonic::SETLE => self.lift_setle(instr),
            Mnemonic::SETNB => self.lift_setnb(instr),
            Mnemonic::SETNBE => self.lift_setnbe(instr),
            Mnemonic::SETNL => self.lift_setnl(instr),
            Mnemonic::SETNLE => self.lift_setnle(instr),
            Mnemonic::SETNO => self.lift_setno(instr),
            Mnemonic::SETNP => self.lift_setnp(instr),
            Mnemonic::SETNS => self.lift_setns(instr),
            Mnemonic::SETNZ => self.lift_setnz(instr),
            Mnemonic::SETO => self.lift_seto(instr),
            Mnemonic::SETP => self.lift_setp(instr),
            Mnemonic::SETS => self.lift_sets(instr),
            Mnemonic::SETZ => self.lift_setz(instr),

            // Shift
            // NOTE: checked
            Mnemonic::SAR | Mnemonic::SARX => self.lift_sar(instr),
            Mnemonic::SHL | Mnemonic::SHLX => self.lift_shl(instr),
            Mnemonic::SHLD => self.lift_shld(instr),
            Mnemonic::SHR | Mnemonic::SHRX => self.lift_shr(instr),
            Mnemonic::SHRD => self.lift_shrd(instr),

            // Stringop
            Mnemonic::MOVSB | Mnemonic::MOVSW | Mnemonic::MOVSD | Mnemonic::MOVSQ => {
                self.lift_movs_x(instr)
            }

            // System
            Mnemonic::RDTSC => self.lift_rdtsc(),

            // TODO: Add  flagops
            //_ => unimplemented!("{} isn't implemented yet", instruction.mnemonic),
            _ => Err(Error::UnsupportedInstr("Instruction isnt implemented")),
        }
    }
}
