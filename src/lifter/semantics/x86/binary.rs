// TODO: Implement the following:
// ADC, ADDPD, ADDPS, ADDSD, ADDSS, CMP, DEC, DIVPD, DIVPS, DIVSD, DIVSS, IMUL, INC,
// MULPD, MULSD, MULSS, MULX, NEG, SBB, SUB (implement flags for it), SUBPD, SUBPS, SUBSD, SUBSS
use crate::{lifter::LifterX86, miscellaneous::ExtendedRegister};
use inkwell::builder::BuilderError;
use inkwell::values::IntValue;
use inkwell::IntPredicate;
use zydis::{Instruction, Operands};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn lift_add<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        let operands = instr.operands();

        let operand_values = self.load_2_operands(operands)?;

        let lhs = operand_values[0].into_int_value();
        let rhs = operand_values[1].into_int_value();

        let result = self.builder.build_int_add(lhs, rhs, "add_")?;

        self.retdec_store_registers_plus_sflags(
            result,
            &[
                (
                    ExtendedRegister::AF,
                    self.generate_carry_add_int4(&lhs, &rhs)?,
                ),
                (
                    ExtendedRegister::CF,
                    self.generate_carry_add(&result, &lhs)?,
                ),
                (
                    ExtendedRegister::OF,
                    self.generate_overflow_add(&result, &lhs, &rhs)?,
                ),
            ],
        )?;

        self.retdec_store_op(&operands[0], result, None)?;
        if instr.mnemonic == zydis::Mnemonic::XADD {
            //self.store_op(&operands[1].kind, lhs)?;
            self.retdec_store_op(&operands[1], lhs, None)?;
        }
        Ok(())
    }

    pub(super) fn lift_sub<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        let operands = instr.operands();
        //let operand_values = self.load_2_operands(operands, eOpConv::SEXT_TRUNC_OR_BITCAST)?;
        let operand_values = self.load_2_operands(operands)?;
        let lhs = operand_values[0].into_int_value();
        let rhs = operand_values[1].into_int_value();

        let result = self.builder.build_int_sub(lhs, rhs, "sub_")?;

        self.retdec_store_registers_plus_sflags(
            result,
            &[
                (
                    ExtendedRegister::AF,
                    self.generate_borrow_sub_int4(&lhs, &rhs)?,
                ),
                (
                    ExtendedRegister::CF,
                    self.generate_borrow_sub(&result, &lhs)?,
                ),
                (
                    ExtendedRegister::OF,
                    self.generate_overflow_sub(&result, &lhs, &rhs)?,
                ),
            ],
        )?;
        self.retdec_store_op(&operands[0], result, None)?;
        Ok(())
    }

    // region:   Stolen from retdec
    fn generate_overflow_add(
        &self,
        add: &IntValue<'ctx>,
        op0: &IntValue<'ctx>,
        op1: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        let builder = &self.builder;
        let xor0 = builder.build_xor(*op0, *add, "")?;
        let xor1 = builder.build_xor(*op1, *add, "")?;

        let of_and = builder.build_and(xor0, xor1, "")?;

        builder.build_int_compare(
            IntPredicate::SLT,
            of_and,
            of_and.get_type().const_zero(),
            "",
        )
    }

    fn generate_carry_add(
        &self,
        add: &IntValue<'ctx>,
        op0: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        self.builder
            .build_int_compare(IntPredicate::ULT, *add, *op0, "")
    }

    fn generate_carry_add_int4(
        &self,
        op0: &IntValue<'ctx>,
        op1: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        let builder = &self.builder;
        let ci15 = op0.get_type().const_int(15, false);
        let and0 = self.builder.build_and(*op0, ci15, "")?;
        let and1 = builder.build_and(*op1, ci15, "")?;
        let add = builder.build_int_add(and0, and1, "")?;

        builder.build_int_compare(IntPredicate::UGT, add, ci15, "")
    }

    fn generate_borrow_sub_int4(
        &self,
        op0: &IntValue<'ctx>,
        op1: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        let builder = &self.builder;
        let ci15 = op0.get_type().const_int(15, false);
        let and0 = builder.build_and(*op0, ci15, "")?;
        let and1 = builder.build_and(*op1, ci15, "")?;
        let af_sub = builder.build_int_sub(and0, and1, "")?;

        builder.build_int_compare(IntPredicate::UGT, af_sub, ci15, "")
    }

    fn generate_borrow_sub(
        &self,
        op0: &IntValue<'ctx>,
        op1: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        self.builder
            .build_int_compare(IntPredicate::ULT, *op0, *op1, "")
    }

    fn generate_overflow_sub(
        &self,
        sub: &IntValue<'ctx>,
        op0: &IntValue<'ctx>,
        op1: &IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        let builder = &self.builder;
        let xor0 = builder.build_xor(*op0, *op1, "")?;
        let xor1 = builder.build_xor(*op0, *sub, "")?;

        let of_and = builder.build_and(xor0, xor1, "")?;

        builder.build_int_compare(
            IntPredicate::SLT,
            of_and,
            of_and.get_type().const_zero(),
            "",
        )
    }
    // endregion:   Stolen from retdec
}
