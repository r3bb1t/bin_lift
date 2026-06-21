use super::{Error, LiftValue, LifterBuilder, LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{IntDyn, IntType, IntValue};
use std::collections::HashMap;
use zydis::{MachineMode, Register};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn builder(&self) -> Result<&LifterBuilder<'m, 'ctx>> {
        self.builder
            .as_ref()
            .ok_or(Error::FunctionAlreadyTerminated)
    }

    pub(crate) fn take_builder(&mut self) -> Result<LifterBuilder<'m, 'ctx>> {
        self.builder.take().ok_or(Error::FunctionAlreadyTerminated)
    }

    pub(crate) fn runtime_address(&self) -> Option<u64> {
        self.runtime_address.as_ref().map(std::cell::Cell::get)
    }

    pub(crate) fn increase_ip(&self, instr_length: u8) {
        let Some(current_ip_cell) = &self.runtime_address else {
            return;
        };
        current_ip_cell.set(current_ip_cell.get() + u64::from(instr_length));
    }

    pub(super) fn get_max_int_type(&self) -> Result<IntType<'ctx, IntDyn>> {
        let example_reg = Register::AX.largest_enclosing(self.mode);
        Ok(self
            .module
            .custom_width_int_type(example_reg.width(self.mode).into())?)
    }

    pub(crate) fn create_z_ext_or_trunc(
        &self,
        value: IntValue<'ctx, IntDyn>,
        dest: IntType<'ctx, IntDyn>,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let value_bits = value.ty().bit_width();
        let dest_bits = dest.bit_width();
        if value_bits < dest_bits {
            Ok(self.builder()?.build_zext_dyn(value, dest, "")?)
        } else if value_bits > dest_bits {
            Ok(self.builder()?.build_trunc_dyn(value, dest, "")?)
        } else {
            Ok(value)
        }
    }

    pub fn retdec_get_default_type(&self) -> Result<IntType<'ctx, IntDyn>> {
        let arch_byte_size = self.retdec_get_arch_byte_size();
        Ok(self
            .module
            .custom_width_int_type(u32::from(arch_byte_size))?)
    }

    pub fn retdec_get_arch_byte_size(&self) -> u8 {
        match self.mode {
            MachineMode::LONG_64 => 8,
            MachineMode::LONG_COMPAT_32 | MachineMode::LEGACY_32 => 4,
            MachineMode::LONG_COMPAT_16 | MachineMode::LEGACY_16 | MachineMode::REAL_16 => 2,
        }
    }

    pub(super) fn get_register_largest_enclosing(&self, register: &Register) -> Register {
        if [Register::RBP, Register::EBP, Register::BP].contains(register) {
            match self.mode {
                MachineMode::LONG_64 => Register::RBP,
                MachineMode::LONG_COMPAT_32 | MachineMode::LEGACY_32 => Register::EBP,
                MachineMode::LONG_COMPAT_16 | MachineMode::LEGACY_16 | MachineMode::REAL_16 => {
                    Register::BP
                }
            }
        } else if [Register::RSP, Register::ESP, Register::SP].contains(register) {
            match self.mode {
                MachineMode::LONG_64 => Register::RSP,
                MachineMode::LONG_COMPAT_32 | MachineMode::LEGACY_32 => Register::ESP,
                MachineMode::LONG_COMPAT_16 | MachineMode::LEGACY_16 | MachineMode::REAL_16 => {
                    Register::SP
                }
            }
        } else {
            register.largest_enclosing(self.mode)
        }
    }

    pub(super) fn regs_hashmap_mut(
        &mut self,
    ) -> &mut HashMap<ExtendedRegisterEnum, LiftValue<'ctx>> {
        &mut self.regs_hashmap
    }

    pub(super) fn regs_hashmap(&self) -> &HashMap<ExtendedRegisterEnum, LiftValue<'ctx>> {
        &self.regs_hashmap
    }

    pub(super) fn get_register(&self, register: Register) -> Result<LiftValue<'ctx>> {
        let key = self.get_register_largest_enclosing(&register);
        if let Some(value) = self.regs_hashmap().get(&key.into()).copied() {
            return Ok(value);
        }
        Ok(self.get_max_int_type()?.const_zero().into())
    }
}
