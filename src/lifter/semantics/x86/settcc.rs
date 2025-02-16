use super::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegister;

use inkwell::IntPredicate;
use zydis::{Instruction, Operands};

impl LifterX86<'_> {
    pub(super) fn lift_setb<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let cf = self.load_flag(ExtendedRegister::CF)?;

        let result = self
            .builder
            .build_int_z_extend(cf, self.context.i8_type(), "setb")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_setbe<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let cf = self.load_flag(ExtendedRegister::CF)?;
        let zf = self.load_flag(ExtendedRegister::ZF)?;

        let condition = builder.build_or(cf, zf, "setbe_or")?;

        let result = builder.build_int_z_extend(condition, self.context.i8_type(), "setbe")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_setl<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let sf = self.load_flag(ExtendedRegister::SF)?;
        let of = self.load_flag(ExtendedRegister::OF)?;

        let condition = builder.build_int_compare(IntPredicate::NE, sf, of, "setl_condition")?;

        let result = builder.build_int_z_extend(condition, self.context.i8_type(), "setl")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_setle<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let zf = self.load_flag(ExtendedRegister::ZF)?;
        let sf = self.load_flag(ExtendedRegister::SF)?;
        let of = self.load_flag(ExtendedRegister::OF)?;

        let sf_ne_of = builder.build_int_compare(IntPredicate::NE, sf, of, "setl_condition")?;
        let condition = builder.build_or(zf, sf_ne_of, "setle_or")?;

        let result = builder.build_int_z_extend(condition, self.context.i8_type(), "setle")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_setnb<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let cf = self.load_flag(ExtendedRegister::CF)?;

        let result = builder.build_int_compare(
            IntPredicate::EQ,
            cf,
            // in mergen its i1 , its strange, so i use cf type (which is i8)
            cf.get_type().const_zero(),
            "setnb_result",
        )?;

        let byte_result =
            builder.build_int_z_extend(result, self.context.i8_type(), "setnb_byte_result")?;
        self.store_op(&instr.operands()[0], byte_result)?;
        Ok(())
    }

    pub(super) fn lift_setnbe<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let cf = self.load_flag(ExtendedRegister::CF)?;
        let zf = self.load_flag(ExtendedRegister::ZF)?;

        let condition = builder.build_and(
            builder.build_not(cf, "not_cf")?,
            builder.build_not(zf, "not_zf")?,
            "setnbe_and",
        )?;

        let result = builder.build_int_z_extend(condition, self.context.i8_type(), "setl")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_setnl<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let sf = self.load_flag(ExtendedRegister::SF)?;
        let of = self.load_flag(ExtendedRegister::OF)?;

        let condition = builder.build_int_compare(IntPredicate::EQ, sf, of, "setnl_cond")?;

        let result = builder.build_int_z_extend(condition, self.context.i8_type(), "setnl")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_setnle<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let zf = self.load_flag(ExtendedRegister::ZF)?;
        let sf = self.load_flag(ExtendedRegister::SF)?;
        let of = self.load_flag(ExtendedRegister::OF)?;

        let zf_not_set = builder.build_int_compare(
            IntPredicate::EQ,
            zf,
            zf.get_type().const_zero(),
            "setnle_zf_not",
        )?;
        let sf_eq_of = builder.build_int_compare(IntPredicate::EQ, sf, of, "sf_eq_of")?;

        let combined_cond = builder.build_and(zf_not_set, sf_eq_of, "setnle-and")?;

        let byte_result =
            builder.build_int_z_extend(combined_cond, self.context.i8_type(), "setnle_result")?;

        self.store_op(&instr.operands()[0], byte_result)?;
        Ok(())
    }

    pub(super) fn lift_setno<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let of = self.load_flag(ExtendedRegister::OF)?;
        let not_of = builder.build_not(of, "not_of")?;

        let result =
            self.builder
                .build_int_z_extend(not_of, self.context.i8_type(), "setno_not_of_zext")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_setnp<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let pf = self.load_flag(ExtendedRegister::PF)?;

        let result = builder.build_int_z_extend(
            builder.build_not(pf, "not_pf")?,
            self.context.i8_type(),
            "setnp",
        )?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_setns<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let sf = self.load_flag(ExtendedRegister::SF)?;

        let result = builder.build_int_compare(
            IntPredicate::EQ,
            sf,
            // in mergen its i1 , its strange, so i use cf type (which is i8). Yes, here too
            sf.get_type().const_zero(),
            "setnb_result",
        )?;

        let byte_result =
            builder.build_int_z_extend(result, self.context.i8_type(), "setnb_byte_result")?;
        self.store_op(&instr.operands()[0], byte_result)?;
        Ok(())
    }

    pub(super) fn lift_setnz<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let zf = self.load_flag(ExtendedRegister::ZF)?;

        let result = builder.build_int_z_extend(
            builder.build_not(zf, "not_zf")?,
            self.context.i8_type(),
            "setnz",
        )?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_seto<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let cf = self.load_flag(ExtendedRegister::CF)?;

        let result = self
            .builder
            .build_int_z_extend(cf, self.context.i8_type(), "seto_result")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_setp<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let pf = self.load_flag(ExtendedRegister::PF)?;

        let result = self
            .builder
            .build_int_z_extend(pf, self.context.i8_type(), "setp_extend")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_sets<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let sf = self.load_flag(ExtendedRegister::SF)?;

        let result = self
            .builder
            .build_int_z_extend(sf, self.context.i8_type(), "sets")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }

    pub(super) fn lift_setz<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let zf = self.load_flag(ExtendedRegister::ZF)?;

        let result = self
            .builder
            .build_int_z_extend(zf, self.context.i8_type(), "setz_extend")?;

        self.store_op(&instr.operands()[0], result)?;
        Ok(())
    }
}
