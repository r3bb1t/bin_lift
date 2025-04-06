use super::{definintions::PossibleLLVMValueEnum, Error, ExtendedRegisterEnum, LifterX86, Result};

use inkwell::{intrinsics::Intrinsic, values::IntValue, IntPredicate};

const CTPOP_INSTRINSIC: &str = "llvm.ctpop";

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn compute_aux_flag(
        &self,
        lhs: IntValue<'ctx>,
        rhs: IntValue<'ctx>,
        result: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let aux_c = result.get_type().const_int(0x10, false);

        let aux_1 = builder.build_xor(result, builder.build_xor(lhs, rhs, "")?, "aux_1_")?;
        let aux_2 = builder.build_and(aux_c, aux_1, "aux_2_")?;
        let af = builder.build_int_compare(IntPredicate::EQ, aux_2, aux_1, "computed_af_")?;
        Ok(af)
    }
    pub(super) fn compute_sign_flag(&self, value: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
        let sf = self.builder.build_int_compare(
            IntPredicate::SLT,
            value,
            value.get_type().const_zero(),
            "sign_flag_",
        )?;
        Ok(sf)
    }

    pub(super) fn compute_zero_flag(&self, value: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
        let zf = self.builder.build_int_compare(
            IntPredicate::EQ,
            value,
            value.get_type().const_zero(),
            "sign_flag_",
        )?;
        Ok(zf)
    }
    // TODO: Review it and check if i need it to
    // FIXME: Steal from Mergen
    pub(super) fn compute_parity_flag(&self, value: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
        // Steal from retdec instead of naci code for now
        let builder = &self.builder;
        let i8t = self.context.i8_type();
        let trunc = self.builder.build_int_truncate(value, i8t, "")?;
        let f =
            Intrinsic::find(CTPOP_INSTRINSIC).ok_or(Error::IntrinsicNotFound(CTPOP_INSTRINSIC))?;

        let f_func = f.get_declaration(&self.module, &[i8t.into()]).unwrap();
        let c = builder.build_call(f_func, &[trunc.into()], "")?;

        let c_retval = c.try_as_basic_value().left().unwrap().into_int_value();

        let a = builder
            // TODO: Check if "false" really fits
            .build_and(c_retval, c_retval.get_type().const_int(1, false), "")?;

        // TODO: Check if false fits here as well
        let pf =
            builder.build_int_compare(IntPredicate::EQ, a, a.get_type().const_int(0, false), "")?;
        Ok(pf)
    }

    // Region: flags
    pub(super) fn retdec_store_registers_plus_sflags<T>(
        &self,
        sflags_val: T,
        regs: &[(ExtendedRegisterEnum, IntValue<'ctx>)],
    ) -> Result<()>
    where
        PossibleLLVMValueEnum<'ctx>: From<T>,
    {
        let sflags_val = PossibleLLVMValueEnum::from(sflags_val);
        for (reg, val) in regs {
            let val = *val;
            self.store_cpu_flag(*reg, val);
        }

        let sflags_val_as_int: IntValue<'_> = sflags_val.try_into()?;
        self.store_cpu_flag(
            ExtendedRegisterEnum::ZF,
            self.generate_zero_flag(sflags_val_as_int)?,
        );
        self.store_cpu_flag(
            ExtendedRegisterEnum::SF,
            self.generate_sign_flag(sflags_val_as_int)?,
        );
        self.store_cpu_flag(
            ExtendedRegisterEnum::PF,
            self.generate_parity_flag(sflags_val_as_int)?,
        );
        Ok(())
    }

    fn generate_zero_flag(&self, val: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
        let zero = val.get_type().const_int(0, false);
        let r = self
            .builder
            .build_int_compare(IntPredicate::EQ, val, zero, "")?;
        Ok(r)
    }

    fn generate_sign_flag(&self, val: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
        let zero = val.get_type().const_int(0, false);
        let r = self
            .builder
            .build_int_compare(IntPredicate::SLT, val, zero, "")?;
        Ok(r)
    }

    fn generate_parity_flag(&self, val: IntValue<'ctx>) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let i8t = self.context.i8_type();
        let trunc = self.builder.build_int_truncate(val, i8t, "")?;
        let f = Intrinsic::find("llvm.ctpop").ok_or(Error::IntrinsicNotFound(CTPOP_INSTRINSIC))?;

        let f_func = f.get_declaration(&self.module, &[i8t.into()]).unwrap();
        let c = builder.build_call(f_func, &[trunc.into()], "")?;

        let c_retval = c.try_as_basic_value().left().unwrap().into_int_value();

        let a = builder
            // TODO: Check if "false" really fits
            .build_and(c_retval, c_retval.get_type().const_int(1, false), "")?;

        // TODO: Check if false fits here as well
        let r =
            builder.build_int_compare(IntPredicate::EQ, a, a.get_type().const_int(0, false), "")?;
        Ok(r)
    }

    pub(super) fn set_rflags_value(&self, value: IntValue<'ctx>) -> Result<()> {
        let builder = &self.builder;
        for flag in 0..12 {
            let shift_amount = flag;
            let shifted_flag_value = builder.build_right_shift(
                value,
                value.get_type().const_int(shift_amount, false),
                false,
                "",
            )?;
            let flag_value =
                builder.build_int_truncate(shifted_flag_value, self.context.bool_type(), "")?;
            let resolved_flag = Self::resolve_flag_from_range(flag)?;
            self.store_cpu_flag(resolved_flag, flag_value);
        }
        Ok(())
    }

    pub(super) fn get_rflags_value(&self) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let biggest_int_type = self.get_max_int_type();
        let mut rflags = biggest_int_type.const_zero();
        for flag in 0..12 {
            let flag_value = self.load_flag(Self::resolve_flag_from_range(flag)?)?;
            let shift_amount = flag;
            let shifted_flag_value = builder.build_left_shift(
                builder.build_int_z_extend(flag_value, biggest_int_type, "")?,
                biggest_int_type.const_int(shift_amount, false),
                "",
            )?;
            rflags = builder.build_or(rflags, shifted_flag_value, "")?;
        }
        Ok(rflags)
    }

    fn resolve_flag_from_range(range_int: u64) -> Result<ExtendedRegisterEnum> {
        match range_int {
            0 => Ok(ExtendedRegisterEnum::CF),
            1 => Ok(ExtendedRegisterEnum::Reserved1),
            2 => Ok(ExtendedRegisterEnum::PF),
            3 => Ok(ExtendedRegisterEnum::Reserved3),
            4 => Ok(ExtendedRegisterEnum::AF),
            5 => Ok(ExtendedRegisterEnum::Reserved5),
            6 => Ok(ExtendedRegisterEnum::ZF),
            7 => Ok(ExtendedRegisterEnum::SF),
            8 => Ok(ExtendedRegisterEnum::TF),
            9 => Ok(ExtendedRegisterEnum::IF),
            10 => Ok(ExtendedRegisterEnum::DF),
            11 => Ok(ExtendedRegisterEnum::OF),
            12 => Ok(ExtendedRegisterEnum::IOPL), // prob the unnecesarry one
            _ => Err(Error::FlagResolveError(range_int)),
        }
    }
}
