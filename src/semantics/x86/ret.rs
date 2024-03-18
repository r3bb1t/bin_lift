use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use inkwell::values::BasicValueEnum;
use zydis::ffi::DecodedOperandKind;
use zydis::{Instruction, Operands, Register};

// ret
impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub(in crate::semantics) fn lift_ret<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let operands = instr.operands();

        let lhs = self.load_operand(&operands[0])?;

        let sp_reg = Register::SP.largest_enclosing(*self.mode);
        self.store_op(operands[0].to_owned().kind, lhs);

        let curr_stack_pointer = self.builder.build_int_add(
            self.load_stack_pointer().into_int_value(),
            self.context
                .i8_type()
                .const_int((operands[0].size / 8) as u64, true),
            "",
        )?;

        self.store_op(
            DecodedOperandKind::Reg(sp_reg),
            BasicValueEnum::from(curr_stack_pointer),
        );

        Ok(())
    }
}
