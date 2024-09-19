/// This module contains various contexts for starting the simulation(?) by providing CPU context for
/// the lifter
use std::collections::HashMap;

use inkwell::{
    context::Context,
    values::{BasicValue, BasicValueEnum, IntValue},
};
use zydis::{MachineMode, Register};

use crate::{miscellaneous::ExtendedRegister, util::get_int_type};

/// Trait for defining your CPU context for simulation
pub trait CpuContext {
    fn create_variables(
        self,
        context: &Context,
        mode: MachineMode,
    ) -> HashMap<ExtendedRegister, BasicValueEnum<'_>>;
}

/// Marker trait for x86 cpu contexts
pub trait SupportedIntTypesX86 {}

impl SupportedIntTypesX86 for u32 {}
impl SupportedIntTypesX86 for u64 {}

/// Context for x86 CPUs. Accepts both 64 and 32 bit variables
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StartContextX86<I: SupportedIntTypesX86> {
    // General purpose registers
    pub rax: I,
    pub rbx: I,
    pub rcx: I,
    pub rdx: I,
    pub rsi: I,
    pub rdi: I,
    pub rbp: I,
    pub rsp: I,
    pub r8: I,
    pub r9: I,
    pub r10: I,
    pub r11: I,
    pub r12: I,
    pub r13: I,
    pub r14: I,
    pub r15: I,

    // Instruction pointer (may remove this in future)
    pub rip: I,

    // Flags
    pub cf: u8,
    pub pf: u8,
    pub af: u8,
    pub zf: u8,
    pub sf: u8,
    pub tf: u8,
    pub r#if: u8,
    pub df: u8,
    pub of: u8,
    pub iopl: u8,
    pub nt: u8,
    pub rf: u8,
    pub vm: u8,
    pub ac: u8,
    pub vif: u8,
    pub vip: u8,
    pub id: u8,
}

