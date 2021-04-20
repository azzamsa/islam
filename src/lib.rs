#![allow(
    clippy::must_use_candidate,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::missing_errors_doc,
    clippy::excessive_precision,
    clippy::new_without_default,
    clippy::cast_possible_wrap,
    clippy::missing_const_for_fn,
    clippy::cast_sign_loss
)]

mod baselib;
pub mod hijri;
pub mod pray;

// A convenience module for islam cuonsumer to use
pub mod chrono {
    // `.year`, `.day` needs `Datelike`
    // `.ymd` needs `Timezone`
    pub use chrono::{Date, DateTime, Datelike, Duration, Local, TimeZone, Timelike, Utc};
}
