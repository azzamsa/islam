use chrono::{NaiveDate, Timelike};

use islam::salah::{Config, Location, Madhab, Method, PrayerSchedule};

fn example() -> Result<(), islam::Error> {
    // https://www.mapcoordinates.net/en
    let jakarta_city = Location::new(6.182_34_f32, 106.842_87_f32);
    let config = Config::new().with(Method::Egyptian, Madhab::Shafi);

    // custom time
    let now = NaiveDate::from_ymd_opt(2023, 8, 28)
        .unwrap()
        .and_hms_opt(1, 11, 00)
        .unwrap();

    println!("Current Time: {}\n", now);
    // Tested against https://www.jadwalsholat.org/
    let prayer_times = PrayerSchedule::new(jakarta_city)?
        .at(now)
        .with_config(config)
        .calculate()?;

    let fajr = prayer_times.fajr;
    println!("fajr: {}:{}:{}", fajr.hour(), fajr.minute(), fajr.second());

    let sherook = prayer_times.sherook;
    println!(
        "sherook: {}:{}:{}",
        sherook.hour(),
        sherook.minute(),
        sherook.second()
    );

    let dohr = prayer_times.dohr;
    println!("dohr: {}:{}:{}", dohr.hour(), dohr.minute(), dohr.second());

    let asr = prayer_times.asr;
    println!("asr: {}:{}:{}", asr.hour(), asr.minute(), asr.second());

    let maghreb = prayer_times.maghreb;
    println!(
        "maghreb: {}:{}:{}",
        maghreb.hour(),
        maghreb.minute(),
        maghreb.second()
    );

    let ishaa = prayer_times.ishaa;
    println!(
        "ishaa: {}:{}:{}",
        ishaa.hour(),
        ishaa.minute(),
        ishaa.second()
    );

    let current_prayer = prayer_times.current()?;
    let (hour, minute) = prayer_times.time_remaining()?;
    println!("\nCurrent Prayer");
    println!("{}: ({:02}:{:02})", current_prayer.name()?, hour, minute);

    println!("\nNext Prayer");
    let next_prayer = prayer_times.next()?;
    let time = prayer_times.time(next_prayer);
    let time = time.format("%H:%M").to_string();
    println!("{}: ({})", next_prayer.name()?, time);

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        eprintln!("Error: {:?}", err);
    }
}
