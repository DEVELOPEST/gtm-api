use chrono::{DateTime, TimeZone, FixedOffset, Offset};
use serde::Serialize;

#[derive(Debug)]
pub struct Interval<Tz: TimeZone> {
    pub start: DateTime<Tz>,
    pub end: DateTime<Tz>,
    pub time: i64,
    pub users: Vec<String>,
}

impl<Tz: TimeZone> Interval<Tz> {
    pub fn attach(&self) -> IntervalJson {
        IntervalJson {
            start: format!("{:?}{}", self.start.naive_local(), self.start.offset().fix()),
            end: format!("{:?}{}", self.end.naive_local(), self.end.offset().fix()),
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