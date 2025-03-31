use inkwell::values::IntValue;
use zydis::{Instruction, Operands, Register};

use super::Result;
use crate::lifter::LifterX86;

impl LifterX86<'_> {
    pub(super) fn lift_jmp<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let dst_op = &instr.operands()[0];

        let destination: IntValue<'_> = self.load_single_op(dst_op, dst_op.size)?.try_into()?;
        let rip_reg = self.get_register_largest_enclosing(&Register::IP);
        let rip_val: IntValue<'_> = self.load_register_value(&rip_reg)?.try_into()?;
        let updated_rip_val = self.builder.build_int_add(rip_val, destination, "")?;
        self.store_reg(rip_reg, updated_rip_val)?;

        #[cfg(debug_assertions)]
        {
            if let Some(constant) = destination.get_sign_extended_constant() {
                eprintln!("JMP destination: {constant}");
                let old_ip = self.runtime_address.get();
                let new_ip = old_ip.wrapping_sub(constant as u64);
                self.runtime_address.set(new_ip);
            } else {
                dbg!(dst_op);
                dbg!(destination);
                eprintln!("ERROR: JMP operand is not a constant")
            }
        }

        //panic!("{:#?}", instr.operands());
        Ok(())
    }
}
