use crate::{
    lifter::{eOpConv, LifterX86},
    miscellaneous::ExtendedRegister,
};
use inkwell::builder::BuilderError;
use zydis::{Instruction, Operands};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn lift_xor<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        let operands = instr.operands();

        let operand_values = self.load_2_operands(operands)?;
        let lhs = operand_values[0].into_int_value();
        let rhs = operand_values[1].into_int_value();

        let result = self.builder.build_xor(lhs, rhs, "xor_")?;

        let bool_ty = self.context.bool_type();

        self.retdec_store_registers_plus_sflags(
            result,
            &[
                (ExtendedRegister::AF, bool_ty.const_zero()),
                (ExtendedRegister::CF, bool_ty.const_zero()),
                (ExtendedRegister::OF, bool_ty.const_zero()),
            ],
        )?;

        self.store_op(&operands[0].kind, result)?;
        Ok(())
    }

    pub(super) fn lift_or<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        let operands = instr.operands();

        let ops = self.retdec_loadOpBinary(operands, eOpConv::SEXT_TRUNC_OR_BITCAST)?;
        let lhs = ops[0].into_int_value();
        let rhs = ops[1].into_int_value();

        let or = self.builder.build_or(lhs, rhs, "")?;
        let bool_ty = self.context.bool_type();

        self.retdec_store_registers_plus_sflags(
            or,
            &[
                (ExtendedRegister::AF, bool_ty.const_int(0, false)),
                (ExtendedRegister::CF, bool_ty.const_int(0, false)),
                (ExtendedRegister::OF, bool_ty.const_int(0, false)),
            ],
        )?;

        self.retdec_store_op(&operands[0], or, None)?;
        todo!()
    }

    pub(super) fn lift_test<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        todo!()
    }
    pub(super) fn lift_not<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        todo!()
    }

    pub(super) fn lift_and<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        todo!()
    }
}
