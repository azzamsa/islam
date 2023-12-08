use crate::salah::config::{Config, IshaInterval};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Method {
    /// University of Islamic Sciences, Karachi (UISK)
    /// Ministry of Religious Affairs, Tunisia
    /// France - Angle 18°
    Karachi,

    /// Muslim World League (MWL)
    /// Ministry of Religious Affairs and Awqaf, Algeria
    /// Presidency of Religious Affairs, Turkey
    MuslimWorldLeague,

    /// Egyptian General Authority of Survey (EGAS)
    Egyptian,

    /// Umm al-Qura University, Makkah (UMU)
    UmmAlQura,

    /// Islamic Society of North America (ISNA)
    /// France - Angle 15°
    NorthAmerica,

    /// French Muslims (ex-UOIF)
    French,

    /// Islamic Religious Council of Singapore (MUIS)
    /// Department of Islamic Advancements of Malaysia (JAKIM)
    // Ministry of Religious Affairs of Indonesia (KEMENAG)
    Singapore,

    /// Spiritual Administration of Muslims of Russia
    Russia,

    /// Fixed Ishaa Time Interval, 90min
    FixedInterval,
}

impl Method {
    /// Generate configs
    pub fn configs(self) -> Config {
        match self {
            Self::Karachi => Config::new().angle(18.0, 18.0).method(self),
            Self::MuslimWorldLeague => Config::new().angle(18.0, 17.0).method(self),
            Self::Egyptian => Config::new().angle(19.5, 17.5).method(self),
            Self::UmmAlQura => {
                Config::new()
                    .angle(18.5, 0.0)
                    .method(self)
                    .isha_interval(IshaInterval {
                        all_year: 90.0,
                        ramdan: 120.0,
                    })
            }
            Self::NorthAmerica => Config::new().angle(15.0, 15.0).method(self),
            Self::French => Config::new().angle(12.0, 12.0).method(self),
            Self::Singapore => Config::new().angle(20.0, 18.0).method(self),
            Self::Russia => Config::new().angle(16.0, 15.0).method(self),
            Self::FixedInterval => {
                Config::new()
                    .angle(19.5, 0.0)
                    .method(self)
                    .isha_interval(IshaInterval {
                        all_year: 90.0,
                        ramdan: 120.0,
                    })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configs_for_muslim_world_league() {
        let method = Method::MuslimWorldLeague;
        let params = method.configs();

        assert_eq!(params.method, Method::MuslimWorldLeague);
    }

    #[test]
    fn configs_for_egyptian() {
        let method = Method::Egyptian;
        let params = method.configs();

        assert_eq!(params.method, Method::Egyptian);
    }
}
