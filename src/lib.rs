#![allow(clippy::excessive_precision)]

mod baselib;
pub mod hijri;
pub mod pray;
pub mod time;

// Use internal type. Chrono API changes very often
type Date = chrono::NaiveDate;
type DateTime = chrono::NaiveDateTime;
