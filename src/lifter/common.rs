use crate::miscellaneous::ExtendedRegister;

use super::{PossibleLLVMValueEnum, Result};

use inkwell::values::IntValue;
use zydis::{
    ffi::{DecodedOperand, DecodedOperandKind, MemoryInfo},
    MachineMode, Register,
};

use super::LifterX86;

impl<'ctx> LifterX86<'ctx> {
    //pub(super) fn experimental_get_register(
    //    &self,
    //    r: Register,
    //) -> Result<PossibleLLVMValueEnum<'ctx>> {
    //    let pr = r.largest_enclosing(self.mode);
    //    let regs_hashmap = self.regs_hashmap.borrow();
    //    regs_hashmap
    //        .get(&pr.into())
    //        .copied()
    //        .ok_or(Error::RegUnwrapError(pr.into()))
    //}

    /// Wrapper because of zydis largest_enclosing doesnt work correctly with SP
    pub(super) fn get_register_largest_enclosing(&self, register: &Register) -> ExtendedRegister {
        let pr = if [Register::RBP, Register::EBP, Register::BP].contains(register) {
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
        };

        pr.into()
    }
    pub(super) fn get_register(&self, r: Register) -> Result<PossibleLLVMValueEnum<'ctx>> {
        let pr = self.get_register_largest_enclosing(&r);
        //let pr = r.largest_enclosing(self.mode);
        let regs_hashmap = self.regs_hashmap.get();
        let lookup_result = unsafe {
            (*regs_hashmap).get(&pr).copied()
            //.ok_or(Error::RegUnwrapError(pr))
        };

        let reg_val = match lookup_result {
            Some(val) => val,
            //None => self.get_max_int_type().get_undef().into(),
            None => self.get_max_int_type().const_zero().into(),
        };

        Ok(reg_val)
    }

    pub(crate) fn calc_mem_operand(&self, mem: &MemoryInfo) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;

        let base_r: Option<IntValue> = self
            .load_reg_internal(&mem.base)
            .ok()
            .map(TryInto::try_into)
            .map(|v| v.unwrap());

        let t = match base_r {
            Some(int_val) => int_val.get_type(),
            None => self.retdec_get_default_type(),
        };

        let disp = if mem.disp.has_displacement {
            let disp_val = mem.disp.displacement as u64;

            // Not sure abt extend
            Some(t.const_int(disp_val, true))
        } else {
            None
        };

        let mut idx_r: Option<IntValue> = self
            .load_reg_internal(&mem.base)
            .ok()
            .map(TryInto::try_into)
            .map(|v| v.unwrap());

        if let Some(idx_r_) = idx_r {
            let scale = idx_r_.get_type().const_int(mem.scale.into(), false);
            idx_r = Some(builder.build_int_mul(scale, scale, "")?);
        }

        let addr = if let (Some(base_r), None) = (base_r, disp) {
            base_r
        } else if let (Some(disp), None) = (disp, base_r) {
            disp
        } else if let (Some(base_r_), Some(disp_)) = (base_r, disp) {
            let disp_new = self.create_s_ext_or_trunc(disp_, base_r_.get_type())?;
            builder.build_int_add(base_r_, disp_new, "")?
        } else if let Some(idx_r) = idx_r {
            idx_r
        } else {
            self.retdec_get_default_type().const_zero()
        };

        Ok(addr)
    }

    // Used only in 'lea' mnemonic for now
    pub(crate) fn get_effective_address(
        &self,
        op: &DecodedOperand,
        possible_size: u16,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let i64_type = self.context.i64_type();

        let mem = if let DecodedOperandKind::Mem(mem) = &op.kind {
            mem
        } else {
            unreachable!("Got a weird/buggy lea instruction")
        };

        let mut base_value = None;
        if let Ok(base) = self.load_reg_internal(&mem.base) {
            let base_into_int: IntValue<'ctx> = base.try_into()?;
            base_value = Some(builder.build_int_z_extend(base_into_int, i64_type, "")?);
        }

        let mut index_value = None;
        if let Ok(index) = self.load_reg_internal(&mem.index) {
            let index_2 = builder.build_int_z_extend(index.try_into()?, i64_type, "")?;
            if mem.scale > 1 {
                let scale_value = i64_type.const_int(mem.scale.into(), false);
                index_value = Some(builder.build_int_mul(index_2, scale_value, "mul_ea_")?);
            }
        }

        let effective_address =
            if let [Some(base_value), Some(index_value)] = [base_value, index_value] {
                builder.build_int_add(base_value, index_value, "bvalue_indexvalue_set")?
            } else if let Some(base_value) = base_value {
                base_value
            } else if let Some(index_value) = index_value {
                index_value
            } else {
                i64_type.const_zero()
            };

        let final_value = if mem.disp.has_displacement {
            i64_type.const_int(mem.disp.displacement.try_into().unwrap(), false)
        } else {
            effective_address
        };

        let target_sized_type = self.context.custom_width_int_type(possible_size.into());
        let retval = self.create_z_ext_or_trunc(final_value, target_sized_type)?;

        Ok(retval)
    }
}
