# For debugging purpose

from datetime import date

from pyIslam.praytimes import (
    PrayerConf,
    Prayer,
    LIST_FAJR_ISHA_METHODS,
)
from pyIslam.hijri import HijriDate
from pyIslam.qiblah import Qiblah

latitude = 6.10
longitude = 106.49
timezone = 7
fajr_isha_method = 7  # Singapore
asr_fiqh = 1  # Shafii
pconf = PrayerConf(longitude, latitude, timezone, fajr_isha_method, asr_fiqh)

prayer_times = Prayer(pconf, date.today())
hijri = HijriDate.today()

print("Fajr      : " + str(prayer_times.fajr_time()))
print("Sherook   : " + str(prayer_times.sherook_time()))
print("Dohr      : " + str(prayer_times.dohr_time()))
print("Asr       : " + str(prayer_times.asr_time()))
print("Maghreb   : " + str(prayer_times.maghreb_time()))
print("Ishaa     : " + str(prayer_times.ishaa_time()))
print("Qiyam     : " + str(prayer_times.last_third_of_night()))

print("Qiblah direction from the north: " + Qiblah(pconf).sixty())
