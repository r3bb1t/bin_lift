use crate::lifter::{Error, LifterX86};
use llvmkit::ir::IntDyn;
use zydis::{ffi::DecodedOperand, Instruction, Mnemonic, Operands};

use super::Result;

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_pop<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let (dest, stack_src, sp) = match instr.mnemonic {
            Mnemonic::POP => (
                operands
                    .get(0)
                    .ok_or(Error::UnsupportedInstr("pop without destination"))?,
                operands
                    .get(2)
                    .ok_or(Error::UnsupportedInstr("pop without stack source"))?,
                operands
                    .get(1)
                    .ok_or(Error::UnsupportedInstr("pop without stack pointer"))?,
            ),
            Mnemonic::POPFQ => (
                operands
                    .get(2)
                    .ok_or(Error::UnsupportedInstr("popfq without flags destination"))?,
                operands
                    .get(1)
                    .ok_or(Error::UnsupportedInstr("popfq without stack source"))?,
                operands
                    .get(0)
                    .ok_or(Error::UnsupportedInstr("popfq without stack pointer"))?,
            ),
            _ => return Err(Error::UnsupportedInstr("unsupported pop instruction")),
        };

        self.pop_stack_value(dest, stack_src, sp)
    }

    pub(super) fn lift_popfq<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        if instr.mnemonic != Mnemonic::POPFQ {
            return Err(Error::UnsupportedInstr("unsupported popfq instruction"));
        }

        let operands = instr.operands();
        let dest = operands
            .get(2)
            .ok_or(Error::UnsupportedInstr("popfq without flags destination"))?;
        let stack_src = operands
            .get(1)
            .ok_or(Error::UnsupportedInstr("popfq without stack source"))?;
        let sp = operands
            .get(0)
            .ok_or(Error::UnsupportedInstr("popfq without stack pointer"))?;

        self.pop_stack_value(dest, stack_src, sp)
    }

    fn pop_stack_value(
        &mut self,
        dest: &DecodedOperand,
        stack_src: &DecodedOperand,
        sp: &DecodedOperand,
    ) -> Result<()> {
        let value = self.load_single_op(stack_src, dest.size)?;
        let sp_value = self.load_single_int_op(sp, sp.size)?;
        let next_sp = self.builder()?.build_int_add::<IntDyn, _, _, _>(
            sp_value,
            sp_value
                .ty()
                .const_int_raw(u64::from(dest.size / 8), false)?,
            "pop_sp",
        )?;

        self.store_op(sp, next_sp)?;
        self.store_op(dest, value)
    }
}
