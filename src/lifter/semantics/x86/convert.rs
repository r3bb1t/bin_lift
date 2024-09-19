use crate::lifter::LifterX86;
use inkwell::{builder::BuilderError, values::BasicValue};
use zydis::{Instruction, Operands, Register};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn lift_cwd<O: Operands>(&self, _instr: Instruction<O>) -> Result<(), BuilderError> {
        //let op0 = self.load_reg(&Register::RAX)?.into_int_value();
        let op0 = self
            .retdec_load_reg(&Register::RAX, None, None)?
            .unwrap()
            .into_int_value();
        // TODO: Check if second arg is correct
        let e = self.builder.build_right_shift(
            op0,
            op0.get_type()
                .const_int((Register::RAX.width(self.mode) - 1).into(), false),
            false,
            "",
        )?;

        //self.store_reg(Register::DX, e.as_basic_value_enum());
        self.retdec_store_register(Register::DX, e.as_basic_value_enum(), None)?;

        Ok(())
    }

    pub(super) fn lift_cwde<O: Operands>(
        &self,
        _instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }
}
