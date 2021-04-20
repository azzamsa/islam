#![allow(clippy::module_name_repetitions, clippy::non_ascii_literal)]

use std::usize;

use chrono::{Date, TimeZone, Utc};
use thiserror::Error;

use crate::baselib;

const ARABIC_MONTHS: [&str; 12] = [
    "محرم",
    "صفر",
    "ربيع الأول",
    "ربيع الثاني",
    "جمادى الأولى",
    "جمادى الثانية",
    "رجب",
    "شعبان",
    "رمضان",
    "شوال",
    "ذو القعدة",
    "ذو الحجة",
];

const ENGLISH_MONTHS: [&str; 12] = [
    "Moharram",
    "Safar",
    "Rabie-I",
    "Rabie-II",
    "Jumada-I",
    "Jumada-II",
    "Rajab",
    "Shaban",
    "Ramadan",
    "Shawwal",
    "Delqada",
    "Delhijja",
];

#[derive(Error, Debug, PartialEq)]
pub enum HijriError {
    #[error("No such time: {0:?}")]
    InvalidTime(u32),
}

#[derive(Debug, Clone)]
pub struct HijriDate {
    pub day: u32,
    pub month: u32,
    pub month_arabic: String,
    pub month_english: String,
    pub year: i32,
}

impl HijriDate {
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, HijriError> {
        if !(1..=12).contains(&month) {
            return Err(HijriError::InvalidTime(month));
        }
        Ok(Self {
            year,
            month,
            day,
            month_arabic: Self::month_arabic(month),
            month_english: Self::month_english(month),
        })
    }
    pub fn to_julian(&self) -> i32 {
        let date = Utc.ymd(self.year, self.month, self.day);
        baselib::hijri_to_julian(date)
    }
    pub fn to_gregorian(&self) -> Date<Utc> {
        let julian = self.to_julian();
        let (year, month, day) = baselib::julian_to_gregorian(julian as f32);
        Utc.ymd(year, month, day)
    }
    pub fn next_date(self) -> Self {
        let julian = self.to_julian();
        Self::from_julian(julian + 1, 0)
    }
    // NOTE (upstream) never used
    // fn is_last(self) -> bool {
    //     if self.month != self.next_date().month {
    //         return true;
    //     }
    //     return false;
    // }
    pub fn today(correction_val: i32) -> Self {
        Self::from_gregorian(Utc::today(), correction_val)
    }
    pub fn from_julian(julian_date: i32, correction_val: i32) -> Self {
        let (year, month, day) = baselib::julian_to_hijri(julian_date, correction_val);

        Self {
            year,
            month,
            day,
            month_arabic: Self::month_arabic(month),
            month_english: Self::month_english(month),
        }
    }
    fn month_arabic(month: u32) -> String {
        ARABIC_MONTHS[(month - 1) as usize].to_string()
    }
    fn month_english(month: u32) -> String {
        ENGLISH_MONTHS[(month - 1) as usize].to_string()
    }
    pub fn from_gregorian(date: Date<Utc>, correction_val: i32) -> Self {
        let (year, month, day) =
            baselib::julian_to_hijri(baselib::gregorian_to_julian(date) as i32, correction_val);

        Self {
            year,
            month,
            day,
            month_arabic: Self::month_arabic(month),
            month_english: Self::month_english(month),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hijri() {
        let hijri_date = HijriDate::new(1442, 8, 25).unwrap();
        assert_eq!(hijri_date.day, 25);
        assert_eq!(hijri_date.month, 8);
        assert_eq!(hijri_date.year, 1442);
    }
    #[test]
    fn tomorrow() {
        let hijri_date = HijriDate::new(1442, 8, 25).unwrap();
        let tomorrow = hijri_date.next_date();
        assert_eq!(tomorrow.day, 26);
        assert_eq!(tomorrow.month, 8);
        assert_eq!(tomorrow.year, 1442);
    }

    #[test]
    fn to_gregorian() {
        let hijri_date = HijriDate::new(1442, 8, 25).unwrap();
        let gregorian = hijri_date.to_gregorian();
        assert_eq!(gregorian, Utc.ymd(2021, 4, 13));
    }
    #[test]
    fn from_gregorian() {
        let hijri_from_gregorian = HijriDate::from_gregorian(Utc.ymd(2021, 4, 9), 0);
        assert_eq!(hijri_from_gregorian.day, 25); // FIXME: this should be 27
        assert_eq!(hijri_from_gregorian.month, 8);
        assert_eq!(hijri_from_gregorian.month_arabic, "شعبان".to_string());
        assert_eq!(hijri_from_gregorian.month_english, "Shaban".to_string());
        assert_eq!(hijri_from_gregorian.year, 1442);
    }
    #[test]
    fn from_gregorian_1() {
        let hijri_from_gregorian = HijriDate::from_gregorian(Utc.ymd(2020, 4, 18), 0);
        // tested against https://www.islamicfinder.org/islamic-calendar/2021/April/?type=Gregorian
        assert_eq!(hijri_from_gregorian.day, 23); // FIXME: this should be 25
        assert_eq!(hijri_from_gregorian.month, 8);
        assert_eq!(hijri_from_gregorian.month_english, "Shaban".to_string());
        assert_eq!(hijri_from_gregorian.year, 1441);
    }
    #[test]
    fn from_julian() {
        let hijri_from_julian = HijriDate::from_julian(2459313, 0);
        assert_eq!(hijri_from_julian.day, 25);
        assert_eq!(hijri_from_julian.month, 8);
        assert_eq!(hijri_from_julian.month_arabic, "شعبان".to_string());
        assert_eq!(hijri_from_julian.month_english, "Shaban".to_string());
        assert_eq!(hijri_from_julian.year, 1442);
    }
    #[test]
    fn min_month() {
        let hijri_date = HijriDate::new(1442, 1, 25).unwrap();
        assert_eq!(hijri_date.month_arabic, "محرم".to_string());
        assert_eq!(hijri_date.month_english, "Moharram".to_string());
    }
    #[test]
    fn max_month() {
        let hijri_date = HijriDate::new(1442, 12, 25).unwrap();
        assert_eq!(hijri_date.month_arabic, "ذو الحجة".to_string());
        assert_eq!(hijri_date.month_english, "Delhijja".to_string());
    }
    #[test]
    fn out_of_index_month() {
        let err = HijriDate::new(1442, 13, 25).unwrap_err().to_string();
        assert_eq!(err, "No such time: 13");

        let err = HijriDate::new(1442, 0, 25).unwrap_err().to_string();
        assert_eq!(err, "No such time: 0");
    }
    #[test]
    fn min_year() {
        let hijri_date = HijriDate::new(1, 1, 1).unwrap();
        assert_eq!(hijri_date.day, 1);
        assert_eq!(hijri_date.month, 1);
        assert_eq!(hijri_date.year, 1);
        assert_eq!(hijri_date.month_arabic, "محرم".to_string());
        assert_eq!(hijri_date.month_english, "Moharram".to_string());
    }
    #[test]
    fn max_year() {
        let hijri_date = HijriDate::new(2000, 1, 1).unwrap();
        assert_eq!(hijri_date.day, 1);
        assert_eq!(hijri_date.month, 1);
        assert_eq!(hijri_date.year, 2000);
        assert_eq!(hijri_date.month_arabic, "محرم".to_string());
        assert_eq!(hijri_date.month_english, "Moharram".to_string());
    }
}
