use crate::lifter::{Error, LifterX86};
use llvmkit::ir::{IntDyn, IntValue};
use zydis::{Instruction, Operands, Register};

use super::Result;

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_jmp<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let dst_op = instr
            .operands()
            .get(0)
            .ok_or(Error::UnsupportedInstr("jmp without destination"))?;
        let destination = self.load_single_int_op(dst_op, dst_op.size)?;
        let ip = Register::IP.largest_enclosing(self.mode);
        let ip_value: IntValue<'ctx, IntDyn> = self.load_register_value(&ip)?.try_into()?;
        let updated =
            self.builder()?
                .build_int_add::<IntDyn, _, _, _>(ip_value, destination, "jmp_ip")?;
        self.store_reg(ip, updated)
    }
}
