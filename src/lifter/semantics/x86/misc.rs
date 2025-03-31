use super::{LifterX86, Result};

use zydis::{ffi::DecodedOperandKind, Instruction, Operands};

impl LifterX86<'_> {
    pub(super) fn lift_lea<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();

        let dest = &operands[0];
        let src = &operands[1];

        let DecodedOperandKind::Mem(mem) = &src.kind else {
            unreachable!("Buggy lea? operands: {operands:?}")
        };
        let r_value = self.mergen_get_effective_address(mem)?;
        self.store_op(dest, r_value)?;

        Ok(())
    }
}
