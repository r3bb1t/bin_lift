use super::{LifterX86, Result};

use inkwell::{builder, types::PointerType, values::IntValue, AddressSpace};
use zydis::{ffi::DecodedOperandKind, Instruction, Operands, Register};

impl LifterX86<'_> {
    pub(super) fn lift_call<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let src = &ops[0];
        let rsp = &ops[2];
        let rsp_memory = &ops[3];

        let rsp_value: IntValue<'_> = self.load_single_op(rsp, rsp.size)?.try_into()?;

        // FIXME: Replace 8 with actual calculations
        let val = self
            .context
            .i64_type()
            .const_int((self.retdec_get_arch_byte_size() / 8).into(), true);

        let result = self
            .builder
            .build_int_sub(rsp_value, val, "pushing_new_rsp_")?;

        if let DecodedOperandKind::Reg(register) = &src.kind {
            let register_value: IntValue<'_> = self.load_register_value(register)?.try_into()?;

            if !register_value.is_constant_int() {
                let id_llvm = builder.build_int_to_ptr(
                    register_value,
                    self.context.ptr_type(AddressSpace::default()),
                    "",
                );
                // TODO: implement later
            }
            let register_c_value = register_value.get_zero_extended_constant().unwrap();
            #[cfg(debug_assertions)]
            println!("CALL: jump address : {register_value}");
        }

        self.store_op(rsp, result)?;
        let push_into_rsp: IntValue<'_> = self.load_register_value(&Register::IP)?.try_into()?;
        self.store_op(rsp_memory, push_into_rsp)?;

        Ok(())
    }
}
