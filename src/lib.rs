#![allow(clippy::excessive_precision)]

mod baselib;
pub mod hijri;
pub mod pray;
pub mod time;

type LocalDate = chrono::Date<chrono::Local>;
type LocalDateTime = chrono::DateTime<chrono::Local>;
