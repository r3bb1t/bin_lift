use super::{LiftValue, LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{IntDyn, IntValue};
use zydis::{
    ffi::{DecodedOperand, DecodedOperandKind},
    Register, RegisterClass,
};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn store_op<T>(&mut self, op: &DecodedOperand, value: T) -> Result<()>
    where
        LiftValue<'ctx>: From<T>,
    {
        let val = LiftValue::from(value);
        match &op.kind {
            DecodedOperandKind::Reg(reg) => self.store_reg(*reg, val.try_into()?),
            DecodedOperandKind::Mem(mem) => self.mergen_store_mem(mem, val),
            _ => Err(super::Error::UnsupportedInstr(
                "unsupported destination operand kind",
            )),
        }
    }

    pub(super) fn store_reg(&mut self, reg: Register, val: IntValue<'ctx, IntDyn>) -> Result<()> {
        if reg.class() == RegisterClass::FLAGS {
            self.set_rflags_value(val)?;
            return Ok(());
        }

        let key = if reg.class() == RegisterClass::IP {
            reg
        } else {
            self.get_register_largest_enclosing(&reg)
        };
        let reg_width = reg.width(self.mode);
        let key_width = key.width(self.mode);
        let value = if reg_width < key_width && reg_width == 8 {
            self.set_val_to_sub_reg_8b(reg, val)?
        } else if reg_width < key_width && reg_width == 16 {
            self.set_val_to_sub_reg_16b(reg, val)?
        } else {
            val
        };

        self.store_register_internal(key, value);
        Ok(())
    }

    fn set_val_to_sub_reg_8b(
        &self,
        reg: Register,
        value: IntValue<'ctx, IntDyn>,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let full_reg_key = self.get_register_largest_enclosing(&reg);
        let full_ty = self
            .module
            .custom_width_int_type(u32::from(full_reg_key.width(self.mode)))?;
        let full_value: IntValue<'ctx, IntDyn> =
            self.load_register_value(&full_reg_key)?.try_into()?;
        let full_value = self.create_z_ext_or_trunc(full_value, full_ty)?;
        let mut extended_value = self.create_z_ext_or_trunc(value, full_ty)?;
        let high_byte = [Register::AH, Register::CH, Register::DH, Register::BH].contains(&reg);
        let mask = match (full_ty.bit_width(), high_byte) {
            (64, true) => 0xFFFF_FFFF_FFFF_00FF,
            (64, false) => 0xFFFF_FFFF_FFFF_FF00,
            (32, true) => 0xFFFF_00FF,
            (32, false) => 0xFFFF_FF00,
            (16, true) => 0x00FF,
            (16, false) => 0xFF00,
            _ => return Ok(value),
        };
        let masked_full_reg = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            full_value,
            full_ty.const_int_raw(mask, false)?,
            "maskedreg_",
        )?;
        if high_byte {
            extended_value = self.builder()?.build_int_shl::<IntDyn, _, _, _>(
                extended_value,
                full_ty.const_int_raw(8, false)?,
                "shifted_value_",
            )?;
        }
        Ok(self.builder()?.build_int_or::<IntDyn, _, _, _>(
            masked_full_reg,
            extended_value,
            "newreg_",
        )?)
    }

    fn set_val_to_sub_reg_16b(
        &self,
        reg: Register,
        value: IntValue<'ctx, IntDyn>,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let full_reg_key = self.get_register_largest_enclosing(&reg);
        let full_ty = self
            .module
            .custom_width_int_type(u32::from(full_reg_key.width(self.mode)))?;
        let full_value: IntValue<'ctx, IntDyn> =
            self.load_register_value(&full_reg_key)?.try_into()?;
        let full_value = self.create_z_ext_or_trunc(full_value, full_ty)?;
        let mask = match full_ty.bit_width() {
            64 => 0xFFFF_FFFF_FFFF_0000,
            32 => 0xFFFF_0000,
            _ => return Ok(value),
        };
        let masked_full_reg = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            full_value,
            full_ty.const_int_raw(mask, false)?,
            "maskedreg_",
        )?;
        let extended_value = self.create_z_ext_or_trunc(value, full_ty)?;
        Ok(self.builder()?.build_int_or::<IntDyn, _, _, _>(
            masked_full_reg,
            extended_value,
            "newreg_",
        )?)
    }

    fn store_register_internal<T>(&mut self, register: Register, value: T)
    where
        LiftValue<'ctx>: From<T>,
    {
        let key = self.get_register_largest_enclosing(&register);
        self.regs_hashmap_mut()
            .insert(key.into(), LiftValue::from(value));
    }

    pub(super) fn store_cpu_flag(
        &mut self,
        flag: ExtendedRegisterEnum,
        value: IntValue<'ctx, IntDyn>,
    ) -> Result<()> {
        let value = self.create_z_ext_or_trunc(value, self.module.bool_type().as_dyn())?;
        self.regs_hashmap_mut().insert(flag, value.into());
        Ok(())
    }

    pub(super) fn store_cpu_flag_bool(
        &mut self,
        flag: ExtendedRegisterEnum,
        value: bool,
    ) -> Result<()> {
        let value = if value {
            self.module.bool_type().const_int(true)
        } else {
            self.module.bool_type().const_zero()
        };
        self.regs_hashmap_mut().insert(flag, value.into());
        Ok(())
    }

    fn set_rflags_value(&mut self, value: IntValue<'ctx, IntDyn>) -> Result<()> {
        let flag_ty = value.ty();
        for flag in 0_u64..12 {
            let shifted_flag_value = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
                value,
                flag_ty.const_int_raw(flag, false)?,
                "",
            )?;
            let flag_value =
                self.create_z_ext_or_trunc(shifted_flag_value, self.module.bool_type().as_dyn())?;
            self.store_cpu_flag(Self::resolve_flag_from_range(flag)?, flag_value)?;
        }
        Ok(())
    }
}
