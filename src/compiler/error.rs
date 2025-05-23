use inkwell::{builder::BuilderError, support::LLVMString};
use thiserror::Error;

pub(crate) type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    LifterError(#[from] crate::lifter::Error),
    #[error(transparent)]
    Builder(#[from] BuilderError),

    #[error("Unable to create targetMachine")]
    UnableToCreateTargetMachine,

    #[error("An error occured while running optimizations: {0}")]
    OptimizationsError(LLVMString),
}
