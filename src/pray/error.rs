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
