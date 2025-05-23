use crate::miscellaneous::ExtendedRegisterEnum;

use super::{LifterX86, Result};

use inkwell::{values::IntValue, IntPredicate};
use zydis::Register;

impl LifterX86<'_> {
    pub(super) fn lift_cbw(&self) -> Result<()> {
        let al: IntValue<'_> = self.load_register_value(&Register::AL)?.try_into()?;
        let al_8_bit = self.create_z_ext_or_trunc(al, self.context.i8_type())?;

        let ax = self
            .builder
            .build_int_s_extend(al_8_bit, self.context.i16_type(), "ax_cbw")?;

        self.store_reg(Register::AX, ax)?;

        Ok(())
    }

    pub(super) fn lift_cdq(&self) -> Result<()> {
        let builder = &self.builder;
        let i32_ty = self.context.i32_type();

        let eax: IntValue<'_> = self.load_register_value(&Register::EAX)?.try_into()?;
        let eax_z_ext_or_trunc = self.create_z_ext_or_trunc(eax, i32_ty)?;

        let sign_bit = self.compute_sign_flag(eax_z_ext_or_trunc)?;

        let edx = builder
            .build_select(
                builder.build_int_compare(
                    IntPredicate::EQ,
                    sign_bit,
                    sign_bit.get_type().const_zero(),
                    "",
                )?,
                i32_ty.const_zero(),
                i32_ty.const_int(0xFFFFFFFF, false),
                "cdq_edx_",
            )?
            .into_int_value();

        self.store_reg(Register::EDX, edx)?;

        Ok(())
    }

    pub(super) fn lift_cdqe(&self) -> Result<()> {
        let eax: IntValue<'_> = self.load_register_value(&Register::EAX)?.try_into()?;
        let eax_z_ext_or_trunc = self.create_z_ext_or_trunc(eax, self.context.i32_type())?;

        let rax =
            self.builder
                .build_int_s_extend(eax_z_ext_or_trunc, self.context.i64_type(), "cdqe")?;

        self.store_reg(Register::RAX, rax)?;
        Ok(())
    }

    pub(super) fn lift_cqo(&self) -> Result<()> {
        let builder = &self.builder;
        let i64_ty = self.context.i64_type();

        let rax: IntValue<'_> = self.load_register_value(&Register::RAX)?.try_into()?;
        let rax_z_ext_or_trunc = self.create_z_ext_or_trunc(rax, i64_ty)?;
        let sign_bit = self.compute_sign_flag(rax_z_ext_or_trunc)?;

        let rdx = builder
            .build_select(
                builder.build_int_compare(
                    IntPredicate::EQ,
                    sign_bit,
                    sign_bit.get_type().const_zero(),
                    "",
                )?,
                i64_ty.const_zero(),
                // hopefully wont be a bug
                i64_ty.const_int(0xFFFFFFFFFFFFFFFF, false),
                "cqo",
            )?
            .into_int_value();

        self.store_reg(Register::RDX, rdx)?;

        Ok(())
    }

    pub(super) fn lift_cwd(&self) -> Result<()> {
        let builder = &self.builder;
        let i_16_ty = self.context.i16_type();

        let ax: IntValue<'_> = self.load_register_value(&Register::RAX)?.try_into()?;

        let ax_16_bit = self.create_z_ext_or_trunc(ax, i_16_ty)?;
        let sign_bit = self.compute_sign_flag(ax_16_bit)?;

        let sign_bit_ty = sign_bit.get_type();

        let dx = builder
            .build_select(
                builder.build_int_compare(
                    IntPredicate::EQ,
                    sign_bit,
                    sign_bit_ty.const_zero(),
                    "",
                )?,
                i_16_ty.const_zero(),
                i_16_ty.const_int(0xFFFF, false),
                "set_dx_",
            )?
            .into_int_value();

        self.store_reg(ExtendedRegisterEnum::DX.into(), dx)?;

        Ok(())
    }

    pub(super) fn lift_cwde(&self) -> Result<()> {
        let ax: IntValue<'_> = self.load_register_value(&Register::RAX)?.try_into()?;
        let ax = self.create_z_ext_or_trunc(ax, self.context.i16_type())?;

        //let dst_ty = self.get_max_int_type();
        let val = self
            .builder
            .build_int_s_extend(ax, self.context.i32_type(), "cwde")?;
        //.build_int_s_extend(ax, dst_ty, "cwde")?;

        self.store_reg(Register::RAX, val)?;
        Ok(())
    }
}
