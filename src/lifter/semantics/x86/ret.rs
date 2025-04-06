use super::{LifterX86, Result};

use inkwell::values::IntValue;
use zydis::{ffi::DecodedOperandKind, Instruction, Operands, Register};

impl LifterX86<'_> {
    //pub(super) fn lift_ret<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
    //    let builder = &self.builder;
    //    let context = &self.context;
    //
    //    let sp_reg = Register::RSP.largest_enclosing(self.mode);
    //    let int_ty = context.custom_width_int_type(sp_reg.width(self.mode).into());
    //
    //    let sp_reg_largest_enclosing = sp_reg.largest_enclosing(self.mode);
    //    let sp_reg_width = sp_reg_largest_enclosing.width(self.mode) / 8;
    //
    //    let size_to_add = int_ty.const_int(sp_reg_width as u64, false);
    //    let sp_value = self.load_register_value(&sp_reg)?.try_into()?;
    //    let curr_stack_pointer_val = self.builder.build_int_add(
    //        //self.load_stack_pointer_value(),
    //        sp_value,
    //        size_to_add,
    //        "updated_sp_by_ret",
    //    )?;
    //
    //    let first_imm = &instr.raw.imm[0];
    //    if first_imm.value == 0 {
    //        let addr2 = builder.build_int_to_ptr(
    //            curr_stack_pointer_val,
    //            self.context.ptr_type(AddressSpace::default()),
    //            "",
    //        )?;
    //        // TODO: Unwrap here
    //        let l2: IntValue<'_> = builder.build_load(int_ty, addr2, "")?.try_into().unwrap();
    //        self.store_cpu_flag(ExtendedRegisterEnum::CS, l2);
    //        // TODO: Fix
    //        //self.store_op(
    //        //    &DecodedOperandKind::Reg(sp_reg),
    //        //    curr_stack_pointer_val.as_basic_value_enum(),
    //        //)?;
    //        self.store_reg(sp_reg, curr_stack_pointer_val)?;
    //    } else {
    //        let imm = int_ty.const_int(first_imm.value, first_imm.is_signed);
    //        let op0 = self.create_z_ext_or_trunc(imm, imm.get_type())?;
    //        let add = builder.build_int_add(curr_stack_pointer_val, op0, "")?;
    //        // TODO: Fix
    //        //self.store_op(&DecodedOperandKind::Reg(sp_reg), add.as_basic_value_enum())?;
    //        self.store_reg(sp_reg, add.into())?;
    //    }
    //
    //    //todo!()
    //    Ok(())
    //}

    pub(super) fn lift_ret<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();

        let rsp_addr = &ops[2];
        let rsp_value: IntValue<'_> = self.get_register(Register::SP)?.try_into()?;

        let mut rsp_addr = if let DecodedOperandKind::Imm(_) = &ops[0].kind {
            &ops[3]
        } else {
            &ops[2]
        };

        let real_val: IntValue<'_> = self.load_single_op(rsp_addr, rsp_addr.size)?.try_into()?;

        let block = builder.get_insert_block().unwrap();
        block.set_name("ret_check");
        let function = block.get_parent().unwrap();
        let last_inst = builder.build_return(Some(&real_val))?;

        let rop_result = if let Some(const_int) = rsp_value.get_sign_extended_constant() {
            let rsp_val = const_int;
            RopResult::RealReturn
        } else {
            RopResult::RopReturn
        };

        if rop_result == RopResult::RealReturn {
            last_inst.erase_from_basic_block();
            block.set_name("real_ret");
            let rax: IntValue<'_> = self.get_register(Register::AX)?.try_into()?;
            builder.build_return(Some(&builder.build_int_z_extend(
                rax,
                self.context.i64_type(),
                "",
            )?))?;

            let orignal_func_final_opt = builder.get_insert_block().unwrap().get_parent().unwrap();
            return Ok(());
        }

        block.set_name("previousret_block");

        last_inst.erase_from_basic_block();
        block.set_name("fake_ret");

        let val = self
            .context
            .i64_type()
            .const_int((self.retdec_get_arch_byte_size() / 8).into(), true);

        let mut rsp_result = builder.build_int_add(rsp_value, val, "ret_new_rsp_")?;

        if let DecodedOperandKind::Imm(immediate) = &ops[0].kind {
            rsp_addr = &ops[3];
            rsp_result = builder.build_int_add(
                rsp_result,
                rsp_result
                    .get_type()
                    .const_int(immediate.value, immediate.is_signed),
                "",
            )?;
        }

        self.store_reg(Register::SP, rsp_result)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum RopResult {
    RealReturn,
    RopReturn,
}
