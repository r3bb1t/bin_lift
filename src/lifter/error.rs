use inkwell::{builder::BuilderError, support::LLVMString};
use thiserror::Error;

use super::ExtendedRegisterEnum;

pub(crate) type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to convert")]
    ConvertError,
    #[error(transparent)]
    Builder(#[from] BuilderError),
    #[error("Tried to convert zydis::Register::NONE to something")]
    RegisterConverError,
    #[error("Tried to unwrap a register which doesn't exist. {0:?}")]
    RegUnwrapError(ExtendedRegisterEnum),
    #[error(transparent)]
    RunPassesError(LLVMString),
    #[error("Can't find the following LLVM intrinsic: {0}")]
    IntrinsicNotFound(&'static str),

    #[error("Unable to resolve flag from number: {0}")]
    FlagResolveError(u64),

    #[error("{0}")]
    UnsupportedInstr(&'static str),
}
