use std::f32::consts::PI;

use chrono::prelude::*;

use crate::baselib::{dcos, dsin};
use crate::pray::config::Config;
use crate::pray::prayer::Prayer;
use crate::{baselib, hijri::HijriDate};

#[derive(Debug, Copy, Clone)]
pub struct PrayerTimes {
    pub date: DateTime<Local>,
    pub location: Location,
    pub config: Config,
    pub dohr: DateTime<Local>,
    pub asr: DateTime<Local>,
    pub maghreb: DateTime<Local>,
    pub ishaa: DateTime<Local>,
    pub fajr: DateTime<Local>,
    pub sherook: DateTime<Local>,
    pub first_third_of_night: DateTime<Local>,
    pub midnight: DateTime<Local>,
    pub last_third_of_night: DateTime<Local>,
}

impl PrayerTimes {
    pub fn new(date: Date<Local>, location: Location, config: Config) -> Self {
        let date = date.and_hms(0, 0, 0);

        // dohr time must be calculated at first, every other time depends on it!
        let dohr_time = PrayerTimes::dohr(date, location);
        let dohr: DateTime<Local> = PrayerTimes::hours_to_time(date, dohr_time, 0.0, config);

        let asr_time = PrayerTimes::asr(date, location, config);
        let asr = PrayerTimes::hours_to_time(date, asr_time, 0.0, config);

        let maghreb_time = PrayerTimes::maghreb(date, location, config);
        let maghreb = PrayerTimes::hours_to_time(date, maghreb_time, 0.0, config);

        let ishaa_time = PrayerTimes::ishaa(date, location, config);
        let ishaa = PrayerTimes::hours_to_time(date, ishaa_time, 0.0, config);

        let fajr_time = PrayerTimes::fajr(date, location, config);
        let fajr = PrayerTimes::hours_to_time(date, fajr_time, 0.0, config);

        let sherook_time = PrayerTimes::sherook(date, location, config);
        let sherook = PrayerTimes::hours_to_time(date, sherook_time, 0.0, config);

        // These must be called after ishaa, since they depends on it
        let first_third_of_night_time = PrayerTimes::first_third_of_night(date, location, config);
        let first_third_of_night =
            PrayerTimes::hours_to_time(date, first_third_of_night_time, 0.0, config);
        let midnight_time = PrayerTimes::midnight(date, location, config);
        let midnight = PrayerTimes::hours_to_time(date, midnight_time, 0.0, config);

        let last_third_of_night_time = PrayerTimes::last_third_of_night(date, location, config);
        let last_third_of_night =
            PrayerTimes::hours_to_time(date, last_third_of_night_time, 0.0, config);

        Self {
            location,
            date,
            config,
            dohr,
            asr,
            maghreb,
            ishaa,
            fajr,
            sherook,
            first_third_of_night,
            midnight,
            last_third_of_night,
        }
    }
    fn longitude_difference(location: Location) -> f32 {
        let middle_longitude = location.timezone as f32 * 15.0;
        (middle_longitude - location.longitude) / 15.0
    }
    /// Get sun declination
    fn sun_declination(date: DateTime<Local>) -> f32 {
        let date_utc = Local
            .ymd(date.year(), date.month(), date.day())
            .with_timezone(&Utc);
        let julian_day = baselib::gregorian_to_julian(date_utc);

        let n = julian_day - 2_451_544.5;
        let epsilon = 23.44 - (0.000_000_4 * n);
        let l = 280.466 + 0.985_647_4 * n;
        let g = 357.528 + 0.985_600_3 * n;
        let lamda = l + 1.915 * dsin(g) + 0.02 * dsin(2.0 * g);
        let x = dsin(epsilon) * dsin(lamda);
        return (180.0 / (4.0 * (1.0_f32).atan())) as f32 * (x / (-x * x + 1.0).sqrt()).atan();
    }
    // Get the angle angle for asr (according to choosen madhab)
    fn asr_angle(date: DateTime<Local>, location: Location, config: Config) -> f32 {
        let delta = PrayerTimes::sun_declination(date);
        let x = dsin(location.latitude as f32) * dsin(delta)
            + dcos(location.latitude as f32) * dcos(delta);
        let a = (x / (-x * x + 1.0).sqrt()).atan();
        let x = config.madhab as i32 as f32 + (1.0 / (a).tan());
        return 90.0 - (180.0 / PI) * ((x as f32).atan() + 2.0 * (1.0_f32).atan());
    }
    // Get Times for "Fajr, Sherook, Asr, Maghreb, ishaa"
    fn time_for_angle(angle: f32, date: DateTime<Local>, location: Location) -> f32 {
        let delta = PrayerTimes::sun_declination(date);
        let s = (dcos(angle) - dsin(location.latitude as f32) * dsin(delta))
            / (dcos(location.latitude as f32) * dcos(delta));
        return (180.0 / PI * ((-s / (-s * s + 1.0).sqrt()).atan() + PI / 2.0)) / 15.0;
    }
    // Convert a decimal value (in hours) to time object
    fn hours_to_time(
        date: DateTime<Local>,
        val: f32,
        shift: f32,
        config: Config,
    ) -> DateTime<Local> {
        let is_summer = if config.is_summer { 1 } else { 0 };
        let hours = val + (shift / 3600.0);
        let minutes = (hours as f32 - (hours as f32).floor()) * 60.0;
        let seconds = (minutes as f32 - (minutes as f32).floor()) * 60.0;
        let hours = ((hours + is_summer as f32) as f32).floor() % 24.0;
        return Local.ymd(date.year(), date.month(), date.day()).and_hms(
            hours as u32,
            minutes.floor() as u32,
            seconds.floor() as u32,
        );
    }
    /// Get the Dohr
    fn dohr(date: DateTime<Local>, location: Location) -> f32 {
        let longitude_difference = PrayerTimes::longitude_difference(location);

        let date_utc = Local
            .ymd(date.year(), date.month(), date.day())
            .with_timezone(&Utc);
        let julian_day = baselib::gregorian_to_julian(date_utc);
        let time_equation = baselib::equation_of_time(julian_day);
        let dohr_time = (12.0 + longitude_difference) as f32 + (time_equation / 60.0);
        return dohr_time;
    }
    /// Get the Asr time
    fn asr(date: DateTime<Local>, location: Location, config: Config) -> f32 {
        let dohr_time = PrayerTimes::dohr(date, location);
        let angle = PrayerTimes::asr_angle(date, location, config);
        let time = dohr_time + PrayerTimes::time_for_angle(angle, date, location);
        return time;
    }
    /// Get the Maghreb time
    fn maghreb(date: DateTime<Local>, location: Location, _config: Config) -> f32 {
        let dohr_time = PrayerTimes::dohr(date, location);

        let angle = 90.83333; // constants
        let time = dohr_time + PrayerTimes::time_for_angle(angle, date, location);
        return time;
    }
    /// Get the Ishaa time
    fn ishaa(date: DateTime<Local>, location: Location, config: Config) -> f32 {
        let dohr_time = PrayerTimes::dohr(date, location);

        // checking one of `all_year` or `ramadan` is enough
        // because if set, none of them would be 0.0
        let time = if config.isha_interval.all_year > 0.0 {
            let date_utc = Local
                .ymd(date.year(), date.month(), date.day())
                .with_timezone(&Utc);
            let is_ramadan = HijriDate::from_gregorian(date_utc, 0).month == 9;
            let time_after_maghreb = if is_ramadan {
                config.isha_interval.ramdan / 60.0
            } else {
                config.isha_interval.all_year / 60.0
            };
            let angle = 90.83333; //  Constants (maghreb angle)
            time_after_maghreb + dohr_time + PrayerTimes::time_for_angle(angle, date, location)
        } else {
            // NOTE (upstream) why still need FixedInterval comparison?
            // let angle = if config.method == Method::FixedInterval {
            //     config.ishaa_angle
            // } else {
            //     config.ishaa_angle + 90.0
            // };

            let angle = config.ishaa_angle + 90.0;
            dohr_time + PrayerTimes::time_for_angle(angle, date, location)
        };
        return time;
    }
    /// Get the Fajr time
    fn fajr(date: DateTime<Local>, location: Location, config: Config) -> f32 {
        let dohr_time = PrayerTimes::dohr(date, location);

        // NOTE (upstream) wrong if-else?
        // let angle = if config.method == Method::FixedInterval {
        //     config.fajr_angle + 90.0
        // } else {
        //     config.fajr_angle
        // };

        let angle = config.fajr_angle + 90.0;
        let time = dohr_time - PrayerTimes::time_for_angle(angle, date, location);
        return time;
    }
    /// Get the Sherook time
    fn sherook(date: DateTime<Local>, location: Location, _config: Config) -> f32 {
        let dohr_time = PrayerTimes::dohr(date, location);

        let angle = 90.83333;
        let time = dohr_time - PrayerTimes::time_for_angle(angle, date, location);
        return time;
    }
    /// Get the third of night
    fn first_third_of_night(date: DateTime<Local>, location: Location, config: Config) -> f32 {
        let maghreb_time = PrayerTimes::maghreb(date, location, config);
        let fajr_time = PrayerTimes::fajr(date, location, config);

        let time = maghreb_time + (24.0 - (maghreb_time - fajr_time)) / 3.0;
        return time;
    }
    /// Midnight is the exact time between sunrise (Shorook) and sunset (Maghreb),
    /// It defines usually the end of Ishaa time
    fn midnight(date: DateTime<Local>, location: Location, config: Config) -> f32 {
        let maghreb_time = PrayerTimes::maghreb(date, location, config);
        let fajr_time = PrayerTimes::fajr(date, location, config);

        let time = maghreb_time + (24.0 - (maghreb_time - fajr_time)) / 2.0;
        return time;
    }
    /// Qiyam time starts after Ishaa directly, however, the best time for Qiyam is the last third of night
    fn last_third_of_night(date: DateTime<Local>, location: Location, config: Config) -> f32 {
        let maghreb_time = PrayerTimes::maghreb(date, location, config);
        let fajr_time = PrayerTimes::fajr(date, location, config);

        let time = maghreb_time + (2.0 * (24.0 - (maghreb_time - fajr_time)) / 3.0);
        return time;
    }
    pub fn current(&self) -> Prayer {
        self.current_time(Local::now()).expect("Out of bounds")
    }
    fn current_time(&self, time: DateTime<Local>) -> Option<Prayer> {
        // only obligatory prayer
        let current_prayer: Option<Prayer> = {
            if self.ishaa.signed_duration_since(time).num_seconds() <= 0 {
                Some(Prayer::Ishaa)
            } else if self.maghreb.signed_duration_since(time).num_seconds() <= 0 {
                Some(Prayer::Maghreb)
            } else if self.asr.signed_duration_since(time).num_seconds() <= 0 {
                Some(Prayer::Asr)
            } else if self.dohr.signed_duration_since(time).num_seconds() <= 0 {
                Some(Prayer::Dohr)
            } else if self.fajr.signed_duration_since(time).num_seconds() <= 0 {
                Some(Prayer::Fajr)
            } else {
                None
            }
        };
        current_prayer
    }
    pub fn next(&self) -> Prayer {
        match self.current() {
            Prayer::Fajr => Prayer::Dohr,
            Prayer::Dohr => Prayer::Asr,
            Prayer::Asr => Prayer::Maghreb,
            Prayer::Maghreb => Prayer::Ishaa,
            Prayer::Ishaa => Prayer::Fajr,
            _ => Prayer::Fajr,
        }
    }
    pub fn time(&self, prayer: Prayer) -> DateTime<Local> {
        match prayer {
            Prayer::Fajr => self.fajr,
            Prayer::Sherook => self.sherook,
            Prayer::Dohr => self.dohr,
            Prayer::Asr => self.asr,
            Prayer::Maghreb => self.maghreb,
            Prayer::Ishaa => self.ishaa,
        }
    }
    pub fn time_remaining(&self) -> (u32, u32) {
        let next_time = self.time(self.next());
        let now = Utc::now();
        let now_to_next = next_time.signed_duration_since(now).num_seconds() as f64;
        let whole: f64 = now_to_next / 60.0 / 60.0;
        let fract = whole.fract();
        let hours = whole.trunc() as u32;
        let minutes = (fract * 60.0).round() as u32;

        (hours, minutes)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Location {
    /// geographical latitude of the given location
    latitude: f32,
    /// geographical longitude of the given location
    longitude: f32,
    /// the time zone GMT(+/-timezone)
    timezone: i32,
}

impl Location {
    pub fn new(latitude: f32, longitude: f32, timezone: i32) -> Self {
        Self {
            latitude,
            longitude,
            timezone,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PrayerSchedule {
    location: Location,
    date: Date<Local>,
    config: Config,
}

impl PrayerSchedule {
    pub fn new(location: Location) -> Self {
        Self {
            location,
            date: Local::today(),
            // default config
            config: Config::new(),
        }
    }
    pub fn on(mut self, date: Date<Local>) -> Self {
        self.date = date;
        self
    }
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }
    pub fn calculate(&self) -> PrayerTimes {
        PrayerTimes::new(self.date, self.location, self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pray::madhab::Madhab;
    use crate::pray::method::Method;
    use crate::pray::prayer::Prayer;

    #[test]
    fn praytimes_jakarta() {
        // tested against https://www.jadwalsholat.org/
        // and the result is extremely accurate

        // GMT+7
        let timezone = 7;
        // https://www.mapcoordinates.net/en
        let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
        let date = Local.ymd(2021, 4, 9);
        let config = Config::new().with(Method::Singapore, Madhab::Shafi);
        let prayer_times = PrayerSchedule::new(jakarta_city)
            .on(date)
            .with_config(config)
            .calculate();

        assert_eq!(prayer_times.dohr, date.and_hms(11, 54, 14));
        assert_eq!(prayer_times.asr, date.and_hms(15, 12, 14));
        assert_eq!(prayer_times.maghreb, date.and_hms(17, 54, 14));
        assert_eq!(prayer_times.ishaa, date.and_hms(19, 03, 49));
        assert_eq!(prayer_times.fajr, date.and_hms(4, 36, 34));
        assert_eq!(prayer_times.sherook, date.and_hms(5, 54, 14));
        assert_eq!(prayer_times.first_third_of_night, date.and_hms(21, 28, 21));
        assert_eq!(prayer_times.midnight, date.and_hms(23, 15, 24));
        assert_eq!(prayer_times.last_third_of_night, date.and_hms(1, 2, 28));
    }
    #[test]
    fn praytimes_jakarta_umm_alqura() {
        let timezone = 7;
        let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
        let date = Local.ymd(2021, 4, 9);
        let config = Config::new().with(Method::UmmAlQura, Madhab::Shafi);
        let prayer_times = PrayerSchedule::new(jakarta_city)
            .on(date)
            .with_config(config)
            .calculate();

        assert_eq!(prayer_times.ishaa, date.and_hms(19, 24, 14));
        assert_eq!(prayer_times.fajr, date.and_hms(4, 42, 39));
        assert_eq!(prayer_times.first_third_of_night, date.and_hms(21, 30, 22));
        assert_eq!(prayer_times.midnight, date.and_hms(23, 18, 26));
        assert_eq!(prayer_times.last_third_of_night, date.and_hms(1, 6, 30));
    }
    #[test]
    fn praytimes_jakarta_fixed_interval() {
        let timezone = 7;
        let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
        let date = Local.ymd(2021, 4, 9);
        let config = Config::new().with(Method::FixedInterval, Madhab::Shafi);
        let prayer_times = PrayerSchedule::new(jakarta_city)
            .on(date)
            .with_config(config)
            .calculate();

        assert_eq!(prayer_times.ishaa, date.and_hms(19, 24, 14));
        assert_eq!(prayer_times.fajr, date.and_hms(4, 38, 36));
        assert_eq!(prayer_times.first_third_of_night, date.and_hms(21, 29, 01));
        assert_eq!(prayer_times.midnight, date.and_hms(23, 16, 25));
        assert_eq!(prayer_times.last_third_of_night, date.and_hms(1, 3, 49));
    }
    #[test]
    fn current_prayer_is_dohr() {
        // Dohr is: 2021-04-19T11:51:45+07:00
        let timezone = 7;
        let config = Config::new().with(Method::Singapore, Madhab::Shafi);
        let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
        let date = Local.ymd(2021, 4, 19);
        let times = PrayerTimes::new(date, jakarta_city, config);
        let current_prayer_time = date.and_hms(11, 52, 0);

        assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Dohr));
    }
    #[test]
    fn current_prayer_is_asr() {
        // Asr is: 2021-04-19T15:11:51+07:00
        let timezone = 7;
        let config = Config::new().with(Method::Singapore, Madhab::Shafi);
        let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
        let date = Local.ymd(2021, 4, 19);
        let times = PrayerTimes::new(date, jakarta_city, config);
        let current_prayer_time = date.and_hms(15, 13, 0);

        assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Asr));
    }
    #[test]
    fn current_prayer_is_maghreb() {
        // Maghreb is: 2021-04-19T17:50:12+07:00
        let timezone = 7;
        let config = Config::new().with(Method::Singapore, Madhab::Shafi);
        let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
        let date = Local.ymd(2021, 4, 19);
        let times = PrayerTimes::new(date, jakarta_city, config);
        let current_prayer_time = date.and_hms(17, 51, 0);

        assert_eq!(
            times.current_time(current_prayer_time),
            Some(Prayer::Maghreb)
        );
    }
    #[test]
    fn current_prayer_is_ishaa() {
        // Ishaa is: 2021-04-19T19:00:27+07:00
        let timezone = 7;
        let config = Config::new().with(Method::Singapore, Madhab::Shafi);
        let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
        let date = Local.ymd(2021, 4, 19);
        let times = PrayerTimes::new(date, jakarta_city, config);
        let current_prayer_time = date.and_hms(19, 01, 0);

        assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Ishaa));
    }
    #[test]
    fn current_prayer_is_fajr() {
        // Fajr is: 2021-04-19T04:34:54+07:00,
        let timezone = 7;
        let config = Config::new().with(Method::Singapore, Madhab::Shafi);
        let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
        let date = Local.ymd(2021, 4, 19);
        let times = PrayerTimes::new(date, jakarta_city, config);
        let current_prayer_time = date.and_hms(4, 35, 0);

        assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Fajr));
    }
    #[test]
    fn current_prayer_is_none() {
        // FIXME this should be None
        let timezone = 7;
        let config = Config::new().with(Method::Singapore, Madhab::Shafi);
        let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
        let date = Local.ymd(2021, 4, 19);
        let times = PrayerTimes::new(date, jakarta_city, config);
        let current_prayer_time = date.and_hms(8, 0, 0);

        assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Fajr));
    }
    #[test]
    fn next_prayer_is_dohr() {
        // FIXME this should be None
        let timezone = 7;
        let config = Config::new().with(Method::Singapore, Madhab::Shafi);
        let jakarta_city = Location::new(-6.18233995_f32, 106.84287154_f32, timezone);
        let date = Local.ymd(2021, 4, 19);
        let times = PrayerTimes::new(date, jakarta_city, config);
        let current_prayer_time = date.and_hms(8, 0, 0);

        assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Fajr));
    }
}
