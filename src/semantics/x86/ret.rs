use crate::lifter::LifterX86;
use crate::miscellaneous::ExtendedRegister;
use inkwell::builder::BuilderError;
use inkwell::values::BasicValue;
use inkwell::AddressSpace;
use zydis::ffi::DecodedOperandKind;
use zydis::{Instruction, Operands};

// ret, retf(?), retfq (?)
impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub(in crate::semantics) fn lift_ret<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let operands = instr.operands();

        let sp_reg = self.load_stack_pointer_reg();
        let int_ty = self.get_int_type(&sp_reg);

        let size_to_add = int_ty.const_int((operands[0].size / 8) as u64, false);
        let curr_stack_pointer_val = self.builder.build_int_add(
            self.load_stack_pointer_value(),
            size_to_add,
            "updated_sp_by_ret",
        )?;

        let builder = self.builder;
        let first_imm = &instr.raw.imm[0];
        if first_imm.value == 0 {
            let addr2 = builder.build_int_to_ptr(
                curr_stack_pointer_val,
                int_ty.ptr_type(AddressSpace::default()),
                "",
            )?;
            let l2 = builder.build_load(int_ty, addr2, "")?;
            self.store_cpu_flag(ExtendedRegister::CS, l2);
            self.store_op(
                &DecodedOperandKind::Reg(sp_reg),
                curr_stack_pointer_val.as_basic_value_enum(),
            )?;
        } else {
            let imm = int_ty.const_int(first_imm.value, first_imm.is_signed);
            let op0 = self.create_z_ext_or_trunc(imm, imm.get_type())?;
            let add = builder.build_int_add(curr_stack_pointer_val, op0, "")?;
            self.store_op(&DecodedOperandKind::Reg(sp_reg), add.as_basic_value_enum())?;
        }

        Ok(())
    }
}
