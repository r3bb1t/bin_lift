use super::{Error, Result};

use inkwell::{
    types::{FloatType, IntType},
    values::{BasicValueEnum, FloatValue, IntValue},
};

/// Basically shrinked [BasicValueEnum]
/// (https://thedan64.github.io/inkwell/inkwell/values/enum.BasicValueEnum.html)
#[derive(Debug, Clone, Copy)]
pub(crate) enum PossibleLLVMValueEnum<'ctx> {
    IntValue(IntValue<'ctx>),
    FloatValue(FloatValue<'ctx>),
}

/// Basically shrinked [BasicValueEnum]
/// (https://thedan64.github.io/inkwell/inkwell/types/enum.BasicTypeEnum.html)
#[derive(Debug, Clone, Copy)]
pub(crate) enum PossibleLLVMTypeEnum<'ctx> {
    IntType(IntType<'ctx>),
    FloatType(FloatType<'ctx>),
}

impl<'ctx> From<IntValue<'ctx>> for PossibleLLVMValueEnum<'ctx> {
    fn from(value: IntValue<'ctx>) -> Self {
        Self::IntValue(value)
    }
}

impl<'ctx> From<FloatValue<'ctx>> for PossibleLLVMValueEnum<'ctx> {
    fn from(value: FloatValue<'ctx>) -> Self {
        Self::FloatValue(value)
    }
}

impl<'ctx> From<PossibleLLVMValueEnum<'ctx>> for BasicValueEnum<'ctx> {
    fn from(value: PossibleLLVMValueEnum<'ctx>) -> Self {
        match value {
            PossibleLLVMValueEnum::IntValue(int_value) => Self::IntValue(int_value),
            PossibleLLVMValueEnum::FloatValue(float_value) => Self::FloatValue(float_value),
        }
    }
}

impl<'ctx> From<IntType<'ctx>> for PossibleLLVMTypeEnum<'ctx> {
    fn from(value: IntType<'ctx>) -> Self {
        Self::IntType(value)
    }
}

impl<'ctx> From<FloatType<'ctx>> for PossibleLLVMTypeEnum<'ctx> {
    fn from(value: FloatType<'ctx>) -> Self {
        Self::FloatType(value)
    }
}

impl<'ctx> TryFrom<PossibleLLVMTypeEnum<'ctx>> for IntType<'ctx> {
    type Error = Error;

    fn try_from(value: PossibleLLVMTypeEnum<'ctx>) -> Result<Self> {
        if let PossibleLLVMTypeEnum::IntType(int_ty) = value {
            Ok(int_ty)
        } else {
            Err(Error::ConvertError)
        }
    }
}

impl<'ctx> TryFrom<PossibleLLVMTypeEnum<'ctx>> for FloatType<'ctx> {
    type Error = Error;

    fn try_from(value: PossibleLLVMTypeEnum<'ctx>) -> Result<Self> {
        if let PossibleLLVMTypeEnum::FloatType(float_ty) = value {
            Ok(float_ty)
        } else {
            Err(Error::ConvertError)
        }
    }
}

impl<'ctx> TryFrom<BasicValueEnum<'ctx>> for PossibleLLVMValueEnum<'ctx> {
    type Error = Error;

    fn try_from(value: BasicValueEnum<'ctx>) -> Result<Self> {
        match value {
            BasicValueEnum::IntValue(int_value) => Ok(Self::IntValue(int_value)),
            BasicValueEnum::FloatValue(float_value) => Ok(Self::FloatValue(float_value)),
            _ => Err(Error::ConvertError),
        }
    }
}

impl<'ctx> TryFrom<PossibleLLVMValueEnum<'ctx>> for IntValue<'ctx> {
    type Error = Error;

    fn try_from(value: PossibleLLVMValueEnum<'ctx>) -> Result<Self> {
        if let PossibleLLVMValueEnum::IntValue(int_val) = value {
            Ok(int_val)
        } else {
            Err(Error::ConvertError)
        }
    }
}
