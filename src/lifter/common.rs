use super::{PossibleLLVMValueEnum, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use std::collections::HashMap;

use inkwell::values::IntValue;
use zydis::{
    ffi::{DecodedOperand, DecodedOperandKind, MemoryInfo},
    MachineMode, Register,
};

use super::LifterX86;

impl<'ctx> LifterX86<'ctx> {
    #[allow(clippy::mut_from_ref)]
    pub(super) fn regs_hashmap_mut(
        &self,
    ) -> &mut HashMap<ExtendedRegisterEnum, PossibleLLVMValueEnum<'ctx>> {
        unsafe { &mut (*self.regs_hashmap.get()) }
    }

    #[allow(clippy::mut_from_ref)]
    pub(super) fn regs_hashmap(
        &self,
    ) -> &HashMap<ExtendedRegisterEnum, PossibleLLVMValueEnum<'ctx>> {
        unsafe { &(*self.regs_hashmap.get()) }
    }

    /// Wrapper because of zydis largest_enclosing doesnt work correctly with SP
    pub(super) fn get_register_largest_enclosing(&self, register: &Register) -> Register {
        if [Register::RBP, Register::EBP, Register::BP].contains(register) {
            match self.mode {
                MachineMode::LONG_64 => Register::RBP,
                MachineMode::LEGACY_32 => Register::EBP,
                _ => unimplemented!("Only 32 and 64 bit supported right now"),
            }
        } else if [Register::RSP, Register::ESP, Register::SP].contains(register) {
            match self.mode {
                MachineMode::LONG_64 => Register::RSP,
                MachineMode::LEGACY_32 => Register::ESP,
                _ => unimplemented!("Only 32 and 64 bit supported right now"),
            }
        } else {
            register.largest_enclosing(self.mode)
        }
    }
    pub(super) fn get_register(&self, r: Register) -> Result<PossibleLLVMValueEnum<'ctx>> {
        let pr = self.get_register_largest_enclosing(&r);

        let lookup_result = self.regs_hashmap().get(&pr.into()).copied();

        let reg_val = match lookup_result {
            Some(val) => val,
            //None => self.get_max_int_type().get_undef().into(),
            None => self.get_max_int_type().const_zero().into(),
        };

        Ok(reg_val)
    }
}
