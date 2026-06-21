use super::{Error, Result};

use llvmkit::ir::{
    ConstantIntValue, FloatDyn, FloatKind, FloatType, FloatValue, IntDyn, IntType, IntValue,
    IntWidth,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum LiftValue<'ctx> {
    Int(IntValue<'ctx, IntDyn>),
    Float(FloatValue<'ctx, FloatDyn>),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum LiftType<'ctx> {
    Int(IntType<'ctx, IntDyn>),
    Float(FloatType<'ctx, FloatDyn>),
}

impl<'ctx, W: IntWidth> From<IntValue<'ctx, W>> for LiftValue<'ctx> {
    fn from(value: IntValue<'ctx, W>) -> Self {
        Self::Int(value.as_dyn())
    }
}

impl<'ctx, W: IntWidth> From<ConstantIntValue<'ctx, W>> for LiftValue<'ctx> {
    fn from(value: ConstantIntValue<'ctx, W>) -> Self {
        let int_value = IntValue::<IntDyn>::try_from(value.as_value())
            .unwrap_or_else(|_| unreachable!("ConstantIntValue always wraps an integer value"));
        Self::Int(int_value)
    }
}

impl<'ctx, K: FloatKind> From<FloatValue<'ctx, K>> for LiftValue<'ctx> {
    fn from(value: FloatValue<'ctx, K>) -> Self {
        Self::Float(value.as_dyn())
    }
}

impl<'ctx> From<IntType<'ctx, IntDyn>> for LiftType<'ctx> {
    fn from(value: IntType<'ctx, IntDyn>) -> Self {
        Self::Int(value)
    }
}

impl<'ctx> From<FloatType<'ctx, FloatDyn>> for LiftType<'ctx> {
    fn from(value: FloatType<'ctx, FloatDyn>) -> Self {
        Self::Float(value)
    }
}

impl<'ctx> TryFrom<LiftValue<'ctx>> for IntValue<'ctx, IntDyn> {
    type Error = Error;

    fn try_from(value: LiftValue<'ctx>) -> Result<Self> {
        if let LiftValue::Int(int_val) = value {
            Ok(int_val)
        } else {
            Err(Error::ConvertError)
        }
    }
}

impl<'ctx> TryFrom<LiftValue<'ctx>> for FloatValue<'ctx, FloatDyn> {
    type Error = Error;

    fn try_from(value: LiftValue<'ctx>) -> Result<Self> {
        if let LiftValue::Float(float_val) = value {
            Ok(float_val)
        } else {
            Err(Error::ConvertError)
        }
    }
}

impl<'ctx> TryFrom<LiftType<'ctx>> for IntType<'ctx, IntDyn> {
    type Error = Error;

    fn try_from(value: LiftType<'ctx>) -> Result<Self> {
        if let LiftType::Int(int_ty) = value {
            Ok(int_ty)
        } else {
            Err(Error::ConvertError)
        }
    }
}

impl<'ctx> TryFrom<LiftType<'ctx>> for FloatType<'ctx, FloatDyn> {
    type Error = Error;

    fn try_from(value: LiftType<'ctx>) -> Result<Self> {
        if let LiftType::Float(float_ty) = value {
            Ok(float_ty)
        } else {
            Err(Error::ConvertError)
        }
    }
}
