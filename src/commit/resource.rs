use serde::Serialize;
use rocket_okapi::{JsonSchema};
use crate::file::resource::FileJson;

#[derive(Serialize, JsonSchema)]
pub struct LastCommitHash {
    pub hash: String,
    pub timestamp: i64,
    pub tracked_commit_hashes: Vec<String>,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommitJson {
    pub id: i32,
    pub email: String,
    pub git_user_name: String,
    pub branch: String,
    pub message: String,
    pub hash: String,
    pub time: i64,
    pub files: Vec<FileJson>
}