use islam::hijri::{HijriDate, HijriError};
use time::macros::date;

fn example() -> Result<(), HijriError> {
    let hijri_date = HijriDate::new(1442, 8, 25)?;
    let tomorrow = hijri_date.clone().next_date()?;
    let gregorian = hijri_date.to_gregorian()?;
    let from_gregorian = HijriDate::from_gregorian(date!(2021 - 4 - 9), 0);
    let from_julian = HijriDate::from_julian(2459313, 0);

    println!(
        "Hijri date: {}-{}-{}",
        hijri_date.year, hijri_date.month, hijri_date.day
    );
    println!(
        "Hijri date: {}-{}-{}",
        hijri_date.year, hijri_date.month_arabic, hijri_date.day
    );
    println!(
        "Hijri date: {}-{}-{}",
        hijri_date.year, hijri_date.month_english, hijri_date.day
    );
    println!(
        "Tomorrow: {}-{}-{}",
        tomorrow.year, tomorrow.month, tomorrow.day
    );
    println!(
        "To gregorian: {}-{}-{}",
        gregorian.year(),
        gregorian.month(),
        gregorian.day()
    );
    println!(
        "From gregorian: {}-{}-{}",
        from_gregorian.year, from_gregorian.month, from_gregorian.day,
    );
    println!(
        "From julian: {}-{}-{}",
        from_julian.year, from_julian.month, from_julian.day,
    );

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        eprintln!("Error: {:?}", err);
    }
}
