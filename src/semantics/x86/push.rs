// push

use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use inkwell::types::BasicType;
use inkwell::AddressSpace;
use zydis::{Instruction, Operands};

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    /// Push essentially translates to:
    /// ```assembly
    /// sub rsp, operand_size ; operand.size / 8 in zydis
    /// mov rsp, value
    /// ```
    pub(in crate::semantics) fn lift_push<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let builder = self.builder;
        let operands = instr.operands();
        let op0 = self.load_operand(&operands[0])?;
        let sp = self.load_stack_pointer_value();

        let ci = sp
            .get_type()
            .const_int((operands[0].size / 8) as u64, false);
        let sub = builder.build_int_sub(sp, ci, "")?;
        let pt = op0.get_type().ptr_type(AddressSpace::default());
        let addr = builder.build_int_to_ptr(sub, pt, "")?;

        builder.build_store(addr, op0)?;
        self.store_op(&operands[1].kind, sub)?;

        Ok(())
    }
}
