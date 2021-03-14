use serde::Serialize;
use crate::file::model::FileJson;

#[derive(Queryable)]
pub struct Commit {
    pub id: i32,
    pub repository_id: i32,
    pub hash: String,
    pub message: String,
    pub email: String,
    pub branch: String,
    pub time: i64,
    pub git_user_name: String,
}

impl Commit {
    pub fn attach(&self, files: Vec<FileJson>) -> CommitJson {
        CommitJson {
            id: self.id,
            hash: self.hash.clone(),
            branch: self.branch.clone(),
            message: self.message.clone(),
            email: self.email.clone(),
            git_user_name: self.git_user_name.clone(),
            time: self.time,
            files
        }
    }
}

#[derive(Serialize)]
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
