use serde::Serialize;
use crate::models::file::FileJson;

#[derive(Queryable)]
pub struct Commit {
    pub id: i32,
    pub repository_id: i32,
    pub hash: String,
    pub message: String,
    pub author: String,
    pub branch: String,
    pub time: i64,
}

impl Commit {
    pub fn attach(self, files: Vec<FileJson>) -> CommitJson {
        CommitJson {
            id: self.id,
            hash: self.hash,
            branch: self.branch,
            message: self.message,
            author: self.author,
            time: self.time,
            files
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitJson {
    pub id: i32,
    pub author: String,
    pub branch: String,
    pub message: String,
    pub hash: String,
    pub time: i64,
    pub files: Vec<FileJson>
}
