use inkwell::{
    builder::BuilderError,
    types::{BasicType, BasicTypeEnum, IntType},
    values::{BasicValue, BasicValueEnum, IntValue},
};
use zydis::{ffi::MemoryInfo, Register, RegisterClass};

use crate::util;

use super::{eOpConv, LifterX86};

impl<'ctx> LifterX86<'ctx> {
    pub fn retdec_calc_mem_operand(
        &self,
        mem: &MemoryInfo,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        let builder = &self.builder;

        let base_r = self
            .retdec_load_reg(&mem.base, None, None)?
            .map(|val| val.into_int_value());

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

        let mut idx_r = self
            .retdec_load_reg(&mem.index, None, None)?
            .map(|val| val.into_int_value());

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

    pub fn retdec_get_default_type(&self) -> IntType<'ctx> {
        self.retdec_get_integer_type_from_byte_size(self.retdec_get_arch_byte_size().into())
    }

    pub fn retdec_get_arch_byte_size(&self) -> u8 {
        match self.mode {
            zydis::MachineMode::LONG_64 => 8,
            zydis::MachineMode::LONG_COMPAT_32 => 4,
            zydis::MachineMode::LONG_COMPAT_16 => 2,
            zydis::MachineMode::LEGACY_32 => 4,
            zydis::MachineMode::LEGACY_16 => 2,
            zydis::MachineMode::REAL_16 => 2,
        }
    }

    pub fn retdec_get_integer_type_from_byte_size(&self, size: u32) -> IntType<'ctx> {
        let sz = size * 8;
        util::get_int_n_ty(self.context, sz)
    }

    pub fn get_register_type(&self, reg: Register) -> BasicTypeEnum<'ctx> {
        let ctx = self.context;
        match reg.class() {
            RegisterClass::GPR8 => ctx.i8_type().as_basic_type_enum(),
            RegisterClass::GPR16 => ctx.i16_type().as_basic_type_enum(),
            RegisterClass::GPR32 => ctx.i32_type().as_basic_type_enum(),
            RegisterClass::GPR64 => ctx.i64_type().as_basic_type_enum(),
            RegisterClass::MMX => ctx.f64_type().as_basic_type_enum(),
            RegisterClass::XMM => ctx.i128_type().as_basic_type_enum(),
            RegisterClass::YMM => panic!("LLVM Doesn't support 256 bit floats"),
            RegisterClass::ZMM => panic!("LLVM Doesn't support 512 bit floats"),
            RegisterClass::FLAGS => {
                util::get_int_ty(ctx, reg.width(self.mode).into()).as_basic_type_enum()
            }
            RegisterClass::IP => {
                util::get_int_ty(ctx, reg.width(self.mode).into()).as_basic_type_enum()
            }
            RegisterClass::SEGMENT => ctx.i16_type().as_basic_type_enum(),
            RegisterClass::INVALID => panic!("Invalid register"),
            _ => util::get_int_ty(ctx, reg.width(self.mode).into()).as_basic_type_enum(),
        }
    }

    pub fn retdec_generate_type_conversion(
        &self,
        from: BasicValueEnum<'ctx>,
        to: Option<BasicTypeEnum<'ctx>>,
        ct: eOpConv,
    ) -> Result<BasicValueEnum<'ctx>, BuilderError> {
        if let Some(to) = to {
            if from.get_type() == to {
                return Ok(from);
            }

            match ct {
                eOpConv::SEXT_TRUNC_OR_BITCAST | eOpConv::ZEXT_TRUNC_OR_BITCAST => {
                    if !to.is_int_type() {
                        panic!("Invalid combination of conversion method and destination type")
                    }
                    let to = to.into_int_type();
                    if let BasicValueEnum::IntValue(from) = from {
                        let r = if ct == eOpConv::SEXT_TRUNC_OR_BITCAST {
                            self.create_s_ext_or_trunc(from, to)?.as_basic_value_enum()
                        } else {
                            self.create_z_ext_or_trunc(from, to)?.as_basic_value_enum()
                        };

                        Ok(r)
                    } else {
                        let size = util::get_type_store_size_in_bits(from.get_type());
                        let int_ty = util::get_int_n_ty(self.context, size);
                        // FIXME: IDK. We seem to be crash here when working with pop instruction
                        let ret = self.builder.build_bit_cast(from, int_ty, "")?;
                        let ret2 = self.create_z_ext_or_trunc(ret.into_int_value(), to)?;
                        Ok(ret2.as_basic_value_enum())
                    }
                }
                // TODO: Replace with error
                eOpConv::Nothing => Ok(from),
                eOpConv::Throw => panic!("Called epOpConv throw"),
                eOpConv::FPCAST_OR_BITCAST
                | eOpConv::SITOFP_OR_FPCAST
                | eOpConv::UITOFP_OR_FPCAST => unimplemented!(),
            }
        } else {
            Ok(from)
        }
    }
}
