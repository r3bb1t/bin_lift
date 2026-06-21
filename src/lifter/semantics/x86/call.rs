use crate::lifter::{Error, LifterX86};
use llvmkit::ir::IntDyn;
use zydis::{ffi::DecodedOperandKind, Instruction, Operands, Register};

use super::Result;

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_call<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let target = operands
            .get(0)
            .ok_or(Error::UnsupportedInstr("call without target"))?;
        match &target.kind {
            DecodedOperandKind::Reg(_)
            | DecodedOperandKind::Imm(_)
            | DecodedOperandKind::Mem(_) => {}
            DecodedOperandKind::Ptr(_) => return Err(Error::UnsupportedInstr("far call operand")),
            DecodedOperandKind::Unused => {
                return Err(Error::UnsupportedInstr("missing call target"))
            }
        }

        let sp = operands
            .get(2)
            .ok_or(Error::UnsupportedInstr("call without stack pointer"))?;
        let stack_dest = operands
            .get(3)
            .ok_or(Error::UnsupportedInstr("call without stack destination"))?;

        let sp_value = self.load_single_int_op(sp, sp.size)?;
        let next_sp = self.builder()?.build_int_sub::<IntDyn, _, _, _>(
            sp_value,
            sp_value
                .ty()
                .const_int_raw(u64::from(self.retdec_get_arch_byte_size()), false)?,
            "call_sp",
        )?;
        let return_address = self.load_register_value(&Register::IP)?;

        self.store_op(sp, next_sp)?;
        self.store_op(stack_dest, return_address)
    }
}
