use chrono::{Local, NaiveDate, NaiveTime};

use crate::{Date, DateTime};

pub fn now() -> DateTime {
    Local::now().naive_local()
}

pub fn one_sec_before_midnight() -> Option<DateTime> {
    let midnight = NaiveTime::from_hms_opt(23, 59, 59)?;
    let date = Local::now().date_naive();

    Some(date.and_time(midnight))
}

pub fn midnight() -> Option<DateTime> {
    let midnight = NaiveTime::from_hms_opt(0, 0, 00)?;
    let date = Local::now().date_naive();

    Some(date.and_time(midnight))
}

pub fn today() -> Date {
    Local::now().date_naive()
}

pub fn date(year: i32, month: u32, day: u32) -> Result<Date, crate::Error> {
    NaiveDate::from_ymd_opt(year, month, day).ok_or(crate::Error::InvalidTime)
}
