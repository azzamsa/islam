use chrono::NaiveDate;

use islam::salah::{Config, Location, Madhab, Method, PrayerSchedule};

fn example() -> Result<(), islam::Error> {
    // https://www.mapcoordinates.net/en
    let jakarta_city = Location::new(6.182_34_f32, 106.842_87_f32);
    let config = Config::new().with(Method::Egyptian, Madhab::Shafi);

    // custom time
    let now = NaiveDate::from_ymd_opt(2023, 8, 1)
        .unwrap()
        // fajr: 4:29:50
        // .and_hms_opt(4, 30, 00)
        // sherook: 5:46:23
        // .and_hms_opt(5, 47, 00)
        // dohr: 11:54:1
        // .and_hms_opt(11, 55, 00)
        // asr: 15:2:16
        // .and_hms_opt(15, 3, 00)
        // maghreb: 18:1:40
        // .and_hms_opt(18, 2, 00)
        // ishaa: 19:9:58
        // .and_hms_opt(19, 10, 00)
        //
        // time before fajar (late ishaa, after midnight)
        // FIXME
        // tomorrow fajr: 4:29:50
        .and_hms_opt(4, 00, 00)
        .unwrap();

    println!("Current time: {}\n", now);
    // Tested against https://www.jadwalsholat.org/
    let prayer_times = PrayerSchedule::new(jakarta_city)?
        .at(now)
        .with_config(config)
        .calculate()?;

    println!("fajr: {}", prayer_times.fajr);
    println!("sherook: {}", prayer_times.sherook);
    println!("dohr: {}", prayer_times.dohr);
    println!("asr: {}", prayer_times.asr);
    println!("maghreb: {}", prayer_times.maghreb);
    println!("ishaa: {}", prayer_times.ishaa);
    println!("fajr tomorrow: {}", prayer_times.fajr_tomorrow);

    let current_prayer = prayer_times.current();
    let (hour, minute) = prayer_times.time_remaining();
    println!("\nCurrent Prayer");
    println!(
        "{}: ({:02}:{:02} left)",
        current_prayer.name()?,
        hour,
        minute
    );

    println!("\nNext Prayer");
    let next_prayer = prayer_times.next();
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
