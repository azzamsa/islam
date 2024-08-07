#![allow(clippy::excessive_precision)]

pub mod error;
pub mod hijri;
pub mod salah;
mod time;

pub use error::Error;

// Re-eport time library;
pub use jiff;
