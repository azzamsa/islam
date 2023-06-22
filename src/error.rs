use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("No such month: {0:?}")]
    InvalidMonth(u32),

    #[error("No such time")]
    InvalidTime,

    #[error("{0}")]
    InvalidArgument(String),
}
