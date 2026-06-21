use crate::lifter::LifterX86;
use llvmkit::ir::{IntDyn, IntValue};
use zydis::{ffi::DecodedOperandKind, Instruction, Operands, Register};

use super::{LiftControl, Result};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_ret<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<LiftControl> {
        let operands = instr.operands();

        if self.runtime_address().is_some() {
            let sp = Register::SP.largest_enclosing(self.mode);
            let sp_value: IntValue<'ctx, IntDyn> = self.get_register(sp)?.try_into()?;
            let sp_ty = sp_value.ty();
            let mut next_sp = self.builder()?.build_int_add::<IntDyn, _, _, _>(
                sp_value,
                sp_ty.const_int_raw(u64::from(self.retdec_get_arch_byte_size()), false)?,
                "ret_sp",
            )?;

            if let Some(first) = operands.first() {
                if let DecodedOperandKind::Imm(immediate) = &first.kind {
                    next_sp = self.builder()?.build_int_add::<IntDyn, _, _, _>(
                        next_sp,
                        sp_ty.const_int_raw(immediate.value, immediate.is_signed)?,
                        "ret_sp_imm",
                    )?;
                }
            }

            self.store_reg(sp, next_sp)?;
            return Ok(LiftControl::Continue);
        }

        let rax = Register::AX.largest_enclosing(self.mode);
        let rax_value: IntValue<'ctx, IntDyn> = self.load_register_value(&rax)?.try_into()?;
        let ret_value = self.create_z_ext_or_trunc(rax_value, self.get_max_int_type()?)?;
        let builder = self.take_builder()?;
        builder.build_ret(ret_value)?;
        Ok(LiftControl::FunctionTerminated)
    }
}
