// push

use crate::lifter::LifterX86;
use inkwell::{builder::BuilderError, AddressSpace};
use zydis::{Instruction, Operands};

impl<'ctx> LifterX86<'ctx> {
    /// Push essentially translates to:
    /// ```assembly
    /// sub rsp, operand_size ; operand.size / 8 in zydis
    /// mov rsp, value
    /// ```
    pub(super) fn lift_push<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        let builder = &self.builder;
        let operands = instr.operands();
        //let op0 = self.load_operand(&operands[0])?;
        //let op0 = self.retdec_load_op_2(&operands[0], &None, false)?;
        //let sp = self.load_stack_pointer_value();
        //
        //let ci = sp
        //    .get_type()
        //    .const_int((operands[0].size / 8) as u64, false);
        //let sub = builder.build_int_sub(sp, ci, "push_")?;
        //let addr = builder.build_int_to_ptr(
        //    sub,
        //    self.context.ptr_type(AddressSpace::default()),
        //    "push_",
        //)?;
        //
        //builder.build_store(addr, op0)?;
        //
        //let op0 = self.retdec_loadOpUnary(operands, None, None, None)?;
        let op0 = self.retdec_load_operand(&operands[0], &None, false)?;
        let sp = self.retdec_get_sp_pointer_reg_value()?;

        let ci = sp
            .get_type()
            .const_int((operands[0].size / 8) as u64, false);
        let sub = builder.build_int_sub(sp, ci, "push_")?;
        let pt = self.context.ptr_type(AddressSpace::default());
        let addr = builder.build_int_to_ptr(sub, pt, "push_")?;

        builder.build_store(addr, op0)?;
        self.retdec_store_register(self.retdec_load_sp_reg(), sub, None)?;

        Ok(())
    }
}
