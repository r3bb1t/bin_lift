use super::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegister;

use inkwell::{values::IntValue, AddressSpace};
use zydis::{Instruction, Operands, Register};

impl LifterX86<'_> {
    pub(super) fn lift_ret<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let context = &self.context;

        let sp_reg = Register::RSP.largest_enclosing(self.mode);
        let int_ty = context.custom_width_int_type(sp_reg.width(self.mode).into());

        let sp_reg_largest_enclosing = sp_reg.largest_enclosing(self.mode);
        let sp_reg_width = sp_reg_largest_enclosing.width(self.mode) / 8;

        let size_to_add = int_ty.const_int(sp_reg_width as u64, false);
        let sp_value = self.load_reg_internal(&sp_reg)?.try_into()?;
        let curr_stack_pointer_val = self.builder.build_int_add(
            //self.load_stack_pointer_value(),
            sp_value,
            size_to_add,
            "updated_sp_by_ret",
        )?;

        let first_imm = &instr.raw.imm[0];
        if first_imm.value == 0 {
            let addr2 = builder.build_int_to_ptr(
                curr_stack_pointer_val,
                self.context.ptr_type(AddressSpace::default()),
                "",
            )?;
            // TODO: Unwrap here
            let l2: IntValue<'_> = builder.build_load(int_ty, addr2, "")?.try_into().unwrap();
            self.store_cpu_flag(ExtendedRegister::CS, l2);
            // TODO: Fix
            //self.store_op(
            //    &DecodedOperandKind::Reg(sp_reg),
            //    curr_stack_pointer_val.as_basic_value_enum(),
            //)?;
            self.store_reg(sp_reg, curr_stack_pointer_val.into())?;
        } else {
            let imm = int_ty.const_int(first_imm.value, first_imm.is_signed);
            let op0 = self.create_z_ext_or_trunc(imm, imm.get_type())?;
            let add = builder.build_int_add(curr_stack_pointer_val, op0, "")?;
            // TODO: Fix
            //self.store_op(&DecodedOperandKind::Reg(sp_reg), add.as_basic_value_enum())?;
            self.store_reg(sp_reg, add.into())?;
        }

        //todo!()
        Ok(())
    }

    //pub(super) fn lift_ret<O: Operands>(&self, instr: Instruction<O>) -> Result<()> {
    //    let operands = instr.operands();
    //    let builder = &self.builder;
    //
    //    let rsp_value = self.experimental_load_register_internal(&Register::RSP)?;
    //
    //    let rsp_addr = if matches!(operands[0].kind, DecodedOperandKind::Imm(_)) {
    //        &operands[0]
    //    } else {
    //        &operands[2]
    //    };
    //
    //    let retval: BasicValueEnum<'ctx> = self
    //        .experimental_load_single_op(rsp_addr, rsp_addr.size)?
    //        .into();
    //
    //    // TODO: Consider wrapping in a new error
    //    let block = builder.get_insert_block().unwrap();
    //    block.set_name("ret_check_");
    //    let function = block.get_parent();
    //    let last_inst = self.builder.build_return(Some(&retval));
    //
    //    //todo!()
    //    Ok(())
    //}
}
