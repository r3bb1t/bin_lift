use crate::lifter::{Error, LifterX86};
use llvmkit::ir::IntDyn;
use zydis::{ffi::DecodedOperand, Instruction, Mnemonic, Operands};

use super::Result;

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_push<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let (src, stack_dest, sp) = match instr.mnemonic {
            Mnemonic::PUSH => (
                operands
                    .get(0)
                    .ok_or(Error::UnsupportedInstr("push without source"))?,
                operands
                    .get(2)
                    .ok_or(Error::UnsupportedInstr("push without stack destination"))?,
                operands
                    .get(1)
                    .ok_or(Error::UnsupportedInstr("push without stack pointer"))?,
            ),
            Mnemonic::PUSHFQ => (
                operands
                    .get(2)
                    .ok_or(Error::UnsupportedInstr("pushfq without flags source"))?,
                operands
                    .get(1)
                    .ok_or(Error::UnsupportedInstr("pushfq without stack destination"))?,
                operands
                    .get(0)
                    .ok_or(Error::UnsupportedInstr("pushfq without stack pointer"))?,
            ),
            _ => return Err(Error::UnsupportedInstr("unsupported push instruction")),
        };

        self.push_stack_value(src, stack_dest, sp)
    }

    pub(super) fn lift_pushfq<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::PUSHFQ {
            return Err(Error::UnsupportedInstr("unsupported pushfq instruction"));
        }

        let operands = instr.operands();
        let src = operands
            .get(2)
            .ok_or(Error::UnsupportedInstr("pushfq without flags source"))?;
        let stack_dest = operands
            .get(1)
            .ok_or(Error::UnsupportedInstr("pushfq without stack destination"))?;
        let sp = operands
            .get(0)
            .ok_or(Error::UnsupportedInstr("pushfq without stack pointer"))?;

        self.push_stack_value(src, stack_dest, sp)
    }

    fn push_stack_value(
        &mut self,
        src: &DecodedOperand,
        stack_dest: &DecodedOperand,
        sp: &DecodedOperand,
    ) -> Result<()> {
        let value = self.load_single_op(src, stack_dest.size)?;
        let sp_value = self.load_single_int_op(sp, sp.size)?;
        let next_sp = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
            sp_value,
            sp_value
                .ty()
                .const_int_raw(u64::from(stack_dest.size / 8), false)?,
            "push_sp",
        )?;

        self.store_op(sp, next_sp)?;
        self.store_op(stack_dest, value)
    }
}
