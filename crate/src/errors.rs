use miette::Diagnostic;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, OctolotlError>;

#[derive(Debug, Diagnostic, Error)]
pub enum OctolotlError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
