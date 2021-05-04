use schemars::JsonSchema;
use serde::Serialize;
use crate::domain::repository::model::Repository;
use crate::config::DATE_FORMAT;
use crate::domain::commit::resource::CommitJson;


impl Repository {
    pub fn attach(self, commits: Vec<CommitJson>) -> RepositoryJson {
        RepositoryJson {
            id: self.id,
            user: self.user,
            provider: self.provider,
            repo: self.repo,
            sync_client: self.sync_client,
            timestamp: self.added_at.format(DATE_FORMAT).to_string(),
            commits
        }
    }
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryJson {
    pub id: i32,
    pub user: String,
    pub provider: String,
    pub repo: String,
    pub sync_client: Option<i32>,
    pub timestamp: String,
    pub commits: Vec<CommitJson>
}