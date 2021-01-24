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
