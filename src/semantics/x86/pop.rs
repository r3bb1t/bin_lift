// pop
use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::ffi::{DecodedOperandKind, DisplacementInfo, MemoryInfo};
use zydis::{Instruction, MemoryOperandType, Operands, Register};

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    // TODO. Check implemntation!
    pub(in crate::semantics) fn lift_pop<O: Operands>(
        &'b self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        let operands = instr.operands();

        let lhs = self.load_operand(&operands[0])?;

        let sp_reg = Register::SP.largest_enclosing(*self.mode);
        let sp_value = self.load_stack_pointer_value();

        let op_size = self
            .get_int_type(&sp_reg)
            .const_int(operands[0].size as u64, true);

        let new_sp_value = self.builder.build_int_sub(sp_value, op_size, "")?;

        let to_store = MemoryInfo {
            ty: MemoryOperandType::MEM,
            segment: Register::NONE,
            base: sp_reg,
            index: Register::NONE,
            scale: 0,
            disp: DisplacementInfo {
                has_displacement: false,
                displacement: 0,
            },
        };
        self.store_op(&DecodedOperandKind::Mem(to_store), lhs)?;
        self.store_op(&DecodedOperandKind::Reg(sp_reg), new_sp_value)?;

        // self.store_op(operands[0].to_owned().kind, lhs);
        //
        // let int_type = self.get_int_type(sp_reg);
        // let curr_stack_pointer = self.builder.build_int_sub(
        //     self.load_stack_pointer().into_int_value(),
        //     int_type.const_int((operands[0].size / 8) as u64, true),
        //     "",
        // )?;
        //
        // self.store_op(DecodedOperandKind::Reg(sp_reg), curr_stack_pointer);

        Ok(())
    }
}