impl<I: SupportedIntTypesX86> CpuContext for StartContextX86<I>
where
    u64: From<I>,
{
    fn create_variables(
        self,
        context: &Context,
        mode: MachineMode,
    ) -> HashMap<ExtendedRegister, BasicValueEnum<'_>> {
        let int_type = get_int_type(context, &Register::AX.largest_enclosing(mode), &mode);
        let mut regs_hashmap: HashMap<ExtendedRegister, IntValue> = HashMap::new();

        let rax = int_type.const_int(self.rax.into(), false);
        let rbx = int_type.const_int(self.rbx.into(), false);
        let rcx = int_type.const_int(self.rcx.into(), false);
        let rdx = int_type.const_int(self.rdx.into(), false);
        let rsi = int_type.const_int(self.rsi.into(), false);
        let rdi = int_type.const_int(self.rdi.into(), false);
        let rbp = int_type.const_int(self.rbp.into(), false);
        let rsp = int_type.const_int(self.rsp.into(), false);
        let r8 = int_type.const_int(self.r8.into(), false);
        let r9 = int_type.const_int(self.r9.into(), false);
        let r10 = int_type.const_int(self.r10.into(), false);
        let r11 = int_type.const_int(self.r11.into(), false);
        let r12 = int_type.const_int(self.r12.into(), false);
        let r13 = int_type.const_int(self.r13.into(), false);
        let r14 = int_type.const_int(self.r14.into(), false);
        let r15 = int_type.const_int(self.r15.into(), false);
        let rip = int_type.const_int(self.rip.into(), false);

        let cf = context.i8_type().const_int(self.cf.into(), false);
        let pf = context.i8_type().const_int(self.pf.into(), false);
        let af = context.i8_type().const_int(self.af.into(), false);
        let zf = context.i8_type().const_int(self.zf.into(), false);
        let sf = context.i8_type().const_int(self.sf.into(), false);
        let tf = context.i8_type().const_int(self.tf.into(), false);
        let r#if = context.i8_type().const_int(self.r#if.into(), false);
        let df = context.i8_type().const_int(self.df.into(), false);
        let of = context.i8_type().const_int(self.of.into(), false);
        let iopl = context.i8_type().const_int(self.iopl.into(), false);
        let nt = context.i8_type().const_int(self.nt.into(), false);
        let rf = context.i8_type().const_int(self.rf.into(), false);
        let vm = context.i8_type().const_int(self.vm.into(), false);
        let ac = context.i8_type().const_int(self.ac.into(), false);
        let vif = context.i8_type().const_int(self.vif.into(), false);
        let vip = context.i8_type().const_int(self.vip.into(), false);
        let id = context.i8_type().const_int(self.id.into(), false);

        regs_hashmap.insert(Register::AX.largest_enclosing(mode).into(), rax);
        regs_hashmap.insert(Register::BX.largest_enclosing(mode).into(), rbx);
        regs_hashmap.insert(Register::CX.largest_enclosing(mode).into(), rcx);
        regs_hashmap.insert(Register::DX.largest_enclosing(mode).into(), rdx);
        regs_hashmap.insert(Register::SI.largest_enclosing(mode).into(), rsi);
        regs_hashmap.insert(Register::DI.largest_enclosing(mode).into(), rdi);
        regs_hashmap.insert(Register::BP.largest_enclosing(mode).into(), rbp);
        regs_hashmap.insert(Register::SP.largest_enclosing(mode).into(), rsp);
        regs_hashmap.insert(Register::R8B.largest_enclosing(mode).into(), r8);
        regs_hashmap.insert(Register::R9B.largest_enclosing(mode).into(), r9);
        regs_hashmap.insert(Register::R10B.largest_enclosing(mode).into(), r10);
        regs_hashmap.insert(Register::R11B.largest_enclosing(mode).into(), r11);
        regs_hashmap.insert(Register::R12B.largest_enclosing(mode).into(), r12);
        regs_hashmap.insert(Register::R13B.largest_enclosing(mode).into(), r13);
        regs_hashmap.insert(Register::R14B.largest_enclosing(mode).into(), r14);
        regs_hashmap.insert(Register::R15B.largest_enclosing(mode).into(), r15);
        regs_hashmap.insert(Register::RIP.largest_enclosing(mode).into(), rip);

        regs_hashmap.insert(ExtendedRegister::CF, cf);
        regs_hashmap.insert(ExtendedRegister::PF, pf);
        regs_hashmap.insert(ExtendedRegister::AF, af);
        regs_hashmap.insert(ExtendedRegister::ZF, zf);
        regs_hashmap.insert(ExtendedRegister::SF, sf);
        regs_hashmap.insert(ExtendedRegister::TF, tf);
        regs_hashmap.insert(ExtendedRegister::IF, r#if);
        regs_hashmap.insert(ExtendedRegister::DF, df);
        regs_hashmap.insert(ExtendedRegister::OF, of);
        regs_hashmap.insert(ExtendedRegister::IOPL, iopl);
        regs_hashmap.insert(ExtendedRegister::NT, nt);
        regs_hashmap.insert(ExtendedRegister::RF, rf);
        regs_hashmap.insert(ExtendedRegister::VM, vm);
        regs_hashmap.insert(ExtendedRegister::AC, ac);
        regs_hashmap.insert(ExtendedRegister::VIF, vif);
        regs_hashmap.insert(ExtendedRegister::VIP, vip);
        regs_hashmap.insert(ExtendedRegister::ID, id);

        let mut resulting_hashmap: HashMap<ExtendedRegister, BasicValueEnum<'_>> = HashMap::new();
        for (k, v) in regs_hashmap {
            resulting_hashmap.insert(k, v.as_basic_value_enum());
        }

        resulting_hashmap
    }
}

#[cfg(test)]
mod tests {

    use inkwell::context::Context;
    use zydis::MachineMode;

    use super::{CpuContext, StartContextX86};

    #[test]
    fn create_sample_context() {
        let context = Context::create();
        let x86_ctx: StartContextX86<u32> = StartContextX86 {
            rax: u32::MAX,
            rbx: 777,
            rcx: 333,
            ..Default::default()
        };

        let vars = x86_ctx.create_variables(&context, MachineMode::LONG_64);
        dbg!(vars);
    }
}
