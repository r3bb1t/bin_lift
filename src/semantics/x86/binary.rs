// FIXME: Don't forget to implement flags logic

use crate::lifter::{eOpConv, LifterX86};
use inkwell::builder::BuilderError;
use zydis::{Instruction, Operands};

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub(in crate::semantics) fn lift_add<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let operands = instr.operands();

        let (lhs, rhs) = self.load_2_operands(operands, eOpConv::SEXT_TRUNC_OR_BITCAST)?;

        let result =
            self.builder
                .build_int_add(lhs.into_int_value(), rhs.into_int_value(), "add_")?;
        self.store_op(operands[0].clone().kind, result);
        Ok(())
    }
    pub(in crate::semantics) fn lift_sub<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let operands = instr.operands();
        let (lhs, rhs) = self.load_2_operands(operands, eOpConv::SEXT_TRUNC_OR_BITCAST)?;

        let result =
            self.builder
                .build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "sub_")?;
        self.store_op(operands[0].clone().kind, result);
        Ok(())
    }
}
