use super::Result;
use crate::lifter::{Error, LifterX86};

use zydis::{ffi::DecodedOperandKind, Instruction, Operands};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_lea<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];

        let DecodedOperandKind::Mem(mem) = &src.kind else {
            return Err(Error::UnsupportedInstr("lea without memory source"));
        };

        let value = self.mergen_get_effective_address(mem)?;
        self.store_op(dest, value)
    }
}
