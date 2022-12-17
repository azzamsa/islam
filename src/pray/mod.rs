mod config;
pub mod error;
mod madhab;
mod method;
mod prayer;
mod times;

// shorter access for library consumer
pub use config::Config;
pub use madhab::Madhab;
pub use method::Method;
pub use prayer::Prayer;
pub use times::{Location, PrayerSchedule, PrayerTimes};

use time::{Date, OffsetDateTime};

use self::error::Error;

fn today() -> Result<Date, Error> {
    Ok(OffsetDateTime::now_local()?.date())
}
