use islam::salah::{Config, Location, Madhab, Method, PrayerSchedule};

fn example() -> Result<(), islam::Error> {
    let central_jakarta = Location::new(6.10, 106.49);
    let config = Config::new().with(Method::Singapore, Madhab::Shafi);

    // Use `on()` for specific date
    // ```
    // let now = Local::now().date_naive();
    // .on(now)?
    // .with_config(config)
    // ```
    let prayer_times = PrayerSchedule::new(central_jakarta)
        .with_config(config)
        .calculate()?;

    let fajr = prayer_times.fajr;
    let sherook = prayer_times.sherook;
    let dohr = prayer_times.dohr;
    let asr = prayer_times.asr;
    let maghreb = prayer_times.maghreb;
    let ishaa = prayer_times.ishaa;

    println!("All Prayers");
    println!(
        "fajr    : {}:{}:{}",
        fajr.hour(),
        fajr.minute(),
        fajr.second()
    );
    println!(
        "sherook : {}:{}:{}",
        sherook.hour(),
        sherook.minute(),
        sherook.second()
    );
    println!(
        "dohr    : {}:{}:{}",
        dohr.hour(),
        dohr.minute(),
        dohr.second()
    );
    println!("asr     : {}:{}:{}", asr.hour(), asr.minute(), asr.second());
    println!(
        "maghreb : {}:{}:{}",
        maghreb.hour(),
        maghreb.minute(),
        maghreb.second()
    );
    println!(
        "ishaa   : {}:{}:{}",
        ishaa.hour(),
        ishaa.minute(),
        ishaa.second()
    );

    let current_prayer = prayer_times.current();
    let (hour, minute) = prayer_times.time_remaining();
    println!("\nCurrent Prayer");
    println!(
        "{}: ({:02}:{:02} left)",
        current_prayer.name(),
        hour,
        minute
    );

    println!("\nNext Prayer");
    let next_prayer = prayer_times.next();
    let time = prayer_times.time(next_prayer);
    let time = time.strftime("%H:%M").to_string();
    println!("{}: ({})", next_prayer.name(), time);

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        eprintln!("Error: {:?}", err);
    }
}
