// push

use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use zydis::ffi::{DecodedOperandKind, DisplacementInfo, MemoryInfo};
use zydis::{Instruction, MemoryOperandType, Operands, Register};

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
        let operands = instr.operands();

        let rhs = self.load_operand(&operands[0])?;

        let sp_reg = Register::SP.largest_enclosing(*self.mode);

        let int_type = self.get_int_type(&sp_reg);
        let updated_sp = self.builder.build_int_sub(
            self.load_stack_pointer().into_int_value(),
            int_type.const_int((operands[0].size / 8) as u64, true),
            "",
        )?;

        let to_store = MemoryInfo {
            ty: MemoryOperandType::MEM,
            segment: Register::SS,
            base: sp_reg,
            index: Register::NONE,
            scale: 0,
            disp: DisplacementInfo {
                has_displacement: false,
                displacement: 0,
            },
        };

        updated_sp.set_name("sp_");

        self.store_op(DecodedOperandKind::Mem(to_store), rhs);
        self.store_op(DecodedOperandKind::Reg(sp_reg), updated_sp);

        Ok(())
    }
}
