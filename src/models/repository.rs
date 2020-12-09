use crate::config::DATE_FORMAT;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::models::commit::CommitJson;

#[derive(Queryable)]
pub struct Repository {
    pub id: i32,
    pub url: String,
    pub sync_url: String,
    pub access_token: String,
    pub timestamp: DateTime<Utc>,
}

impl Repository {
    pub fn attach(self, commits: Vec<CommitJson>) -> RepositoryJson {
        RepositoryJson {
            id: self.id,
            url: self.url,
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
    pub url: String,
    pub sync_url: String,
    pub access_token: String,
    pub timestamp: String,
    pub commits: Vec<CommitJson>
}
