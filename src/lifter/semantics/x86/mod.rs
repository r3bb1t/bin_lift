use super::{LiftControl, Lifter, Result};
use crate::lifter::{Error, LifterX86};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::IntValue;
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

impl<'m, 'ctx> Lifter for LifterX86<'m, 'ctx> {
    fn lift_instr(&mut self, instr: &FullInstruction) -> Result<LiftControl> {
        self.increase_ip(instr.length);

        match instr.mnemonic {
            Mnemonic::ADC => self.lift_adc(instr)?,
            Mnemonic::SBB => self.lift_sbb(instr)?,
            Mnemonic::ADD | Mnemonic::SUB => self.lift_add_sub(instr)?,
            Mnemonic::CMP => self.lift_cmp(instr)?,
            Mnemonic::DEC => self.lift_dec(instr)?,
            Mnemonic::INC => self.lift_inc(instr)?,
            Mnemonic::NEG => self.lift_neg(instr)?,
            Mnemonic::NOT => self.lift_not(instr)?,
            Mnemonic::AND | Mnemonic::ANDN => self.lift_and_andn(instr)?,
            Mnemonic::OR => self.lift_or(instr)?,
            Mnemonic::TEST => self.lift_test(instr)?,
            Mnemonic::XOR => self.lift_xor(instr)?,
            Mnemonic::BSF => self.lift_bsf(instr)?,
            Mnemonic::BSR => self.lift_bsr(instr)?,
            Mnemonic::BT => self.lift_bt(instr)?,
            Mnemonic::BTC => self.lift_btc(instr)?,
            Mnemonic::BTR => self.lift_btr(instr)?,
            Mnemonic::BTS => self.lift_bts(instr)?,
            Mnemonic::CALL => self.lift_call(instr)?,
            Mnemonic::CMOVB => self.lift_cmovb(instr)?,
            Mnemonic::CMOVBE => self.lift_cmovbe(instr)?,
            Mnemonic::CMOVL => self.lift_cmovl(instr)?,
            Mnemonic::CMOVLE => self.lift_cmovle(instr)?,
            Mnemonic::CMOVNB => self.lift_cmovnb(instr)?,
            Mnemonic::CMOVNBE => self.lift_cmovnbe(instr)?,
            Mnemonic::CMOVNL => self.lift_cmovnl(instr)?,
            Mnemonic::CMOVNLE => self.lift_cmovnle(instr)?,
            Mnemonic::CMOVNO => self.lift_cmovno(instr)?,
            Mnemonic::CMOVNP => self.lift_cmovnp(instr)?,
            Mnemonic::CMOVNS => self.lift_cmovns(instr)?,
            Mnemonic::CMOVNZ => self.lift_cmovnz(instr)?,
            Mnemonic::CMOVO => self.lift_cmovo(instr)?,
            Mnemonic::CMOVP => self.lift_cmovp(instr)?,
            Mnemonic::CMOVS => self.lift_cmovs(instr)?,
            Mnemonic::CMOVZ => self.lift_cmovz(instr)?,
            Mnemonic::CBW => self.lift_cbw()?,
            Mnemonic::CDQ => self.lift_cdq()?,
            Mnemonic::CDQE => self.lift_cdqe()?,
            Mnemonic::CQO => self.lift_cqo()?,
            Mnemonic::CWD => self.lift_cwd()?,
            Mnemonic::CWDE => self.lift_cwde()?,
            Mnemonic::BSWAP => self.lift_bswap(instr)?,
            Mnemonic::MOV | Mnemonic::MOVSX | Mnemonic::MOVSXD | Mnemonic::MOVZX => {
                self.lift_mov(instr)?
            }
            Mnemonic::XCHG => self.lift_xchg(instr)?,
            Mnemonic::CMC
            | Mnemonic::CLC
            | Mnemonic::CLD
            | Mnemonic::LAHF
            | Mnemonic::SAHF
            | Mnemonic::SALC
            | Mnemonic::STC
            | Mnemonic::STD => {
                self.lift_flagop(instr.mnemonic)?;
            }
            Mnemonic::LEA => self.lift_lea(instr)?,
            Mnemonic::NOP => self.lift_nop()?,
            Mnemonic::POP => self.lift_pop(instr)?,
            Mnemonic::POPFQ => self.lift_popfq(instr)?,
            Mnemonic::PUSH => self.lift_push(instr)?,
            Mnemonic::PUSHFQ => self.lift_pushfq(instr)?,
            Mnemonic::RET => return self.lift_ret(instr),
            Mnemonic::RCL => self.lift_rcl(instr)?,
            Mnemonic::RCR => self.lift_rcr(instr)?,
            Mnemonic::ROL => self.lift_rol(instr)?,
            Mnemonic::ROR => self.lift_ror(instr)?,
            Mnemonic::XADD => self.lift_xadd(instr)?,
            Mnemonic::SETB => self.lift_setb(instr)?,
            Mnemonic::SETBE => self.lift_setbe(instr)?,
            Mnemonic::SETL => self.lift_setl(instr)?,
            Mnemonic::SETLE => self.lift_setle(instr)?,
            Mnemonic::SETNB => self.lift_setnb(instr)?,
            Mnemonic::SETNBE => self.lift_setnbe(instr)?,
            Mnemonic::SETNL => self.lift_setnl(instr)?,
            Mnemonic::SETNLE => self.lift_setnle(instr)?,
            Mnemonic::SETNO => self.lift_setno(instr)?,
            Mnemonic::SETNP => self.lift_setnp(instr)?,
            Mnemonic::SETNS => self.lift_setns(instr)?,
            Mnemonic::SETNZ => self.lift_setnz(instr)?,
            Mnemonic::SETO => self.lift_seto(instr)?,
            Mnemonic::SETP => self.lift_setp(instr)?,
            Mnemonic::SETS => self.lift_sets(instr)?,
            Mnemonic::SETZ => self.lift_setz(instr)?,
            Mnemonic::SAR | Mnemonic::SARX => self.lift_sar(instr)?,
            Mnemonic::SHL | Mnemonic::SHLX => self.lift_shl(instr)?,
            Mnemonic::SHR | Mnemonic::SHRX => self.lift_shr(instr)?,
            Mnemonic::SHLD => self.lift_shld(instr)?,
            Mnemonic::SHRD => self.lift_shrd(instr)?,
            Mnemonic::MOVSB | Mnemonic::MOVSD | Mnemonic::MOVSQ | Mnemonic::MOVSW => {
                self.lift_movs_x(instr)?
            }
            Mnemonic::RDTSC => self.lift_rdtsc()?,
            Mnemonic::JMP => self.lift_jmp(instr)?,
            _ => return Err(Error::UnsupportedInstr("unsupported instruction")),
        }

        Ok(LiftControl::Continue)
    }
}

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    fn load_bool_flag(&self, flag: ExtendedRegisterEnum) -> Result<IntValue<'ctx, bool>> {
        Ok(self.load_flag(flag)?.try_into()?)
    }

    fn bool_not(&self, value: IntValue<'ctx, bool>, name: &str) -> Result<IntValue<'ctx, bool>> {
        Ok(self.builder()?.build_icmp_eq::<bool, _, _, _>(
            value,
            self.module.bool_type().const_zero(),
            name,
        )?)
    }

    fn bool_and(
        &self,
        lhs: IntValue<'ctx, bool>,
        rhs: IntValue<'ctx, bool>,
        name: &str,
    ) -> Result<IntValue<'ctx, bool>> {
        Ok(self
            .builder()?
            .build_int_and::<bool, _, _, _>(lhs, rhs, name)?)
    }

    fn bool_or(
        &self,
        lhs: IntValue<'ctx, bool>,
        rhs: IntValue<'ctx, bool>,
        name: &str,
    ) -> Result<IntValue<'ctx, bool>> {
        Ok(self
            .builder()?
            .build_int_or::<bool, _, _, _>(lhs, rhs, name)?)
    }

    fn condition_for_mnemonic(&self, mnemonic: Mnemonic) -> Result<IntValue<'ctx, bool>> {
        let cf = || self.load_bool_flag(ExtendedRegisterEnum::CF);
        let zf = || self.load_bool_flag(ExtendedRegisterEnum::ZF);
        let sf = || self.load_bool_flag(ExtendedRegisterEnum::SF);
        let of = || self.load_bool_flag(ExtendedRegisterEnum::OF);
        let pf = || self.load_bool_flag(ExtendedRegisterEnum::PF);

        match mnemonic {
            Mnemonic::CMOVB | Mnemonic::SETB => cf(),
            Mnemonic::CMOVBE | Mnemonic::SETBE => self.bool_or(cf()?, zf()?, "cond_be"),
            Mnemonic::CMOVL | Mnemonic::SETL => Ok(self
                .builder()?
                .build_icmp_ne::<bool, _, _, _>(sf()?, of()?, "cond_l")?),
            Mnemonic::CMOVLE | Mnemonic::SETLE => {
                let less =
                    self.builder()?
                        .build_icmp_ne::<bool, _, _, _>(sf()?, of()?, "cond_l")?;
                self.bool_or(zf()?, less, "cond_le")
            }
            Mnemonic::CMOVNB | Mnemonic::SETNB => self.bool_not(cf()?, "cond_nb"),
            Mnemonic::CMOVNBE | Mnemonic::SETNBE => {
                let not_cf = self.bool_not(cf()?, "cond_nbe_cf")?;
                let not_zf = self.bool_not(zf()?, "cond_nbe_zf")?;
                self.bool_and(not_cf, not_zf, "cond_nbe")
            }
            Mnemonic::CMOVNL | Mnemonic::SETNL => Ok(self
                .builder()?
                .build_icmp_eq::<bool, _, _, _>(sf()?, of()?, "cond_nl")?),
            Mnemonic::CMOVNLE | Mnemonic::SETNLE => {
                let not_zf = self.bool_not(zf()?, "cond_nle_zf")?;
                let not_less = self.builder()?.build_icmp_eq::<bool, _, _, _>(
                    sf()?,
                    of()?,
                    "cond_nle_less",
                )?;
                self.bool_and(not_zf, not_less, "cond_nle")
            }
            Mnemonic::CMOVNO | Mnemonic::SETNO => self.bool_not(of()?, "cond_no"),
            Mnemonic::CMOVNP | Mnemonic::SETNP => self.bool_not(pf()?, "cond_np"),
            Mnemonic::CMOVNS | Mnemonic::SETNS => self.bool_not(sf()?, "cond_ns"),
            Mnemonic::CMOVNZ | Mnemonic::SETNZ => self.bool_not(zf()?, "cond_nz"),
            Mnemonic::CMOVO | Mnemonic::SETO => of(),
            Mnemonic::CMOVP | Mnemonic::SETP => pf(),
            Mnemonic::CMOVS | Mnemonic::SETS => sf(),
            Mnemonic::CMOVZ | Mnemonic::SETZ => zf(),
            _ => Err(Error::UnsupportedInstr("unsupported condition mnemonic")),
        }
    }
}
