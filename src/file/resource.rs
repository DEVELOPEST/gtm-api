use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::timeline::resources::TimelineJson;
use crate::timeline::routes::NewTimelineData;

#[derive(Deserialize, Validate, JsonSchema)]
pub struct NewFileData {
    #[validate(length(min = 1))]
    pub path: Option<String>,
    #[validate(length(min = 1))]
    pub status: Option<String>,
    pub time_total: Option<i64>,
    pub added_lines: Option<i64>,
    pub deleted_lines: Option<i64>,
    #[serde(rename = "timeline")]
    pub timeline: Vec<NewTimelineData>,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FileJson {
    pub id: i32,
    pub path: String,
    pub status: String,
    pub time: i64,
    pub lines_added: i64,
    pub lines_deleted: i64,
    pub timeline: Vec<TimelineJson>,
}


