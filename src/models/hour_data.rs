use chrono::{DateTime, TimeZone};
use serde::Serialize;

#[derive(Debug)]
pub struct HourData<Tz: TimeZone> {
    pub start: DateTime<Tz>,
    pub end: DateTime<Tz>,
    pub hour: i32,
    pub time: i64,
    pub users: Vec<String>,
}

impl<Tz: TimeZone> HourData<Tz> {
    pub fn attach(self) -> HourDataJson {
        HourDataJson {
            hour: self.hour,
            time: if self.time == 0 { 0 } else { self.time / self.users.len() as i64},
            users: self.users.len() as i32,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HourDataJson {
    pub hour: i32,
    pub time: i64,
    pub users: i32,
}