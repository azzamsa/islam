use jiff::{self, civil, Zoned};

pub fn now() -> jiff::Zoned {
    Zoned::now()
}

pub fn today() -> civil::Date {
    Zoned::now().date()
}
