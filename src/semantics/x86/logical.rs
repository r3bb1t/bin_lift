use crate::{
    lifter::{eOpConv, LifterX86},
    miscellaneous::ExtendedRegister,
};
use inkwell::{builder::BuilderError, values::BasicValue};
use zydis::{Instruction, Operands};

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub(super) fn lift_xor<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let operands = instr.operands();

        let operand_values = self.load_2_operands(operands, eOpConv::SEXT_TRUNC_OR_BITCAST)?;
        let lhs = operand_values[0].into_int_value();
        let rhs = operand_values[1].into_int_value();

        let result = self.builder.build_xor(lhs, rhs, "add_")?;

        let bool_ty = self.context.bool_type();

        self.storeRegistersPlusSflags(
            result.as_basic_value_enum(),
            &[
                (ExtendedRegister::AF, bool_ty.const_zero()),
                (ExtendedRegister::CF, bool_ty.const_zero()),
                (ExtendedRegister::OF, bool_ty.const_zero()),
            ],
        )?;

        self.store_op(&operands[0].kind, result)?;
        Ok(())
    }
}
