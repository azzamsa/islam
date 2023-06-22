use crate::salah::{madhab::Madhab, method::Method};

#[derive(Debug, Copy, Clone)]
pub struct IshaInterval {
    pub all_year: f32,
    pub ramdan: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub fajr_angle: f32,
    pub ishaa_angle: f32,
    /// fajr and ishaa method
    pub method: Method,
    /// asr madhab:
    pub madhab: Madhab,
    /// is summer time is used in the place
    pub is_summer: bool,
    /// minutes after Maghreb
    pub isha_interval: IshaInterval,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            // default
            fajr_angle: 18.0,
            ishaa_angle: 18.0,
            method: Method::MuslimWorldLeague,
            madhab: Madhab::Shafi,
            is_summer: false,
            isha_interval: IshaInterval {
                all_year: 0.0,
                ramdan: 0.0,
            },
        }
    }
    pub fn with(&self, method: Method, madhab: Madhab) -> Self {
        let mut config = method.configs();
        config.madhab = madhab;
        config
    }
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }
    /// Fajr and Ishaa angle
    pub fn angle(mut self, fajr: f32, isha: f32) -> Self {
        self.fajr_angle = fajr;
        self.ishaa_angle = isha;
        self
    }
    pub fn is_summer(mut self, is_summer: bool) -> Self {
        self.is_summer = is_summer;
        self
    }
    pub fn isha_interval(mut self, isha_interval: IshaInterval) -> Self {
        self.ishaa_angle = 0.0;
        self.isha_interval = isha_interval;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::salah::{madhab::Madhab, method::Method};

    #[test]
    fn default() {
        let config = Config::new().with(Method::Egyptian, Madhab::Shafi);

        assert_eq!(config.method, Method::Egyptian);
    }
}
