use super::{LifterX86, PossibleLLVMValueEnum, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use inkwell::values::IntValue;
use zydis::{
    ffi::{DecodedOperand, DecodedOperandKind},
    Register, RegisterClass,
};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn store_op<T>(&self, op: &DecodedOperand, value: T) -> Result<()>
    where
        PossibleLLVMValueEnum<'ctx>: From<T>,
    {
        let val = PossibleLLVMValueEnum::from(value);

        match &op.kind {
            // NOTE: When adding float support, must revisit this first
            DecodedOperandKind::Reg(reg) => self.store_reg(*reg, val.try_into()?)?,
            DecodedOperandKind::Mem(memory_info) => self.mergen_store_mem(memory_info, val)?,
            _ => unreachable!("Tried to set value to operand with kind {:?}", op.kind),
        };

        Ok(())
    }

    //pub(super) fn store_mem(
    //    &self,
    //    mem: &MemoryInfo,
    //    val: PossibleLLVMValueEnum<'ctx>,
    //) -> Result<()> {
    //    let builder = &self.builder;
    //    let mem_addr = self.calc_mem_operand(mem)?;
    //
    //
    //    let i8_ty = self.context.i8_type();
    //    let pointer =
    //        unsafe { builder.build_gep(i8_ty, self.stackmemory, &[mem_addr], "GEPSTORE")? };
    //
    //    let val: BasicValueEnum = val.into();
    //    builder.build_store(pointer, val)?;
    //    Ok(())
    //}

    pub(super) fn store_reg(&self, reg: Register, mut val: IntValue<'ctx>) -> Result<()> {
        const GPR_8_BIT: [Register; 20] = [
            Register::AL,
            Register::CL,
            Register::DL,
            Register::BL,
            Register::AH,
            Register::CH,
            Register::DH,
            Register::BH,
            Register::SPL,
            Register::BPL,
            Register::SIL,
            Register::DIL,
            Register::R8B,
            Register::R9B,
            Register::R10B,
            Register::R11B,
            Register::R12B,
            Register::R13B,
            Register::R14B,
            Register::R15B,
        ];

        const GPR_16_BIT: [Register; 16] = [
            Register::AX,
            Register::CX,
            Register::DX,
            Register::BX,
            Register::SP,
            Register::BP,
            Register::SI,
            Register::DI,
            Register::R8W,
            Register::R9W,
            Register::R10W,
            Register::R11W,
            Register::R12W,
            Register::R13W,
            Register::R14W,
            Register::R15W,
        ];

        if GPR_8_BIT.contains(&reg) {
            val = self.set_val_to_sub_reg_8b(reg, val)?;
        }

        if GPR_16_BIT.contains(&reg) {
            val = self.set_val_to_sub_reg_16b(reg, val)?;
        }

        if reg.class() == RegisterClass::FLAGS {
            self.set_rflags_value(val)?;
            return Ok(());
        }

        let new_key = if [RegisterClass::FLAGS, RegisterClass::IP].contains(&reg.class()) {
            reg
        } else {
            self.get_register_largest_enclosing(&reg)
        };

        self.store_register_internal(new_key, val);

        Ok(())
    }

    fn set_val_to_sub_reg_8b(
        &self,
        reg: Register,
        value: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let ctx = self.context;

        let full_reg_key = reg.largest_enclosing(self.mode);
        let mut full_reg_value = self.load_register_value(&full_reg_key)?.try_into()?;
        //full_reg_value = self.create_z_ext_or_trunc(full_reg_value, ctx.i64_type())?;
        full_reg_value = self.create_z_ext_or_trunc(full_reg_value, self.get_max_int_type())?;

        let mut extended_value = builder.build_int_z_extend(value, ctx.i64_type(), "")?;

        let is_high_byte_reg =
            [Register::AH, Register::CH, Register::DH, Register::BH].contains(&reg);

        let mask: u64 = if is_high_byte_reg {
            0xFFFFFFFFFFFF00FF
        } else {
            0xFFFFFFFFFFFFFF00
        };

        let mask_value = ctx.i64_type().const_int(mask, false);
        let masked_full_reg = builder.build_and(full_reg_value, mask_value, "maskedreg_")?;

        if is_high_byte_reg {
            extended_value = builder.build_left_shift(
                extended_value,
                extended_value.get_type().const_int(8, false),
                "shifted_value_",
            )?;
        }

        let updated_reg = builder.build_or(masked_full_reg, extended_value, "newreg_")?;

        self.store_register_internal(full_reg_key, updated_reg);

        Ok(updated_reg)
    }

    fn set_val_to_sub_reg_16b(
        &self,
        reg: Register,
        value: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;

        let full_reg_key = reg.largest_enclosing(self.mode);
        let full_reg_value: IntValue<'_> = self.load_register_value(&full_reg_key)?.try_into()?;
        let last_4_cleared = full_reg_value
            .get_type()
            .const_int(0xFFFFFFFFFFFF0000, false);
        let masked_full_reg = builder.build_and(full_reg_value, last_4_cleared, "maskedreg_")?;
        let value = builder.build_int_z_extend(value, full_reg_value.get_type(), "")?;

        let updated_reg = builder.build_or(masked_full_reg, value, "newreg_")?;
        Ok(updated_reg)
    }

    fn store_register_internal<T>(&self, r: Register, val: T)
    where
        PossibleLLVMValueEnum<'ctx>: From<T>,
    {
        let value = PossibleLLVMValueEnum::from(val);
        let pr = self.get_register_largest_enclosing(&r);
        let regs_hashmap = self.regs_hashmap_mut();
        regs_hashmap.insert(pr.into(), value);
    }

    pub(super) fn store_cpu_flag(&self, flag: ExtendedRegisterEnum, val: IntValue<'ctx>) {
        let reg_type = self.context.custom_width_int_type(1);
        let val = self.create_z_ext_or_trunc(val, reg_type).unwrap();
        let regs = self.regs_hashmap_mut();
        regs.insert(flag, val.into());
    }

    pub(super) fn store_cpu_flag_bool(&self, cpu_flag: ExtendedRegisterEnum, val: bool) {
        let bool_ty = &self.context.bool_type();
        let value = match val {
            true => bool_ty.const_int(1, false),
            false => bool_ty.const_zero(),
        };
        let regs_hashmap = self.regs_hashmap_mut();
        regs_hashmap.insert(cpu_flag, value.into());
    }
}
