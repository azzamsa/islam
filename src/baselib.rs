use crate::LocalDate;
use chrono::{Datelike};

// Trigonometric functions takes values in degree
pub fn dcos(deg: f32) -> f32 {
    deg.to_radians().cos()
}

pub fn dsin(deg: f32) -> f32 {
    deg.to_radians().sin()
}

//  Hijri date calculation methods

/// Get equation of time
pub fn equation_of_time(julian_day: f32) -> f32 {
    let n = julian_day - 2_451_544.5;
    let g = 0.985_600_3_f32.mul_add(n, 357.528);
    let c = 0.0003_f32.mul_add(
        dsin(3.0 * g),
        1.9148_f32.mul_add(dsin(g), 0.02 * dsin(2.0 * g)),
    );
    let lamda = 0.985_600_3_f32.mul_add(n, 280.47) + c;
    let r = 0.0014_f32.mul_add(
        dsin(6.0 * lamda),
        (-2.468_f32).mul_add(dsin(2.0 * lamda), 0.053 * dsin(4.0 * lamda)),
    );
    (c + r) * 4.0
}

pub fn hijri_to_julian(date: LocalDate) -> i32 {
    ((((11 * date.year() + 3) / 30) as f32).floor()
        + ((354 * date.year()) as f32).floor()
        + ((30 * date.month() as u8) as f32).floor()
        - (((date.month() as u8 - 1) / 2) as f32).floor()
        + date.day() as f32
        + 1_948_440.0
        - 385.0) as i32
}

/// The Julian Day (JD) is a continuous count of days and fractions from the beginning of the year -4712,
/// I begins at Greenwich mean noon (12h Universal Time)
pub fn gregorian_to_julian(dt: LocalDate) -> f32 {
    let (day, mut month, mut year) = (dt.day(), dt.month() as u8, dt.year());

    if month <= 2 {
        month += 12;
        year -= 1;
    }

    let a = (year as f32 / 100.0).floor() as i32;

    // In this method, the Gregorian calendar reform is taken into account, the day
    // following 04 Oct. 1582 (Julian calendar) is 15 Oct. 1582 (Gregorian calendar)
    // for more information see [3, p60]
    let b = if year > 1582 || year == 1582 && month > 10 || month == 10 && day > 15 {
        2 - a + (a / 4)
    } else {
        0
    };

    // julian day
    ((365.25 * (year + 4716) as f32).floor() as i32
        + (30.6 * (month + 1) as f32).floor() as i32
        + day as i32
        + b) as f32
        - 1524.5
}

pub fn julian_to_hijri(julian_day: i32, correction_val: i32) -> (i32, u32, u32) {
    let mut l = ((julian_day as f32 + correction_val as f32).floor() as i32 - 1_948_440) + 10632;
    let n = (((l - 1) / 10631) as f32).floor();
    l = l - (10631_f32 * n) as i32 + 354;
    let j = (((10985 - l) / 5316) as f32).floor().mul_add(
        (((50 * l) / 17719) as f32).floor(),
        ((l / 5670) as f32).floor() * (((43 * l) / 15238) as f32).floor(),
    );
    l = l
        - (((30 - j as i32) / 15) as f32).floor() as i32
            * (((17719 * j as i32) / 50) as f32).floor() as i32
        - ((j as i32 / 16) as f32).floor() as i32
            * (((15238 * j as i32) / 43) as f32).floor() as i32
        + 29;

    let month = (((24 * l) / 709) as f32).floor();
    let day = ((l - ((709_f32 * month) as i32 / 24)) as f32).floor();
    let year = ((30_f32.mul_add(n, j) as i32 - 30) as f32).floor();

    (year as i32, month as u32, day as u32)
}

pub fn julian_to_gregorian(mut julian_day: f32) -> (i32, u32, u32) {
    julian_day = (julian_day as i32 + 5) as f32;
    let z = julian_day.floor() as i32;
    let f = julian_day as i32 - z;

    let a = if z < 2_299_161 {
        z
    } else {
        let alpha = ((z as f32 - 1_867_216.25) / 36524.25).floor();
        ((z + 1) + alpha as i32) - ((alpha as i32 / 4) as f32).floor() as i32
    };

    let b = a + 1524;
    let c = ((b as f32 - 122.1) / 365.25).floor();
    let d = (365.25 * c).floor();
    let e = ((b - d as i32) as f32 / 30.6001).floor(); //  The 30.6001 SHOULD NOT BE REPLACED by 30.6

    // Calculate the day
    let day = (b - d as i32) - (30.6001 * e).floor() as i32 + f;

    // Calculate the month
    let month = e as i32 - if (e as i32) < 14 { 1 } else { 13 };

    // Calculate the year
    let year = c as i32 - if month > 2 { 4716 } else { 4715 };

    (year, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::date;

    #[test]
    fn test_dcos() {
        assert_eq!(dcos(10.0), 0.9848077);
        assert_eq!(dcos(20.0), 0.9396926);
        assert_eq!(dcos(30.0), 0.8660254);
    }
    #[test]
    fn test_dsin() {
        assert_eq!(dsin(10.0), 0.17364818);
        assert_eq!(dsin(20.0), 0.34202012);
        assert_eq!(dsin(30.0), 0.5);
    }
    #[test]
    fn test_equation_of_time() {
        let precision = 5;

        assert_eq!(equation_of_time(2_436_116.31), -11.653772);
        assert_eq!(equation_of_time(1842713.0), 12.964235);

        let equation = equation_of_time(2451545.0);
        assert_eq!(format!("{:.1$}", equation, precision), "3.53552");
    }
    #[test]
    fn test_hijri_to_julian() {
        assert_eq!(hijri_to_julian(date(1442, 8, 25)), 2459313);
        assert_eq!(hijri_to_julian(date(333, 1, 27)), 2066116);
        assert_eq!(hijri_to_julian(date(1, 1, 27)), 1948466);
    }
    #[test]
    fn test_georgian_to_julian() {
        assert_eq!(
            gregorian_to_julian(date(1957, 10, 4)),
            2436115.5 // python version: 2436116.31
        );
        assert_eq!(
            gregorian_to_julian(date(333, 1, 27)),
            1842712.5 // python version: 1842713.0
        );
        assert_eq!(
            gregorian_to_julian(date(2000, 1, 1)),
            2451544.5 // python version: 2451545.0
        );
    }
    #[test]
    fn test_julian_to_hijri() {
        assert_eq!(julian_to_hijri(2459313, 0), (1442, 8, 25));
        assert_eq!(julian_to_hijri(2066116, 0), (333, 1, 27));
        assert_eq!(julian_to_hijri(1948466, 0), (1, 1, 27));
    }
    #[test]
    fn test_julian_to_gregorian() {
        assert_eq!(julian_to_gregorian(2459313.0), (2021, 4, 13));
        assert_eq!(julian_to_gregorian(2415020.5), (1900, 1, 5));
    }
}
