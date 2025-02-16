use super::{LifterX86, Result};

use zydis::{ffi::DecodedOperandKind, Instruction, Operands};

impl LifterX86<'_> {
    pub(super) fn lift_lea<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();

        let dest = &operands[0];
        //let src = &operands[1];

        //let r_value = self.experimental_get_effective_address(src, dest.size)?;

        if let DecodedOperandKind::Mem(mem) = &operands[1].kind {
            let r_value = self.retdec_calc_mem_operand(mem)?;
            self.store_op(dest, r_value)?;
        } else {
            unreachable!("Buggy lea? operands: {operands:?}")
        }

        Ok(())
    }
}
