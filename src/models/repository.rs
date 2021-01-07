use crate::config::DATE_FORMAT;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::models::commit::CommitJson;

#[derive(Queryable)]
pub struct Repository {
    pub id: i32,
    pub username: String,
    pub provider: String,
    pub repo: String,
    pub sync_url: String,
    pub access_token: String,
    pub timestamp: DateTime<Utc>,
}
 // url laheb minema ja asemele tuleb user / provider / repo
impl Repository {
    pub fn attach(self, commits: Vec<CommitJson>) -> RepositoryJson {
        RepositoryJson {
            id: self.id,
            user: self.username,
            provider: self.provider,
            repo: self.repo,
            sync_url: self.sync_url,
            access_token: self.access_token,
            timestamp: self.timestamp.format(DATE_FORMAT).to_string(),
            commits
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryJson {
    pub id: i32,
    pub user: String,
    pub provider: String,
    pub repo: String,
    pub sync_url: String,
    pub access_token: String,
    pub timestamp: String,
    pub commits: Vec<CommitJson>
}
