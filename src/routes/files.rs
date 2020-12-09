use serde::Deserialize;
use validator::Validate;
use crate::routes::timelines::NewTimelineData;

#[derive(Deserialize, Validate)]
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