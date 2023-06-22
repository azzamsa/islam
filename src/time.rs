use chrono::{Local, NaiveDate};

use crate::{Date, DateTime};

pub fn now() -> DateTime {
    Local::now().naive_local()
}

pub fn today() -> Date {
    Local::now().date_naive()
}

pub fn date(year: i32, month: u32, day: u32) -> Result<Date, crate::Error> {
    Ok(NaiveDate::from_ymd_opt(year, month, day).unwrap())
}
