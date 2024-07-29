// TODO: Test all this

use crate::{lifter::LifterX86, miscellaneous::ExtendedRegister};
use inkwell::{builder::BuilderError, IntPredicate};
use zydis::{Instruction, Operands};

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub(super) fn lift_stc<O: Operands>(
        &'b self,
        _instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        self.store_cpu_flag_bool(ExtendedRegister::CF, true);
        Ok(())
    }

    pub(super) fn lift_cmc<O: Operands>(
        &'b self,
        _instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let cf = self.load_flag(&ExtendedRegister::CF);
        let xor_op = self
            .builder
            .build_xor(cf, cf.get_type().const_int(1, false), "")?;
        self.store_cpu_flag(ExtendedRegister::CF, xor_op);
        Ok(())
    }

    pub(super) fn lift_clc<O: Operands>(
        &'b self,
        _instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        self.store_cpu_flag_bool(ExtendedRegister::CF, false);
        Ok(())
    }

    pub(super) fn lift_cld<O: Operands>(
        &'b self,
        _instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        self.store_cpu_flag_bool(ExtendedRegister::DF, false);
        Ok(())
    }

    pub(super) fn lift_std<O: Operands>(
        &'b self,
        _instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        self.store_cpu_flag_bool(ExtendedRegister::DF, true);
        Ok(())
    }

    pub(super) fn lift_salc<O: Operands>(
        &'b self,
        _instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let builder = self.builder;
        let cf = self.load_flag(&ExtendedRegister::CF);
        let icmp = builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            cf,
            cf.get_type().const_zero(),
            "",
        )?;
        let i8_ty = self.context.i8_type();
        let v = builder.build_select(icmp, i8_ty.const_zero(), i8_ty.const_int(0xff, false), "")?;
        self.store_cpu_flag(ExtendedRegister::AL, v);
        Ok(())
    }

    pub(super) fn lift_lahf<O: Operands>(
        &'b self,
        _instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let builder = self.builder;
        let i8_ty = self.context.i8_type();

        let cf = builder.build_int_z_extend(self.load_flag(&ExtendedRegister::CF), i8_ty, "")?;
        let pf = builder.build_int_z_extend(self.load_flag(&ExtendedRegister::PF), i8_ty, "")?;
        let af = builder.build_int_z_extend(self.load_flag(&ExtendedRegister::AF), i8_ty, "")?;
        let zf = builder.build_int_z_extend(self.load_flag(&ExtendedRegister::ZF), i8_ty, "")?;
        let sf = builder.build_int_z_extend(self.load_flag(&ExtendedRegister::SF), i8_ty, "")?;

        let zero = i8_ty.const_zero();
        let one = i8_ty.const_int(1, false);

        let mut val = zero;
        val = builder.build_or(val, cf, "")?;
        val = builder.build_or(
            val,
            builder.build_left_shift(one, i8_ty.const_int(1, false), "")?,
            "",
        )?;
        val = builder.build_or(
            val,
            builder.build_left_shift(pf, i8_ty.const_int(2, false), "")?,
            "",
        )?;
        val = builder.build_or(
            val,
            builder.build_left_shift(af, i8_ty.const_int(4, false), "")?,
            "",
        )?;
        val = builder.build_or(
            val,
            builder.build_left_shift(zf, i8_ty.const_int(6, false), "")?,
            "",
        )?;
        val = builder.build_or(
            val,
            builder.build_left_shift(sf, i8_ty.const_int(7, false), "")?,
            "",
        )?;

        self.store_cpu_flag(ExtendedRegister::AH, val);
        Ok(())
    }

    pub(super) fn lift_sahf<O: Operands>(
        &'b self,
        _instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let builder = self.builder;

        let ah = self.load_flag(&ExtendedRegister::AH);
        let ty = ah.get_type();

        let zero = ty.const_zero();

        self.store_cpu_flag(
            ExtendedRegister::CF,
            builder.build_and(ah, ty.const_int(1 << 0, false), "")?,
        );
        self.store_cpu_flag(
            ExtendedRegister::PF,
            builder.build_int_compare(
                IntPredicate::NE,
                builder.build_and(ah, ty.const_int(1 << 2, false), "")?,
                zero,
                "",
            )?,
        );
        self.store_cpu_flag(
            ExtendedRegister::AF,
            builder.build_int_compare(
                IntPredicate::NE,
                builder.build_and(ah, ty.const_int(1 << 4, false), "")?,
                zero,
                "",
            )?,
        );
        self.store_cpu_flag(
            ExtendedRegister::ZF,
            builder.build_int_compare(
                IntPredicate::NE,
                builder.build_and(ah, ty.const_int(1 << 6, false), "")?,
                zero,
                "",
            )?,
        );
        self.store_cpu_flag(
            ExtendedRegister::SF,
            builder.build_int_compare(
                IntPredicate::NE,
                builder.build_and(ah, ty.const_int(1 << 7, false), "")?,
                zero,
                "",
            )?,
        );
        Ok(())
    }
}
