use chrono::{DateTime, TimeZone};
use serde::Serialize;

#[derive(Debug)]
pub struct Interval<Tz: TimeZone> {
    pub start: DateTime<Tz>,
    pub end: DateTime<Tz>,
    pub time: i64,
    pub users: Vec<String>,
}

impl<Tz: TimeZone> Interval<Tz> {
    pub fn attach(self) -> IntervalJson {
        IntervalJson {
            start: format!("{:?}", self.start),
            end: format!("{:?}", self.end),
            time: if self.time == 0 { 0 } else { self.time / self.users.len() as i64},
            users: self.users.len() as i32,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntervalJson {
    pub start: String,
    pub end: String,
    pub time: i64,
    pub users: i32,
}