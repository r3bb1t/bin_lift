// pop
use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::{Instruction, Operands};

impl<'ctx> LifterX86<'ctx> {
    //// TODO. Check implemntation!
    //pub(in crate::semantics) fn lift_pop<O: Operands>(
    //    &self,
    //    instr: Instruction<O>,
    //) -> Result<(), BuilderError> {
    //    let operands = instr.operands();
    //
    //    let lhs = self.load_operand(&operands[0])?;
    //
    //    let sp_reg = Register::SP.largest_enclosing(self.mode);
    //    let sp_value = self.load_stack_pointer_value();
    //
    //    let op_size = self
    //        .get_int_type(&sp_reg)
    //        .const_int(operands[0].size as u64, true);
    //
    //    let new_sp_value = self
    //        .builder
    //        .build_int_sub(sp_value, op_size, "pop_new_sp_val_")?;
    //
    //    let ptr_val = self.builder.build_int_to_ptr(
    //        new_sp_value,
    //        new_sp_value.get_type().ptr_type(AddressSpace::default()),
    //        "",
    //    )?;
    //
    //    self.builder.build_store(ptr_val, lhs)?;
    //
    //    self.store_reg(sp_reg, new_sp_value.as_basic_value_enum());
    //
    //    Ok(())
    //}

    pub(super) fn lift_pop<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        let operands = instr.operands();

        let builder = &self.builder;
        let sp = self.retdec_get_sp_pointer_reg_value()?;

        //let ptr_ty = self.context.ptr_type(AddressSpace::default());
        //let addr = builder.build_int_to_ptr(sp, ptr_ty, "")?;
        //let l = builder.build_load(ptr_ty, addr, "")?;
        //dbg!(&operands[0]);
        //dbg!(addr);
        self.retdec_store_op(&operands[0], sp, None)?;

        // Update stack pointer
        let ci = sp
            .get_type()
            .const_int((operands[0].size / 8).into(), false);
        let add = builder.build_int_add(sp, ci, "")?;
        self.retdec_store_register(self.retdec_load_sp_reg(), add, None)?;

        Ok(())
    }
}
