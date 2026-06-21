use super::Result;
use crate::lifter::LifterX86;

use llvmkit::ir::{IntDyn, IntValue};
use zydis::Register;

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_cbw(&mut self) -> Result<()> {
        self.convert_sign_extend_register(Register::AL, Register::AX, 8, 16)
    }

    pub(super) fn lift_cwde(&mut self) -> Result<()> {
        self.convert_sign_extend_register(Register::AX, Register::EAX, 16, 32)
    }

    pub(super) fn lift_cdqe(&mut self) -> Result<()> {
        self.convert_sign_extend_register(Register::EAX, Register::RAX, 32, 64)
    }

    pub(super) fn lift_cwd(&mut self) -> Result<()> {
        self.convert_sign_fill_high_register(Register::AX, Register::DX, 16)
    }

    pub(super) fn lift_cdq(&mut self) -> Result<()> {
        self.convert_sign_fill_high_register(Register::EAX, Register::EDX, 32)
    }

    pub(super) fn lift_cqo(&mut self) -> Result<()> {
        self.convert_sign_fill_high_register(Register::RAX, Register::RDX, 64)
    }

    fn convert_sign_extend_register(
        &mut self,
        src: Register,
        dest: Register,
        src_bits: u32,
        dest_bits: u32,
    ) -> Result<()> {
        let src_ty = self.module.custom_width_int_type(src_bits)?;
        let dest_ty = self.module.custom_width_int_type(dest_bits)?;
        let src_value: IntValue<'ctx, IntDyn> = self.load_register_value(&src)?.try_into()?;
        let src_value = self.create_z_ext_or_trunc(src_value, src_ty)?;
        let value = self
            .builder()?
            .build_sext_dyn(src_value, dest_ty, "sign_extend")?;

        self.store_reg(dest, value)
    }

    fn convert_sign_fill_high_register(
        &mut self,
        src: Register,
        dest: Register,
        bits: u32,
    ) -> Result<()> {
        let ty = self.module.custom_width_int_type(bits)?;
        let src_value: IntValue<'ctx, IntDyn> = self.load_register_value(&src)?.try_into()?;
        let src_value = self.create_z_ext_or_trunc(src_value, ty)?;
        let sign = self.builder()?.build_icmp_slt::<IntDyn, _, _, _>(
            src_value,
            ty.const_zero(),
            "sign_fill",
        )?;
        let all_ones: IntValue<'ctx, IntDyn> = ty.const_all_ones().as_value().try_into()?;
        let zero: IntValue<'ctx, IntDyn> = ty.const_zero().as_value().try_into()?;
        let value = self
            .builder()?
            .build_select(sign, all_ones, zero, "high_sign")?;

        self.store_reg(dest, value)
    }
}
