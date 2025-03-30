use super::{LifterX86, Result};

use inkwell::values::IntValue;
use zydis::{ffi::DecodedOperandKind, Instruction, Operands, Register};

impl LifterX86<'_> {
    pub(super) fn lift_call<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let ops = instr.operands();
        let src = &ops[0];
        let rsp = &ops[2];
        let rsp_memory = &ops[3];

        let rsp_value: IntValue<'_> = self.load_single_op(rsp, rsp.size)?.try_into()?;

        // FIXME: Replace 8 with actual calculations
        let val = self.get_max_int_type().const_int(8, false);

        let result = self.builder.build_int_sub(rsp_value, val, "")?;

        if let DecodedOperandKind::Reg(register) = &src.kind {
            let dst_reg: IntValue<'_> = self.load_register_value(register)?.try_into()?;
            self.runtime_address
                .set(dst_reg.get_sign_extended_constant().unwrap() as u64);
            self.store_op(rsp, rsp_value)?;
        }

        self.store_op(rsp, result)?;
        let push_into_rsp: IntValue<'_> = self.load_register_value(&Register::IP)?.try_into()?;
        self.store_op(rsp_memory, push_into_rsp)?;

        Ok(())
    }
}
