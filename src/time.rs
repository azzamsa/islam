use chrono::{Local, TimeZone};

use crate::{LocalDate, LocalDateTime};

pub fn now() -> LocalDateTime {
    Local::now()
}

pub fn today() -> LocalDate {
    Local::now().date()
}

pub fn date(year: i32, month: u32, day: u32) -> LocalDate {
    Local.ymd(year, month, day)
}
