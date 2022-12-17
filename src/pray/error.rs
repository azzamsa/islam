use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("No such month: {0:?}")]
    InvalidMonth(u8),

    #[error("No such time: {0:?}")]
    InvalidTime(String),

    #[error("{0}")]
    InvalidArgument(String),
}

impl std::convert::From<time::error::ComponentRange> for Error {
    fn from(err: time::error::ComponentRange) -> Self {
        Self::InvalidTime(err.to_string())
    }
}

impl std::convert::From<time::error::IndeterminateOffset> for Error {
    fn from(err: time::error::IndeterminateOffset) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}
