use inkwell::builder::BuilderError;
use thiserror::Error;

pub(crate) type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    LifterError(#[from] crate::lifter::Error),
    #[error(transparent)]
    Builder(#[from] BuilderError),
}
