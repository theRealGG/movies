#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IllegalArgument(String),
    #[error("A unkown error occurred")]
    UnknownError,
}

pub type ApiError<T> = Result<T, Error>;
