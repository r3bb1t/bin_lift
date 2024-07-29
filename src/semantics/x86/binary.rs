// TODO: Implement the following:
// ADC, ADDPD, ADDPS, ADDSD, ADDSS, CMP, DEC, DIVPD, DIVPS, DIVSD, DIVSS, IMUL, INC,
// MULPD, MULSD, MULSS, MULX, NEG, SBB, SUB (implement flags for it), SUBPD, SUBPS, SUBSD, SUBSS
use crate::{
    lifter::{eOpConv, LifterX86},
    miscellaneous::ExtendedRegister,
};
use inkwell::values::IntValue;
use inkwell::IntPredicate;
use inkwell::{builder::BuilderError, values::BasicValue};
use zydis::{Instruction, Operands};

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub(super) fn lift_add<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let operands = instr.operands();

        let operand_values = self.load_2_operands(operands, eOpConv::SEXT_TRUNC_OR_BITCAST)?;
        let lhs = operand_values[0].into_int_value();
        let rhs = operand_values[1].into_int_value();

        let result = self.builder.build_int_add(lhs, rhs, "add_")?;

        self.storeRegistersPlusSflags(
            result.as_basic_value_enum(),
            &[
                (ExtendedRegister::AF, self.generateCarryAddInt4(&lhs, &rhs)?),
                (ExtendedRegister::CF, self.generateCarryAdd(&result, &lhs)?),
                (
                    ExtendedRegister::OF,
                    self.generateOverflowAdd(&result, &lhs, &rhs)?,
                ),
            ],
        )?;

        self.store_op(&operands[0].kind, result)?;
        if instr.mnemonic == zydis::Mnemonic::XADD {
            self.store_op(&operands[1].kind, lhs)?;
        }
        Ok(())
    }

    pub(super) fn lift_sub<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let operands = instr.operands();
        let operand_values = self.load_2_operands(operands, eOpConv::SEXT_TRUNC_OR_BITCAST)?;
        let lhs = operand_values[0].into_int_value();
        let rhs = operand_values[1].into_int_value();

        let result = self.builder.build_int_sub(lhs, rhs, "sub_")?;

        self.storeRegistersPlusSflags(
            result.as_basic_value_enum(),
            &[
                (
                    ExtendedRegister::AF,
                    self.generateBorrowSubInt4(&lhs, &rhs)?,
                ),
                (ExtendedRegister::CF, self.generateBorrowSub(&result, &lhs)?),
                (
                    ExtendedRegister::OF,
                    self.generateOverflowSub(&result, &lhs, &rhs)?,
                ),
            ],
        )?;

        self.store_op(&operands[0].kind, result)?;
        Ok(())
    }

    // region:   Stolen from retdec
    #[allow(non_snake_case)]
    fn generateOverflowAdd(
        &'b self,
        add: &IntValue<'b>,
        op0: &IntValue<'b>,
        op1: &IntValue<'b>,
    ) -> Result<IntValue, BuilderError> {
        let builder = self.builder;
        let xor0 = builder.build_xor(*op0, *add, "")?;
        let xor1 = builder.build_xor(*op1, *add, "")?;

        let ofAnd = builder.build_and(xor0, xor1, "")?;

        builder.build_int_compare(IntPredicate::SLT, ofAnd, ofAnd.get_type().const_zero(), "")
    }

    #[allow(non_snake_case)]
    fn generateCarryAdd(
        &'b self,
        add: &IntValue<'b>,
        op0: &IntValue<'b>,
    ) -> Result<IntValue, BuilderError> {
        self.builder
            .build_int_compare(IntPredicate::ULT, *add, *op0, "")
    }

    #[allow(non_snake_case)]
    fn generateCarryAddInt4(
        &'b self,
        op0: &IntValue<'b>,
        op1: &IntValue<'b>,
    ) -> Result<IntValue, BuilderError> {
        let builder = self.builder;
        let ci15 = op0.get_type().const_int(15, false);
        let and0 = self.builder.build_and(*op0, ci15, "")?;
        let and1 = builder.build_and(*op1, ci15, "")?;
        let add = builder.build_int_add(and0, and1, "")?;

        builder.build_int_compare(IntPredicate::UGT, add, ci15, "")
    }

    #[allow(non_snake_case)]
    fn generateBorrowSubInt4(
        &'b self,
        op0: &IntValue<'b>,
        op1: &IntValue<'b>,
    ) -> Result<IntValue, BuilderError> {
        let builder = self.builder;
        let ci15 = op0.get_type().const_int(15, false);
        let and0 = builder.build_and(*op0, ci15, "")?;
        let and1 = builder.build_and(*op1, ci15, "")?;
        let afSub = builder.build_int_sub(and0, and1, "")?;

        builder.build_int_compare(IntPredicate::UGT, afSub, ci15, "")
    }

    #[allow(non_snake_case)]
    fn generateBorrowSub(
        &'b self,
        op0: &IntValue<'b>,
        op1: &IntValue<'b>,
    ) -> Result<IntValue, BuilderError> {
        self.builder
            .build_int_compare(IntPredicate::ULT, *op0, *op1, "")
    }

    #[allow(non_snake_case)]
    fn generateOverflowSub(
        &'b self,
        sub: &IntValue<'b>,
        op0: &IntValue<'b>,
        op1: &IntValue<'b>,
    ) -> Result<IntValue, BuilderError> {
        let builder = self.builder;
        let xor0 = builder.build_xor(*op0, *op1, "")?;
        let xor1 = builder.build_xor(*op0, *sub, "")?;

        let ofAnd = builder.build_and(xor0, xor1, "")?;

        builder.build_int_compare(IntPredicate::SLT, ofAnd, ofAnd.get_type().const_zero(), "")
    }
    // endregion:   Stolen from retdec
}
