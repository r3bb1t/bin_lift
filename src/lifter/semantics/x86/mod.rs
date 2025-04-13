use super::{Error, Lifter, Result};
use crate::lifter::LifterX86;

use zydis::{FullInstruction, Mnemonic};

mod binary;
mod bitbyte;
mod call;
mod cmov;
mod cond_br;
mod convert;
mod dataxfer;
mod flagop;
mod logical;
mod misc;
mod nop;
mod pop;
mod push;
mod ret;
mod rotate;
mod semaphore;
mod setcc;
mod shift;
mod stringop;
mod system;
mod uncond_br;

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

        self.increase_ip(instr.length);

        #[cfg(debug_assertions)]
        {
            let fmt = zydis::Formatter::intel();
            let formatted = fmt
                .format(None, instr)
                .unwrap_or("unformattable".to_string());
            // FIXME: Ip tracking isn't implemented yet

            let runtime_address_option = self.runtime_address();
            let disassembly = format!("{runtime_address_option:<24X?} {} ; ", formatted);
            println!("{disassembly}");
            self.builder
                .build_alloca(self.context.i128_type(), &disassembly)?;
        }

        match instr.mnemonic {
            // binary
            // NOTE: checked
            Mnemonic::ADC => self.lift_adc(instr),
            Mnemonic::ADD | Mnemonic::SUB => self.lift_add_sub(instr),
            Mnemonic::CMP => self.lift_cmp(instr),
            Mnemonic::DEC => self.lift_dec(instr),
            Mnemonic::INC => self.lift_inc(instr),
            Mnemonic::NEG => self.lift_neg(instr),
            Mnemonic::SBB => self.lift_sbb(instr),
            //Mnemonic::SUB => self.lift_sub(instr),

            // bitbyte
            // NOTE: checked
            Mnemonic::BSR => self.lift_bsr(instr),
            Mnemonic::BSF => self.lift_bsf(instr),
            Mnemonic::BT => self.lift_bt(instr),
            Mnemonic::BTC => self.lift_btc(instr),
            Mnemonic::BTR => self.lift_btr(instr),
            Mnemonic::BTS => self.lift_bts(instr),

            // call
            Mnemonic::CALL => self.lift_call(instr),

            // cmov
            // NOTE: checked
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
            // NOTE: checked
            Mnemonic::CBW => self.lift_cbw(),
            Mnemonic::CDQ => self.lift_cdq(),
            Mnemonic::CDQE => self.lift_cdqe(),
            Mnemonic::CQO => self.lift_cqo(),
            Mnemonic::CWD => self.lift_cwd(),
            Mnemonic::CWDE => self.lift_cwde(),

            // dataxfer
            // NOTE: checked
            Mnemonic::BSWAP => self.lift_bswap(instr),
            Mnemonic::MOVZX | Mnemonic::MOVSX | Mnemonic::MOVSXD | Mnemonic::MOV => {
                self.lift_mov(instr)
            }
            Mnemonic::XCHG => self.lift_xchg(instr),

            // logical
            // NOTE: checked
            Mnemonic::AND | Mnemonic::ANDN => self.lift_and_andn(instr),
            Mnemonic::NOT => self.lift_not(instr),
            Mnemonic::OR => self.lift_or(instr),
            Mnemonic::TEST => self.lift_test(instr),
            Mnemonic::XOR => self.lift_xor(instr),

            // flagop
            // NOTE: checked
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
            Mnemonic::NOP => self.lift_nop(),

            // pop
            // NOTE: checked
            Mnemonic::POP => self.lift_pop(instr),
            Mnemonic::POPFQ => self.lift_popfq(instr),

            // push
            // NOTE: checked
            Mnemonic::PUSH => self.lift_push(instr),
            Mnemonic::PUSHFQ => self.lift_pushfq(instr),

            // ret
            // NOTE: NOT FULLY checked (didn't do the solving of path)
            Mnemonic::RET => self.lift_ret(instr),

            // rotate
            // NOTE: checked
            Mnemonic::RCL => self.lift_rcl(instr),
            Mnemonic::RCR => self.lift_rcr(instr),
            Mnemonic::ROL => self.lift_rol(instr),
            Mnemonic::ROR => self.lift_ror(instr),

            // semaphore
            Mnemonic::XADD => self.lift_xadd(instr),

            // setcc
            // NOTE: checked
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

            // stringop
            // NOTE: checked
            Mnemonic::MOVSB | Mnemonic::MOVSW | Mnemonic::MOVSD | Mnemonic::MOVSQ => {
                self.lift_movs_x(instr)
            }

            // system
            Mnemonic::RDTSC => self.lift_rdtsc(),

            // uncond_br
            Mnemonic::JMP => self.lift_jmp(instr),

            // TODO: Add  flagops
            //_ => unimplemented!("{} isn't implemented yet", instruction.mnemonic),
            _ => Err(Error::UnsupportedInstr("Instruction isnt implemented")),
        }
    }
}
