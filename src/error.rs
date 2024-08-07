use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("No such month: {0:?}")]
    InvalidMonth(i8),

    #[error("No such time")]
    InvalidTime,

    #[error("{0}")]
    InvalidArgument(String),
}

impl std::convert::From<jiff::Error> for Error {
    fn from(err: jiff::Error) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}
