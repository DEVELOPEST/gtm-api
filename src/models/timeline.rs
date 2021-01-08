use serde::Serialize;

#[derive(Queryable)]
pub struct Timeline {
    pub id: i32,
    pub file: i32,
    pub timestamp: i64,
    pub time: i64,
}

impl Timeline {
    pub fn attach(self) -> TimelineJson {
        TimelineJson {
            id: self.id,
            timestamp: self.timestamp,
            time: self.time,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineJson {
    pub id: i32,
    pub timestamp: i64,
    pub time: i64,
}


#[derive(Queryable, Debug)]
pub struct HourData {
    pub start: i64,
    pub end: i64,
    pub hour: i32,
    pub time: i64,
    pub users: Vec<String>
}

impl HourData {
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