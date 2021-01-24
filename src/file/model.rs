use serde::Serialize;
use crate::timeline::model::TimelineJson;

#[derive(Queryable)]
pub struct File {
    pub id: i32,
    pub commit: i32,
    pub path: String,
    pub status: String,
    pub time: i64,
    pub lines_added: i64,
    pub lines_deleted: i64,
}

impl File {
    pub fn attach(self, timeline: Vec<TimelineJson>) -> FileJson {
        FileJson {
            id: self.id,
            path: self.path,
            status: self.status,
            time: self.time,
            lines_added: self.lines_added,
            lines_deleted: self.lines_deleted,
            timeline
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileJson {
    pub id: i32,
    pub path: String,
    pub status: String,
    pub time: i64,
    pub lines_added: i64,
    pub lines_deleted: i64,
    pub timeline: Vec<TimelineJson>
}
