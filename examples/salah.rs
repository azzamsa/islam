use chrono::{Local, TimeZone, Timelike};
use islam::pray::Config;
use islam::pray::Madhab;
use islam::pray::Method;
use islam::pray::{Location, PrayerSchedule};

fn main() {
    // GMT+7
    let timezone = 7;
    // https://www.mapcoordinates.net/en
    let jakarta_city = Location::new(6.182_34_f32, 106.842_87_f32, timezone);
    let date = Local.ymd(2021, 4, 9);
    let config = Config::new().with(Method::Singapore, Madhab::Shafi);
    let prayer_times = PrayerSchedule::new(jakarta_city)
        .on(date)
        .with_config(config)
        .calculate();

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
}
