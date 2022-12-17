#![allow(clippy::excessive_precision)]

mod baselib;
pub mod hijri;
pub mod pray;

// A convenience module for islam cuonsumer to use
pub mod time {
    pub use time::OffsetDateTime;
}
