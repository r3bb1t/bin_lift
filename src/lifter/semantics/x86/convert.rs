use super::{LifterX86, Result};

use inkwell::{values::IntValue, IntPredicate};
use zydis::Register;

impl LifterX86<'_> {
    pub(super) fn lift_cbw(&self) -> Result<()> {
        let al: IntValue<'_> = self.load_reg_internal(&Register::AL)?.try_into()?;
        let ax = self
            .builder
            .build_int_s_extend(al, self.context.i16_type(), "")?;

        self.store_register_internal(Register::AX, ax);

        Ok(())
    }

    pub(super) fn lift_cdq(&self) -> Result<()> {
        let builder = &self.builder;

        let eax: IntValue<'_> = self.load_reg_internal(&Register::EAX)?.try_into()?;
        let sign_bit = self.compute_sign_flag(eax)?;

        let i32_ty = self.context.i32_type();

        let edx = builder
            .build_select(
                builder.build_int_compare(
                    IntPredicate::EQ,
                    sign_bit,
                    sign_bit.get_type().const_zero(),
                    "",
                )?,
                i32_ty.const_zero(),
                // hopefully wont be a bug
                i32_ty.const_all_ones(),
                "",
            )?
            .into_int_value();

        self.store_register_internal(Register::EDX, edx);

        Ok(())
    }

    pub(super) fn lift_cdqe(&self) -> Result<()> {
        let eax: IntValue<'_> = self.load_reg_internal(&Register::EAX)?.try_into()?;

        let rax = self
            .builder
            .build_int_z_extend(eax, self.context.i64_type(), "cdqe")?;

        self.store_register_internal(Register::RAX, rax);
        Ok(())
    }

    pub(super) fn lift_cqo(&self) -> Result<()> {
        let builder = &self.builder;

        let eax: IntValue<'_> = self.load_reg_internal(&Register::RAX)?.try_into()?;
        let sign_bit = self.compute_sign_flag(eax)?;

        let i64_ty = self.context.i64_type();

        let edx = builder
            .build_select(
                builder.build_int_compare(
                    IntPredicate::EQ,
                    sign_bit,
                    sign_bit.get_type().const_zero(),
                    "",
                )?,
                i64_ty.const_zero(),
                // hopefully wont be a bug
                i64_ty.const_all_ones(),
                "",
            )?
            .into_int_value();

        self.store_register_internal(Register::RDX, edx);

        Ok(())
    }

    pub(super) fn lift_cwd(&self) -> Result<()> {
        //let op0 = self.load_reg(&Register::RAX)?.into_int_value();
        //let op0 = self
        //    .retdec_load_reg(&Register::RAX, None, None)?
        //    .unwrap()
        //    .into_int_value();

        let op0: IntValue<'_> = self.get_register(Register::RAX)?.try_into()?;
        // TODO: Check if second arg is correct
        let e = self.builder.build_right_shift(
            op0,
            op0.get_type()
                .const_int((Register::RAX.width(self.mode) - 1).into(), false),
            false,
            "",
        )?;

        //self.store_reg(Register::DX, e.as_basic_value_enum());
        self.store_register_internal(Register::DX, e);
        Ok(())
    }

    pub(super) fn lift_cwde(&self) -> Result<()> {
        let ax: IntValue<'_> = self.load_reg_internal(&Register::RAX)?.try_into()?;

        let dst_ty = self.get_max_int_type();
        let val = self
            .builder
            //.build_int_s_extend(ax, self.context.i32_type(), "cwde")?;
            .build_int_s_extend(ax, dst_ty, "cwde")?;

        self.store_register_internal(Register::RAX, val);
        Ok(())
    }
}
