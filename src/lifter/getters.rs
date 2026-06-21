use super::{Error, LiftValue, LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{IntDyn, IntValue};
use zydis::{
    ffi::{DecodedOperand, DecodedOperandKind, ImmediateInfo},
    Register, RegisterClass,
};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(crate) fn load_register_value(&self, register: &Register) -> Result<LiftValue<'ctx>> {
        if register == &Register::NONE {
            return Err(Error::RegUnwrapError(ExtendedRegisterEnum::NONE));
        }
        if register.class() == RegisterClass::IP {
            let int_ty = self.get_max_int_type()?;
            let value: IntValue<'ctx, IntDyn> =
                if let Some(runtime_address) = self.runtime_address() {
                    int_ty
                        .const_int_raw(runtime_address, false)?
                        .as_value()
                        .try_into()?
                } else {
                    int_ty.as_type().get_undef().as_value().try_into()?
                };
            return Ok(value.into());
        }
        if register.class() == RegisterClass::FLAGS {
            return Ok(self.get_rflags_value()?.into());
        }
        if [Register::AH, Register::CH, Register::DH, Register::BH].contains(register) {
            let full_value: IntValue<'ctx, IntDyn> = self.get_register(*register)?.try_into()?;
            let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
                full_value,
                full_value.ty().const_int_raw(8, false)?,
                "high_byte",
            )?;
            let byte = self.create_z_ext_or_trunc(shifted, self.module.i8_type().as_dyn())?;
            return Ok(byte.into());
        }
        self.get_register(*register)
    }

    pub(super) fn load_single_int_op(
        &self,
        operand: &DecodedOperand,
        possible_size: u16,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let value: IntValue<'ctx, IntDyn> =
            self.load_single_op(operand, possible_size)?.try_into()?;
        let ty = self
            .module
            .custom_width_int_type(u32::from(possible_size))?;
        self.create_z_ext_or_trunc(value, ty)
    }

    pub(super) fn load_single_op(
        &self,
        operand: &DecodedOperand,
        possible_size: u16,
    ) -> Result<LiftValue<'ctx>> {
        match &operand.kind {
            DecodedOperandKind::Reg(register) => self.load_register_value(register),
            DecodedOperandKind::Imm(imm) => Ok(self.load_imm_internal(imm, possible_size)?.into()),
            DecodedOperandKind::Mem(mem) => {
                Ok(self.mergen_load_mem(mem, u32::from(possible_size))?.into())
            }
            DecodedOperandKind::Unused | DecodedOperandKind::Ptr(_) => {
                Err(Error::UnsupportedInstr("unsupported operand kind"))
            }
        }
    }

    fn load_imm_internal(
        &self,
        imm: &ImmediateInfo,
        possible_size: u16,
    ) -> Result<IntValue<'ctx, IntDyn>> {
        let ty = self
            .module
            .custom_width_int_type(u32::from(possible_size))?;
        Ok(ty
            .const_int_raw(imm.value, imm.is_signed)?
            .as_value()
            .try_into()?)
    }

    pub(super) fn load_flag(&self, flag: ExtendedRegisterEnum) -> Result<IntValue<'ctx, IntDyn>> {
        match self.regs_hashmap().get(&flag).copied() {
            Some(value) => value.try_into(),
            None => Ok(self.module.bool_type().const_zero().as_value().try_into()?),
        }
    }

    pub(super) fn resolve_flag_from_range(range_int: u64) -> Result<ExtendedRegisterEnum> {
        match range_int {
            0 => Ok(ExtendedRegisterEnum::CF),
            1 => Ok(ExtendedRegisterEnum::Reserved1),
            2 => Ok(ExtendedRegisterEnum::PF),
            3 => Ok(ExtendedRegisterEnum::Reserved3),
            4 => Ok(ExtendedRegisterEnum::AF),
            5 => Ok(ExtendedRegisterEnum::Reserved5),
            6 => Ok(ExtendedRegisterEnum::ZF),
            7 => Ok(ExtendedRegisterEnum::SF),
            8 => Ok(ExtendedRegisterEnum::TF),
            9 => Ok(ExtendedRegisterEnum::IF),
            10 => Ok(ExtendedRegisterEnum::DF),
            11 => Ok(ExtendedRegisterEnum::OF),
            12 => Ok(ExtendedRegisterEnum::IOPL),
            _ => Err(Error::FlagResolveError(range_int)),
        }
    }

    pub(super) fn get_rflags_value(&self) -> Result<IntValue<'ctx, IntDyn>> {
        let flag_ty = self.get_max_int_type()?;
        let mut rflags: IntValue<'ctx, IntDyn> = flag_ty.const_zero().as_value().try_into()?;
        for flag in 0_u64..12 {
            let flag_value = self.load_flag(Self::resolve_flag_from_range(flag)?)?;
            let shifted_flag = self.builder()?.build_int_shl::<IntDyn, _, _, _>(
                self.create_z_ext_or_trunc(flag_value, flag_ty)?,
                flag_ty.const_int_raw(flag, false)?,
                "",
            )?;
            rflags = self
                .builder()?
                .build_int_or::<IntDyn, _, _, _>(rflags, shifted_flag, "")?;
        }
        Ok(rflags)
    }
}
