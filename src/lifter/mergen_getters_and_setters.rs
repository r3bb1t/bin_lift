use super::{definintions::PossibleLLVMValueEnum, LifterX86, Result};

use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use zydis::{ffi::MemoryInfo, Register};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn mergen_store_mem(
        &self,
        mem: &MemoryInfo,
        val: PossibleLLVMValueEnum<'ctx>,
    ) -> Result<()> {
        let pointer = self.mergen_calculate_memory_operand(mem)?;

        self.builder
            .build_store(pointer, BasicValueEnum::from(val))?;

        Ok(())
    }

    pub(super) fn mergen_load_mem(
        &self,
        mem: &MemoryInfo,
        possible_size: u32,
    ) -> Result<IntValue<'ctx>> {
        let pointer = self.mergen_calculate_memory_operand(mem)?;

        let load_type = self.context.custom_width_int_type(possible_size);
        // NOTE: revisit AddressSpace in future maybe
        let retval = self
            .builder
            .build_load(load_type, pointer, "")?
            .into_int_value();

        Ok(retval)
    }

    pub(crate) fn mergen_get_register(
        &self,
        reg: &Register,
        possible_size: u32,
    ) -> Result<PossibleLLVMValueEnum<'ctx>> {
        let possible_size_type = self.context.custom_width_int_type(possible_size);
        let value = self.load_register_value(reg)?;

        if let PossibleLLVMValueEnum::IntValue(int_val) = value {
            let type_bit_width = int_val.get_type().get_bit_width();

            if type_bit_width < 128 {
                let value_zext =
                    self.create_z_ext_or_trunc(value.try_into()?, possible_size_type)?;
                return Ok(value_zext.into());
            }
        }
        Ok(value)
    }

    pub(crate) fn mergen_calculate_memory_operand(
        &self,
        mem: &MemoryInfo,
    ) -> Result<PointerValue<'ctx>> {
        let effective_address = self.mergen_get_effective_address(mem)?;

        let memory_opperand = if mem.segment == Register::GS {
            unimplemented!("TEB support isn't added yet");
        } else {
            self.stackmemory
        };

        let pointer = unsafe {
            self.builder.build_gep(
                self.context.i8_type(),
                memory_opperand,
                &[effective_address],
                "",
            )?
        };

        Ok(pointer)
    }

    // Used directly only here and by LEA
    pub(crate) fn mergen_get_effective_address(&self, mem: &MemoryInfo) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;
        let ctx = self.context;

        let i_64_ty = ctx.i64_type();

        let base_value = if mem.base != Register::NONE {
            let base_value: IntValue<'_> = self.get_register(mem.base)?.try_into()?;
            // TODO: consider not hardcoding in future
            let base_value_z_ext = builder.build_int_z_extend(base_value, i_64_ty, "")?;
            Some(base_value_z_ext)
        } else {
            None
        };

        let scale_value = if mem.index != Register::NONE {
            let index_value: IntValue<'_> = self.get_register(mem.index)?.try_into()?;
            let index_value_z_ext = builder.build_int_z_extend(index_value, i_64_ty, "")?;

            if mem.scale > 1 {
                let scale_value = i_64_ty.const_int(mem.scale.into(), false);
                let index_value_multiplied_by_scale =
                    builder.build_int_mul(index_value_z_ext, scale_value, "")?;

                Some(index_value_multiplied_by_scale)
            } else {
                Some(index_value_z_ext)
            }
        } else {
            None
        };

        let mut effective_address = if let [Some(base), Some(scale)] = [base_value, scale_value] {
            builder.build_int_add(base, scale, "effective_address_")?
        } else if let Some(base) = base_value {
            base
        } else if let Some(scale) = scale_value {
            scale
        } else {
            // TODO: use error here maybe ???
            i_64_ty.const_zero()
        };

        if mem.disp.displacement != 0 {
            // TODO: Check it
            let disp_value = i_64_ty.const_int(mem.disp.displacement as _, false);
            effective_address = builder.build_int_add(effective_address, disp_value, "")?;
        }

        Ok(effective_address)
    }
}
