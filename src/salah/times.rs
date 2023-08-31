use std::f32::consts::PI;

use chrono::{Datelike, Duration, Local};

use crate::{
    hijri::{cal, HijriDate},
    salah::{config::Config, prayer::Prayer},
    time, Date, DateTime,
};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Location {
    /// geographical latitude of the given location
    latitude: f32,
    /// geographical longitude of the given location
    longitude: f32,
}

impl Location {
    pub fn new(latitude: f32, longitude: f32) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PrayerSchedule {
    location: Location,
    time: DateTime,
    custom_time: Option<DateTime>,
    config: Config,
}

impl PrayerSchedule {
    pub fn new(location: Location) -> Result<Self, crate::Error> {
        Ok(Self {
            location,
            time: time::now(),
            custom_time: None,
            // default config
            config: Config::new(),
        })
    }
    pub fn on(mut self, date: Date) -> Result<Self, crate::Error> {
        self.time = date.and_hms_opt(0, 0, 0).ok_or(crate::Error::InvalidTime)?;
        Ok(self)
    }
    pub const fn at(mut self, time: DateTime) -> Self {
        self.custom_time = Some(time);
        self
    }
    pub const fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }
    pub fn calculate(&self) -> Result<PrayerTimes, crate::Error> {
        PrayerTimes::new(self.time, self.location, self.config, self.custom_time)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PrayerTimes {
    custom_time: Option<DateTime>,
    pub time: DateTime,
    pub location: Location,
    pub config: Config,
    pub dohr: DateTime,
    pub asr: DateTime,
    pub maghreb: DateTime,
    pub ishaa: DateTime,
    pub fajr: DateTime,
    pub fajr_tomorrow: DateTime,
    pub sherook: DateTime,
    pub first_third_of_night: DateTime,
    pub midnight: DateTime,
    pub last_third_of_night: DateTime,
}

impl PrayerTimes {
    pub fn new(
        time: DateTime,
        location: Location,
        config: Config,
        custom_time: Option<DateTime>,
    ) -> Result<Self, crate::Error> {
        let time = match custom_time {
            None => time,
            Some(t) => t,
        };

        // dohr time must be calculated at first, every other time depends on it!
        let dohr_time = Self::dohr(time, location)?;
        let dohr = Self::hours_to_time(time, dohr_time, 0.0, config)?;

        let asr_time = Self::asr(time, location, config)?;
        let asr = Self::hours_to_time(time, asr_time, 0.0, config)?;

        let maghreb_time = Self::maghreb(time, location, config)?;
        let maghreb = Self::hours_to_time(time, maghreb_time, 0.0, config)?;

        let ishaa_time = Self::ishaa(time, location, config)?;
        let ishaa = Self::hours_to_time(time, ishaa_time, 0.0, config)?;

        let fajr_time = Self::fajr(time, location, config)?;
        let fajr = Self::hours_to_time(time, fajr_time, 0.0, config)?;

        let sherook_time = Self::sherook(time, location, config)?;
        let sherook = Self::hours_to_time(time, sherook_time, 0.0, config)?;

        // These must be called after ishaa, since they depends on it
        let first_third_of_night_time = Self::first_third_of_night(time, location, config)?;
        let first_third_of_night =
            Self::hours_to_time(time, first_third_of_night_time, 0.0, config)?;

        let midnight_time = Self::midnight(time, location, config)?;
        let midnight = Self::hours_to_time(time, midnight_time, 0.0, config)?;

        let last_third_of_night_time = Self::last_third_of_night(time, location, config)?;
        let last_third_of_night = Self::hours_to_time(time, last_third_of_night_time, 0.0, config)?;

        let tomorrow = time + Duration::days(1);
        let fajr_time_tomorrow = Self::fajr(tomorrow, location, config)?;
        let fajr_tomorrow = Self::hours_to_time(tomorrow, fajr_time_tomorrow, 0.0, config)?;

        Ok(Self {
            custom_time,
            time,
            location,
            config,
            dohr,
            asr,
            maghreb,
            ishaa,
            fajr,
            fajr_tomorrow,
            sherook,
            first_third_of_night,
            midnight,
            last_third_of_night,
        })
    }
    /// Get the Dohr
    fn dohr(time: DateTime, location: Location) -> Result<f32, crate::Error> {
        let longitude_difference = Self::longitude_difference(location)?;

        let julian_day = cal::gregorian_to_julian(time.date());
        let time_equation = cal::equation_of_time(julian_day);
        Ok((12.0 + longitude_difference) + (time_equation / 60.0))
    }
    /// Get the Asr time
    fn asr(time: DateTime, location: Location, config: Config) -> Result<f32, crate::Error> {
        let dohr_time = Self::dohr(time, location)?;
        let angle = Self::asr_angle(time, location, config)?;
        Ok(dohr_time + Self::time_for_angle(angle, time, location)?)
    }
    /// Get the Maghreb time
    fn maghreb(time: DateTime, location: Location, _config: Config) -> Result<f32, crate::Error> {
        let dohr_time = Self::dohr(time, location)?;

        let angle = 90.83333; // constants
        Ok(dohr_time + Self::time_for_angle(angle, time, location)?)
    }
    /// Get the Ishaa time
    fn ishaa(time: DateTime, location: Location, config: Config) -> Result<f32, crate::Error> {
        let dohr_time = Self::dohr(time, location)?;

        // checking one of `all_year` or `ramadan` is enough
        // because if set, none of them would be 0.0
        if config.isha_interval.all_year > 0.0 {
            let is_ramadan = HijriDate::from_gregorian(time.date(), 0).month == 9;
            let time_after_maghreb = if is_ramadan {
                config.isha_interval.ramdan / 60.0
            } else {
                config.isha_interval.all_year / 60.0
            };
            let angle = 90.83333; //  Constants (maghreb angle)
            Ok(time_after_maghreb + dohr_time + Self::time_for_angle(angle, time, location)?)
        } else {
            // NOTE (upstream) why still need FixedInterval comparison?
            // let angle = if config.method == Method::FixedInterval {
            //     config.ishaa_angle
            // } else {
            //     config.ishaa_angle + 90.0
            // };
            let angle = config.ishaa_angle + 90.0;
            Ok(dohr_time + Self::time_for_angle(angle, time, location)?)
        }
    }
    /// Get the Fajr time
    fn fajr(time: DateTime, location: Location, config: Config) -> Result<f32, crate::Error> {
        let dohr_time = Self::dohr(time, location)?;
        // NOTE (upstream) wrong if-else?
        // let angle = if config.method == Method::FixedInterval {
        //     config.fajr_angle + 90.0
        // } else {
        //     config.fajr_angle
        // };
        let angle = config.fajr_angle + 90.0;
        Ok(dohr_time - Self::time_for_angle(angle, time, location)?)
    }
    /// Get the Sherook time
    fn sherook(time: DateTime, location: Location, _config: Config) -> Result<f32, crate::Error> {
        let dohr_time = Self::dohr(time, location)?;

        let angle = 90.83333;
        Ok(dohr_time - Self::time_for_angle(angle, time, location)?)
    }
    /// Get the third of night
    fn first_third_of_night(
        time: DateTime,
        location: Location,
        config: Config,
    ) -> Result<f32, crate::Error> {
        let maghreb_time = Self::maghreb(time, location, config)?;
        let fajr_time = Self::fajr(time, location, config)?;
        Ok(maghreb_time + (24.0 - (maghreb_time - fajr_time)) / 3.0)
    }
    /// Midnight is the exact time between sunrise (Shorook) and sunset (Maghreb),
    /// It defines usually the end of Ishaa time
    fn midnight(time: DateTime, location: Location, config: Config) -> Result<f32, crate::Error> {
        let maghreb_time = Self::maghreb(time, location, config)?;
        let fajr_time = Self::fajr(time, location, config)?;
        Ok(maghreb_time + (24.0 - (maghreb_time - fajr_time)) / 2.0)
    }
    /// Qiyam time starts after Ishaa directly, however, the best time for Qiyam is the last third of night
    fn last_third_of_night(
        time: DateTime,
        location: Location,
        config: Config,
    ) -> Result<f32, crate::Error> {
        let maghreb_time = Self::maghreb(time, location, config)?;

        let fajr_time = Self::fajr(time, location, config)?;
        Ok(maghreb_time + (2.0 * (24.0 - (maghreb_time - fajr_time)) / 3.0))
    }
    /// Convert a decimal value (in hours) to time object
    fn hours_to_time(
        time: DateTime,
        val: f32,
        shift: f32,
        config: Config,
    ) -> Result<DateTime, crate::Error> {
        let is_summer = i32::from(config.is_summer);
        let hour = val + (shift / 3600.0);
        let minute = (hour - (hour).floor()) * 60.0;
        let second = (minute - (minute).floor()) * 60.0;
        let hour = (hour + is_summer as f32).floor() % 24.0;
        time::date(time.year(), time.month(), time.day())?
            .and_hms_opt(hour as u32, minute as u32, second as u32)
            .ok_or(crate::Error::InvalidTime)
    }
    fn longitude_difference(location: Location) -> Result<f32, crate::Error> {
        let offset_second = Local::now().offset().local_minus_utc();
        let offset_hour = Duration::seconds(offset_second.into()).num_hours();
        let middle_longitude = offset_hour as f32 * 15.0;
        Ok((middle_longitude - location.longitude) / 15.0)
    }
    /// Get the angle angle for asr (according to choosen madhab)
    fn asr_angle(time: DateTime, location: Location, config: Config) -> Result<f32, crate::Error> {
        let delta = Self::sun_declination(time)?;
        let x = cal::dsin(location.latitude).mul_add(
            cal::dsin(delta),
            cal::dcos(location.latitude) * cal::dcos(delta),
        );
        let a = (x / (-x).mul_add(x, 1.0).sqrt()).atan();
        let x = config.madhab as i32 as f32 + (1.0 / (a).tan());
        Ok(90.0 - (180.0 / PI) * 2.0_f32.mul_add((1.0_f32).atan(), (x).atan()))
    }
    /// Get Times for "Fajr, Sherook, Asr, Maghreb, ishaa"
    fn time_for_angle(angle: f32, time: DateTime, location: Location) -> Result<f32, crate::Error> {
        let delta = Self::sun_declination(time)?;
        let s = (cal::dcos(angle) - cal::dsin(location.latitude) * cal::dsin(delta))
            / (cal::dcos(location.latitude) * cal::dcos(delta));
        Ok((180.0 / PI * ((-s / (-s).mul_add(s, 1.0).sqrt()).atan() + PI / 2.0)) / 15.0)
    }
    /// Get sun declination
    fn sun_declination(time: DateTime) -> Result<f32, crate::Error> {
        let julian_day = cal::gregorian_to_julian(time.date());
        let n = julian_day - 2_451_544.5;
        let epsilon = 23.44 - (0.000_000_4 * n);
        let l = 0.985_647_4_f32.mul_add(n, 280.466);
        let g = 0.985_600_3_f32.mul_add(n, 357.528);
        let lamda = 0.02_f32.mul_add(cal::dsin(2.0 * g), 1.915_f32.mul_add(cal::dsin(g), l));
        let x = cal::dsin(epsilon) * cal::dsin(lamda);
        Ok((180.0 / (4.0 * (1.0_f32).atan())) * (x / (-x).mul_add(x, 1.0).sqrt()).atan())
    }
    /// Remaining time to next prayer
    pub fn time_remaining(&self) -> (u32, u32) {
        let mut now = match self.custom_time {
            None => time::now(),
            Some(custom) => custom,
        };

        // Special case if the next prayer time is on the following day
        if self.next() == Prayer::FajrTomorrow {
            // Current time is after midnight
            if now.time() < self.fajr_tomorrow.time() {
                now += Duration::days(1);
            }
        }

        let next_prayer_time = self.time(self.next());
        let duration = next_prayer_time - now;
        let whole: f64 = duration.num_seconds() as f64 / 60.0 / 60.0;
        let fract = whole.fract();
        let hours = whole.trunc() as u32;
        let minutes = (fract * 60.0).round() as u32;

        (hours, minutes)
    }
    /// Get next prayer
    pub fn next(&self) -> Prayer {
        match self.current() {
            Prayer::Fajr => Prayer::Sherook,
            Prayer::Sherook => Prayer::Dohr,
            Prayer::Dohr => Prayer::Asr,
            Prayer::Asr => Prayer::Maghreb,
            Prayer::Maghreb => Prayer::Ishaa,
            Prayer::Ishaa => Prayer::FajrTomorrow,
            _ => Prayer::FajrTomorrow,
        }
    }
    /// Get prayer's time
    pub fn time(&self, prayer: Prayer) -> DateTime {
        match prayer {
            Prayer::Fajr => self.fajr,
            Prayer::Sherook => self.sherook,
            Prayer::Dohr => self.dohr,
            Prayer::Asr => self.asr,
            Prayer::Maghreb => self.maghreb,
            Prayer::Ishaa => self.ishaa,
            Prayer::FajrTomorrow => self.fajr_tomorrow,
        }
    }
    /// Get current prayer
    pub fn current(&self) -> Prayer {
        let now = match self.custom_time {
            None => time::now(),
            Some(custom) => custom,
        };
        self.current_time(now).expect("Out of bounds")
    }
    /// Helper function for `current`
    fn current_time(&self, time: DateTime) -> Option<Prayer> {
        let mut current_prayer: Option<Prayer> = None;

        let ranges = vec![
            (Prayer::Fajr, self.fajr..self.sherook),
            (Prayer::Sherook, self.sherook..self.dohr),
            (Prayer::Dohr, self.dohr..self.asr),
            (Prayer::Asr, self.asr..self.maghreb),
            (Prayer::Maghreb, self.maghreb..self.ishaa),
            (Prayer::Ishaa, self.ishaa..self.fajr_tomorrow),
        ];
        for (prayer, range) in ranges {
            if range.contains(&time) {
                current_prayer = Some(prayer);
            }
        }

        // Special case for time after 00:00
        // It never get any matching prayer in the iteration above
        if current_prayer.is_none() && time < self.fajr {
            current_prayer = Some(Prayer::Ishaa)
        }

        current_prayer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::salah::{madhab::Madhab, method::Method};
    use crate::time;

    fn date() -> Result<Date, crate::Error> {
        time::date(2023, 8, 30)
    }
    /// Central jakarta
    fn city() -> Location {
        // Latitude and longitude is taken from https://www.jadwalsholat.org/
        // > Untuk Kota Jakarta Pusat 6°10' LS 106°49' BT
        Location::new(6.10, 106.49)
    }
    fn config() -> Config {
        // JadwalSholat is also using Shafi as the madhab, `20.0 deg` for fajs angle, and `18.0 deg` for Ishaa angle.
        Config::new().with(Method::Singapore, Madhab::Shafi)
    }
    fn prayer_times() -> Result<PrayerTimes, crate::Error> {
        let prayer_times = PrayerSchedule::new(city())?
            .on(date()?)?
            .with_config(config())
            .calculate()?;
        Ok(prayer_times)
    }
    fn prayer_times_at(time: (u32, u32, u32)) -> Result<PrayerTimes, crate::Error> {
        let time = date()?
            .and_hms_opt(time.0, time.1, time.2)
            .ok_or(crate::Error::InvalidTime)?;
        let prayer_times = PrayerSchedule::new(city())?
            .at(time)
            .with_config(config())
            .calculate()?;
        Ok(prayer_times)
    }
    fn expected_time(hour: u32, minute: u32, second: u32) -> Result<DateTime, crate::Error> {
        let date = date()?;
        date.and_hms_opt(hour, minute, second)
            .ok_or(crate::Error::InvalidTime)
    }
    #[test]
    /// Tested against https://www.jadwalsholat.org/
    /// and the result is pretty accurate
    fn praytimes_jakarta() -> Result<(), crate::Error> {
        let prayer_times = prayer_times()?;
        // jadwalsholat.org 🔵
        // All the jadwalsholat.org iktiyati (additional 2 minutes) is stripped
        // fajr: 4:37
        // sheerook: 5:51
        // dohr: 11:54
        // asr: 15:13
        // maghreb: 17:53
        // ishaa: 19:03

        // pyislam.py 🔷
        // Fajr      : 04:29:10
        // Sherook   : 05:47:34
        // Dohr      : 11:54:50
        // Asr       : 15:01:51
        // Maghreb   : 18:02:06
        // Ishaa     : 19:12:17

        // islam.rs 🟦
        // fajr    : 4:29:10
        // sherook : 5:47:34
        // dohr    : 11:54:50
        // asr     : 15:1:51
        // maghreb : 18:2:6
        // ishaa   : 19:12:17
        // ishaa        : 2023-08-30 19:12:17
        // fajr tomorrow: 2023-08-31 04:29:07

        assert_eq!(prayer_times.fajr, expected_time(4, 29, 10)?);
        assert_eq!(prayer_times.sherook, expected_time(5, 47, 34)?);
        assert_eq!(prayer_times.dohr, expected_time(11, 54, 50)?);
        assert_eq!(prayer_times.asr, expected_time(15, 1, 51)?);
        assert_eq!(prayer_times.maghreb, expected_time(18, 2, 6)?);
        assert_eq!(prayer_times.ishaa, expected_time(19, 12, 17)?);
        Ok(())
    }
    #[test]
    fn current_prayers() -> Result<(), crate::Error> {
        let prayer_times = prayer_times()?;

        let current_prayer_time = expected_time(4, 30, 00)?;
        assert_eq!(
            prayer_times.current_time(current_prayer_time),
            Some(Prayer::Fajr)
        );

        let current_prayer_time = expected_time(5, 48, 00)?;
        assert_eq!(
            prayer_times.current_time(current_prayer_time),
            Some(Prayer::Sherook)
        );

        let current_prayer_time = expected_time(11, 55, 00)?;
        assert_eq!(
            prayer_times.current_time(current_prayer_time),
            Some(Prayer::Dohr)
        );

        let current_prayer_time = expected_time(15, 2, 00)?;
        assert_eq!(
            prayer_times.current_time(current_prayer_time),
            Some(Prayer::Asr)
        );

        let current_prayer_time = expected_time(18, 3, 00)?;
        assert_eq!(
            prayer_times.current_time(current_prayer_time),
            Some(Prayer::Maghreb)
        );

        let current_prayer_time = expected_time(19, 13, 00)?;
        assert_eq!(
            prayer_times.current_time(current_prayer_time),
            Some(Prayer::Ishaa)
        );

        // Current prayer is ishaa (after midnight/early moring, before fajr)
        let current_prayer_time = expected_time(4, 29, 00)?;
        assert_eq!(
            prayer_times.current_time(current_prayer_time),
            Some(Prayer::Ishaa)
        );
        Ok(())
    }
    #[test]
    fn remaining_time() -> Result<(), crate::Error> {
        // Right after Fajr
        let prayer_times = prayer_times_at((4, 30, 0))?;
        assert_eq!(prayer_times.current(), Prayer::Fajr);
        assert_eq!(prayer_times.time_remaining(), (1, 18));

        // 2 minutes before Sherook
        let prayer_times = prayer_times_at((5, 46, 0))?;
        assert_eq!(prayer_times.current(), Prayer::Fajr);
        assert_eq!(prayer_times.time_remaining(), (0, 2));

        // 2 minutes before Asr
        let prayer_times = prayer_times_at((15, 0, 0))?;
        assert_eq!(prayer_times.current(), Prayer::Dohr);
        assert_eq!(prayer_times.time_remaining(), (0, 2));

        // 2 minutes before Maghreb
        let prayer_times = prayer_times_at((18, 0, 0))?;
        assert_eq!(prayer_times.current(), Prayer::Asr);
        assert_eq!(prayer_times.time_remaining(), (0, 2));

        // 2 minutes before Ishaa
        let prayer_times = prayer_times_at((19, 10, 0))?;
        assert_eq!(prayer_times.current(), Prayer::Maghreb);
        assert_eq!(prayer_times.time_remaining(), (0, 2));

        // Current prayer is ishaa (after midnight/early moring, before fajr)
        let prayer_times = prayer_times_at((4, 27, 0))?;
        assert_eq!(prayer_times.current(), Prayer::Ishaa);
        assert_eq!(prayer_times.time_remaining(), (0, 2));

        Ok(())
    }
}
