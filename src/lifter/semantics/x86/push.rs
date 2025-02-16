use super::{LifterX86, Result};

use inkwell::values::IntValue;
use zydis::{Instruction, Operands};

impl LifterX86<'_> {
    pub(super) fn lift_push<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();

        let src = &operands[0];
        let dest = &operands[2];
        let rsp = &operands[1];

        let r_value = self.load_single_op(src, dest.size)?;
        let rsp_value: IntValue<'_> = self.load_single_op(rsp, rsp.size)?.try_into()?;

        let val = self
            .context
            // // TODO: check this. Different from Mergen
            // .custom_width_int_type(dest.size.into())
            .i64_type()
            .const_int((dest.size / 8).into(), true);
        let result = self
            .builder
            .build_int_sub(rsp_value, val, "pushing_new_rsp_")?;

        self.store_op(rsp, result)?;
        self.store_op(dest, r_value)?;

        Ok(())
    }

    pub(super) fn lift_pushfq<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();

        let src = &operands[2];
        let dest = &operands[1];
        let rsp = &operands[0];

        let r_value = self.load_single_op(src, dest.size)?;
        let rsp_value: IntValue<'_> = self.load_single_op(rsp, rsp.size)?.try_into()?;

        let val = self
            .context
            // // TODO: check this. Different from Mergen
            // .custom_width_int_type(dest.size.into())
            .i64_type()
            .const_int((dest.size / 8).into(), true);
        let result = self
            .builder
            .build_int_sub(rsp_value, val, "pushing_new_rsp_")?;

        self.store_op(rsp, result)?;
        self.store_op(dest, r_value)?;

        Ok(())
    }
}
