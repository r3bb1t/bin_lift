use super::{Error, LiftValue, LifterX86, Result};

use llvmkit::ir::{IntDyn, IntValue, PointerValue};
use zydis::{ffi::MemoryInfo, Register};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn mergen_store_mem(
        &mut self,
        mem: &MemoryInfo,
        val: LiftValue<'ctx>,
    ) -> Result<()> {
        let pointer = self.mergen_calculate_memory_operand(mem)?;
        let value: IntValue<'ctx, IntDyn> = val.try_into()?;
        self.builder()?.build_store(value, pointer)?;
        Ok(())
    }

    pub(super) fn mergen_load_mem(
        &self,
        mem: &MemoryInfo,
        possible_size: u32,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let pointer = self.mergen_calculate_memory_operand(mem)?;
        let load_type = self.module.custom_width_int_type(possible_size)?;
        Ok(self.builder()?.build_int_load_dyn(load_type, pointer, "")?)
    }

    pub(super) fn mergen_calculate_memory_operand(
        &self,
        mem: &MemoryInfo,
    ) -> Result<PointerValue<'ctx>> {
        let effective_address = self.mergen_get_effective_address(mem)?;
        Ok(self.builder()?.build_gep(
            self.module.i8_type(),
            self.stackmemory,
            [effective_address],
            "",
        )?)
    }

    pub(crate) fn mergen_get_effective_address(
        &self,
        mem: &MemoryInfo,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        if mem.segment == Register::GS {
            return Err(Error::UnsupportedInstr("gs segment memory"));
        }

        let addr_ty = self.get_max_int_type()?;
        let builder = self.builder()?;
        let base_value = if mem.base != Register::NONE {
            let base: IntValue<'ctx, IntDyn> = self.get_register(mem.base)?.try_into()?;
            Some(self.create_z_ext_or_trunc(base, addr_ty)?)
        } else {
            None
        };
        let index_value = if mem.index != Register::NONE {
            let index: IntValue<'ctx, IntDyn> = self.get_register(mem.index)?.try_into()?;
            let index = self.create_z_ext_or_trunc(index, addr_ty)?;
            if mem.scale > 1 {
                Some(builder.build_int_mul::<IntDyn, _, _, _>(
                    index,
                    addr_ty.const_int_raw(u64::from(mem.scale), false)?,
                    "",
                )?)
            } else {
                Some(index)
            }
        } else {
            None
        };

        let mut effective_address = match (base_value, index_value) {
            (Some(base), Some(index)) => {
                builder.build_int_add::<IntDyn, _, _, _>(base, index, "effective_address_")?
            }
            (Some(base), None) => base,
            (None, Some(index)) => index,
            (None, None) => addr_ty.const_zero().as_value().try_into()?,
        };

        if mem.disp.displacement != 0 {
            let displacement = u64::from_ne_bytes(mem.disp.displacement.to_ne_bytes());
            effective_address = builder.build_int_add::<IntDyn, _, _, _>(
                effective_address,
                addr_ty.const_int_raw(displacement, true)?,
                "",
            )?;
        }

        Ok(effective_address)
    }
}
