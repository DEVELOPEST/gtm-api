use serde::Deserialize;
use validator::Validate;
use crate::routes::files::NewFileData;

#[derive(Deserialize, Validate)]
pub struct NewCommitData {
    #[validate(length(min = 1))]
    pub author: Option<String>,
    #[validate(length(min = 1))]
    pub branch: Option<String>,
    #[validate(length(min = 1))]
    pub message: Option<String>,
    #[validate(length(min = 1))]
    pub hash: Option<String>,
    pub time: Option<i64>,
    #[serde(rename = "files")]
    pub files: Vec<NewFileData>,
}
