use chrono::{Datelike, Local, Weekday};

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
    pub fn name(self) -> String {
        match self {
            Self::Fajr => "Fajr".to_string(),
            Self::Sherook => "Sherook".to_string(),
            Self::Dohr => {
                if Local::now().weekday() == Weekday::Fri {
                    "Jumua".to_string()
                } else {
                    "Dohr".to_string()
                }
            }
            Self::Asr => String::from("Asr"),
            Self::Maghreb => String::from("Maghreb"),
            Self::Ishaa => String::from("Ishaa"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prayer_name() {
        assert_eq!(Prayer::Fajr.name(), "Fajr");
        assert_eq!(Prayer::Sherook.name(), "Sherook");

        if Local::now().weekday() == Weekday::Fri {
            assert_eq!(Prayer::Dohr.name(), "Jumua");
        } else {
            assert_eq!(Prayer::Dohr.name(), "Dohr");
        }

        assert_eq!(Prayer::Asr.name(), "Asr");
        assert_eq!(Prayer::Maghreb.name(), "Maghreb");
        assert_eq!(Prayer::Ishaa.name(), "Ishaa");
    }
}
