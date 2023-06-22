mod config;
pub mod error;
mod madhab;
mod method;
mod prayer;
mod times;

// // shorter access for library consumer
pub use config::Config;
pub use madhab::Madhab;
pub use method::Method;
pub use prayer::Prayer;
pub use times::{Location, PrayerSchedule, PrayerTimes};
