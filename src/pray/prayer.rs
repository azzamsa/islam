use chrono::{Datelike, Weekday};

use super::error::Error;
use crate::time::today;

// only obligatory prayer
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Prayer {
    Fajr,
    Sherook,
    Dohr,
    Asr,
    Maghreb,
    Ishaa,
}

impl Prayer {
    pub fn name(self) -> Result<String, Error> {
        let prayer_name = match self {
            Self::Fajr => "Fajr",
            Self::Sherook => "Sherook",
            Self::Dohr => {
                if today().weekday() == Weekday::Fri {
                    "Jumua"
                } else {
                    "Dohr"
                }
            }
            Self::Asr => "Asr",
            Self::Maghreb => "Maghreb",
            Self::Ishaa => "Ishaa",
        };
        Ok(prayer_name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prayer_name() -> Result<(), Error> {
        assert_eq!(Prayer::Fajr.name()?, "Fajr");
        assert_eq!(Prayer::Sherook.name()?, "Sherook");

        if today().weekday() == Weekday::Fri {
            assert_eq!(Prayer::Dohr.name()?, "Jumua");
        } else {
            assert_eq!(Prayer::Dohr.name()?, "Dohr");
        }

        assert_eq!(Prayer::Asr.name()?, "Asr");
        assert_eq!(Prayer::Maghreb.name()?, "Maghreb");
        assert_eq!(Prayer::Ishaa.name()?, "Ishaa");

        Ok(())
    }
}
