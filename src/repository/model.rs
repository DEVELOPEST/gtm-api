use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::commit::model::CommitJson;
use crate::config::DATE_FORMAT;
use crate::schema::repositories;

#[derive(Queryable, QueryableByName, Identifiable)]
#[table_name = "repositories"]
pub struct Repository {
    pub id: i32,
    pub group: i32,
    pub user: String,
    pub provider: String,
    pub repo: String,
    pub sync_url: String,
    pub access_token: String,
    pub added_at: DateTime<Utc>,
}

impl Repository {
    pub fn attach(self, commits: Vec<CommitJson>) -> RepositoryJson {
        RepositoryJson {
            id: self.id,
            user: self.user,
            provider: self.provider,
            repo: self.repo,
            sync_url: self.sync_url,
            access_token: self.access_token,
            timestamp: self.added_at.format(DATE_FORMAT).to_string(),
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