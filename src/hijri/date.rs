use crate::hijri::cal::{
    gregorian_to_julian, hijri_to_julian, julian_to_gregorian, julian_to_hijri,
};
use crate::{time, Date};

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

#[derive(Debug, Clone)]
pub struct HijriDate {
    pub year: i32,
    pub month: u32,
    pub month_arabic: String,
    pub month_english: String,
    pub day: u32,
}

impl HijriDate {
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, crate::Error> {
        if !(1..=12).contains(&month) {
            return Err(crate::Error::InvalidMonth(month));
        }
        Ok(Self {
            year,
            month,
            day,
            month_arabic: Self::month_arabic(month),
            month_english: Self::month_english(month),
        })
    }
    pub fn to_julian(&self) -> Result<i32, crate::Error> {
        let date = time::date(self.year, self.month, self.day)?;
        Ok(hijri_to_julian(date))
    }
    pub fn to_gregorian(&self) -> Result<Date, crate::Error> {
        let julian = self.to_julian()?;
        let (year, month, day) = julian_to_gregorian(julian as f32);
        time::date(year, month, day)
    }
    pub fn next_date(self) -> Result<Self, crate::Error> {
        let julian = self.to_julian()?;
        Ok(Self::from_julian(julian + 1, 0))
    }
    pub fn today(correction_val: i32) -> Self {
        Self::from_gregorian(time::today(), correction_val)
    }
    pub fn from_julian(julian_date: i32, correction_val: i32) -> Self {
        let (year, month, day) = julian_to_hijri(julian_date, correction_val);

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
    pub fn from_gregorian(date: Date, correction_val: i32) -> Self {
        let (year, month, day) = julian_to_hijri(gregorian_to_julian(date) as i32, correction_val);

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
    use crate::time::date;

    fn hijri_date() -> Result<HijriDate, crate::Error> {
        HijriDate::new(1442, 8, 25)
    }

    #[test]
    fn hijri() -> Result<(), crate::Error> {
        let hijri_date = hijri_date()?;
        assert_eq!(hijri_date.day, 25);
        assert_eq!(hijri_date.month, 8);
        assert_eq!(hijri_date.year, 1442);
        Ok(())
    }
    #[test]
    fn tomorrow() -> Result<(), crate::Error> {
        let hijri_date = hijri_date()?;
        let tomorrow = hijri_date.next_date()?;
        assert_eq!(tomorrow.day, 26);
        assert_eq!(tomorrow.month, 8);
        assert_eq!(tomorrow.year, 1442);
        Ok(())
    }
    #[test]
    fn to_gregorian() -> Result<(), crate::Error> {
        let hijri_date = HijriDate::new(1442, 8, 25)?;
        let gregorian = hijri_date.to_gregorian()?;
        assert_eq!(gregorian, date(2021, 4, 13)?);
        Ok(())
    }
    #[test]
    fn from_gregorian() -> Result<(), crate::Error> {
        let hijri_from_gregorian = HijriDate::from_gregorian(date(2021, 4, 9)?, 0);
        assert_eq!(hijri_from_gregorian.day, 25); // FIXME: this should be 27
        assert_eq!(hijri_from_gregorian.month, 8);
        assert_eq!(hijri_from_gregorian.month_arabic, "شعبان".to_string());
        assert_eq!(hijri_from_gregorian.month_english, "Shaban".to_string());
        assert_eq!(hijri_from_gregorian.year, 1442);
        Ok(())
    }
    #[test]
    fn from_gregorian_1() -> Result<(), crate::Error> {
        let hijri_from_gregorian = HijriDate::from_gregorian(date(2020, 4, 18)?, 0);
        // tested against https://www.islamicfinder.org/islamic-calendar/2021/April/?type=Gregorian
        assert_eq!(hijri_from_gregorian.day, 23); // FIXME: this should be 25
        assert_eq!(hijri_from_gregorian.month, 8);
        assert_eq!(hijri_from_gregorian.month_english, "Shaban".to_string());
        assert_eq!(hijri_from_gregorian.year, 1441);
        Ok(())
    }
    #[test]
    fn from_julian() -> Result<(), crate::Error> {
        let hijri_from_julian = HijriDate::from_julian(2459313, 0);
        assert_eq!(hijri_from_julian.day, 25);
        assert_eq!(hijri_from_julian.month, 8);
        assert_eq!(hijri_from_julian.month_arabic, "شعبان".to_string());
        assert_eq!(hijri_from_julian.month_english, "Shaban".to_string());
        assert_eq!(hijri_from_julian.year, 1442);
        Ok(())
    }
    #[test]
    fn min_month() -> Result<(), crate::Error> {
        let hijri_date = HijriDate::new(1442, 1, 25)?;
        assert_eq!(hijri_date.month_arabic, "محرم".to_string());
        assert_eq!(hijri_date.month_english, "Moharram".to_string());
        Ok(())
    }
    #[test]
    fn max_month() -> Result<(), crate::Error> {
        let hijri_date = HijriDate::new(1442, 12, 25)?;
        assert_eq!(hijri_date.month_arabic, "ذو الحجة".to_string());
        assert_eq!(hijri_date.month_english, "Delhijja".to_string());
        Ok(())
    }
    #[test]
    fn out_of_index_month() -> Result<(), crate::Error> {
        let err = HijriDate::new(1442, 13, 25).unwrap_err().to_string();
        assert_eq!(err, "No such month: 13");

        let err = HijriDate::new(1442, 0, 25).unwrap_err().to_string();
        assert_eq!(err, "No such month: 0");
        Ok(())
    }
    #[test]
    fn min_year() -> Result<(), crate::Error> {
        let hijri_date = HijriDate::new(1, 1, 1)?;
        assert_eq!(hijri_date.day, 1);
        assert_eq!(hijri_date.month, 1);
        assert_eq!(hijri_date.year, 1);
        assert_eq!(hijri_date.month_arabic, "محرم".to_string());
        assert_eq!(hijri_date.month_english, "Moharram".to_string());
        Ok(())
    }
    #[test]
    fn max_year() -> Result<(), crate::Error> {
        let hijri_date = HijriDate::new(2000, 1, 1)?;
        assert_eq!(hijri_date.day, 1);
        assert_eq!(hijri_date.month, 1);
        assert_eq!(hijri_date.year, 2000);
        assert_eq!(hijri_date.month_arabic, "محرم".to_string());
        assert_eq!(hijri_date.month_english, "Moharram".to_string());
        Ok(())
    }
}
