#![allow(clippy::excessive_precision)]

pub mod error;
pub mod hijri;
pub mod salah;
mod time;

pub use error::Error;

// Use internal type. Chrono API changes very often
pub type Date = chrono::NaiveDate;
pub type DateTime = chrono::NaiveDateTime;
