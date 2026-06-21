/// CPU context helpers for seeding the lifter register map.
use std::collections::HashMap;

use llvmkit::ir::{IntDyn, IntType, IntValue, Module};
use zydis::{MachineMode, Register};

use crate::lifter::{LiftValue, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

/// Trait for defining a CPU context for simulation.
pub(super) trait CpuContext {
    fn create_variables<'ctx>(
        self,
        module: &Module<'ctx>,
        mode: MachineMode,
    ) -> Result<HashMap<ExtendedRegisterEnum, LiftValue<'ctx>>>;
}

/// Marker trait for x86 CPU contexts.
pub(super) trait SupportedIntTypesX86 {}

impl SupportedIntTypesX86 for u32 {}
impl SupportedIntTypesX86 for u64 {}

/// Context for x86 CPUs. Accepts both 64 and 32 bit variables.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct StartContextX86<I: SupportedIntTypesX86> {
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
    pub rip: I,
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
    fn create_variables<'ctx>(
        self,
        module: &Module<'ctx>,
        mode: MachineMode,
    ) -> Result<HashMap<ExtendedRegisterEnum, LiftValue<'ctx>>> {
        let int_type = module
            .custom_width_int_type(Register::AX.largest_enclosing(mode).width(mode).into())?;
        let bool_type = module.bool_type().as_dyn();
        let mut regs_hashmap = HashMap::new();

        fn int_value<'ctx>(
            ty: IntType<'ctx, IntDyn>,
            value: u64,
        ) -> Result<IntValue<'ctx, IntDyn>> {
            Ok(ty.const_int_raw(value, false)?.as_value().try_into()?)
        }

        fn bool_value<'ctx>(
            ty: IntType<'ctx, IntDyn>,
            value: u8,
        ) -> Result<IntValue<'ctx, IntDyn>> {
            Ok(ty
                .const_int_raw(u64::from(value != 0), false)?
                .as_value()
                .try_into()?)
        }

        regs_hashmap.insert(
            Register::AX.largest_enclosing(mode).into(),
            int_value(int_type, self.rax.into())?.into(),
        );
        regs_hashmap.insert(
            Register::BX.largest_enclosing(mode).into(),
            int_value(int_type, self.rbx.into())?.into(),
        );
        regs_hashmap.insert(
            Register::CX.largest_enclosing(mode).into(),
            int_value(int_type, self.rcx.into())?.into(),
        );
        regs_hashmap.insert(
            Register::DX.largest_enclosing(mode).into(),
            int_value(int_type, self.rdx.into())?.into(),
        );
        regs_hashmap.insert(
            Register::SI.largest_enclosing(mode).into(),
            int_value(int_type, self.rsi.into())?.into(),
        );
        regs_hashmap.insert(
            Register::DI.largest_enclosing(mode).into(),
            int_value(int_type, self.rdi.into())?.into(),
        );
        regs_hashmap.insert(
            Register::BP.largest_enclosing(mode).into(),
            int_value(int_type, self.rbp.into())?.into(),
        );
        regs_hashmap.insert(
            Register::SP.largest_enclosing(mode).into(),
            int_value(int_type, self.rsp.into())?.into(),
        );
        regs_hashmap.insert(
            Register::R8B.largest_enclosing(mode).into(),
            int_value(int_type, self.r8.into())?.into(),
        );
        regs_hashmap.insert(
            Register::R9B.largest_enclosing(mode).into(),
            int_value(int_type, self.r9.into())?.into(),
        );
        regs_hashmap.insert(
            Register::R10B.largest_enclosing(mode).into(),
            int_value(int_type, self.r10.into())?.into(),
        );
        regs_hashmap.insert(
            Register::R11B.largest_enclosing(mode).into(),
            int_value(int_type, self.r11.into())?.into(),
        );
        regs_hashmap.insert(
            Register::R12B.largest_enclosing(mode).into(),
            int_value(int_type, self.r12.into())?.into(),
        );
        regs_hashmap.insert(
            Register::R13B.largest_enclosing(mode).into(),
            int_value(int_type, self.r13.into())?.into(),
        );
        regs_hashmap.insert(
            Register::R14B.largest_enclosing(mode).into(),
            int_value(int_type, self.r14.into())?.into(),
        );
        regs_hashmap.insert(
            Register::R15B.largest_enclosing(mode).into(),
            int_value(int_type, self.r15.into())?.into(),
        );
        regs_hashmap.insert(
            Register::IP.largest_enclosing(mode).into(),
            int_value(int_type, self.rip.into())?.into(),
        );

        for (flag, value) in [
            (ExtendedRegisterEnum::CF, self.cf),
            (ExtendedRegisterEnum::PF, self.pf),
            (ExtendedRegisterEnum::AF, self.af),
            (ExtendedRegisterEnum::ZF, self.zf),
            (ExtendedRegisterEnum::SF, self.sf),
            (ExtendedRegisterEnum::TF, self.tf),
            (ExtendedRegisterEnum::IF, self.r#if),
            (ExtendedRegisterEnum::DF, self.df),
            (ExtendedRegisterEnum::OF, self.of),
            (ExtendedRegisterEnum::IOPL, self.iopl),
            (ExtendedRegisterEnum::NT, self.nt),
            (ExtendedRegisterEnum::RF, self.rf),
            (ExtendedRegisterEnum::VM, self.vm),
            (ExtendedRegisterEnum::AC, self.ac),
            (ExtendedRegisterEnum::VIF, self.vif),
            (ExtendedRegisterEnum::VIP, self.vip),
            (ExtendedRegisterEnum::ID, self.id),
        ] {
            regs_hashmap.insert(flag, bool_value(bool_type, value)?.into());
        }

        Ok(regs_hashmap)
    }
}

#[cfg(test)]
mod tests {
    use llvmkit::ir::Module;
    use zydis::{MachineMode, Register};

    use super::{CpuContext, StartContextX86};

    #[test]
    fn create_sample_context() -> Result<(), Box<dyn std::error::Error>> {
        Module::with_new("ctx", |module| {
            let x86_ctx: StartContextX86<u32> = StartContextX86 {
                rax: u32::MAX,
                rbx: 777,
                rcx: 333,
                ..Default::default()
            };

            assert_eq!(1110, x86_ctx.rbx + x86_ctx.rcx);
            let vars = x86_ctx.create_variables(&module, MachineMode::LONG_64)?;
            assert!(vars.contains_key(&Register::RBX.into()));
            Ok::<_, Box<dyn std::error::Error>>(())
        })
    }
}
