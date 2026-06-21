use llvmkit::ir::IrError;
use thiserror::Error;

use super::ExtendedRegisterEnum;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to convert")]
    ConvertError,
    #[error("Tried to convert zydis::Register::NONE to something")]
    RegisterConverError,
    #[error("Tried to unwrap a register which doesn't exist. {0:?}")]
    RegUnwrapError(ExtendedRegisterEnum),
    #[error("Unable to resolve flag from number: {0}")]
    FlagResolveError(u64),
    #[error("{0}")]
    UnsupportedInstr(&'static str),
    #[error("function already has a terminator")]
    FunctionAlreadyTerminated,
    #[error(transparent)]
    Ir(#[from] IrError),
}
