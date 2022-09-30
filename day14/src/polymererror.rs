use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PolymerError {
    #[error("Parse error")]
    ParseError,
    #[error("Unknown error")]
    UnknownError,
}
