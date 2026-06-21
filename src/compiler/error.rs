use llvmkit::ir::IrError;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    LifterError(#[from] crate::lifter::Error),
    #[error(transparent)]
    Ir(#[from] IrError),
}
