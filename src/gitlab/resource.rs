use serde::Deserialize;

use crate::common;
use crate::common::git::RepoCredentials;
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct GitlabUser {
    pub id: i64,
    pub username: String
}

#[derive(Deserialize)]
pub struct GitlabEmail {
    pub email: String,
}

#[derive(Deserialize)]
pub struct GitlabStatistics {
    pub commit_count: i32,
    pub repository_size: i64,
}

#[derive(Deserialize)]
pub struct GitlabRepo {
    pub name_with_namespace: String,
    pub description: Option<String>,
    pub web_url: String,
    pub ssh_url_to_repo: String,
    pub last_activity_at: DateTime<Utc>,
    pub star_count: i32,
    pub visibility: String,
    pub statistics: GitlabStatistics,
}

impl common::git::GitRepo for GitlabRepo {
    fn get_repo_credentials(&self) -> Option<RepoCredentials> {
        common::git::generate_credentials_from_clone_url(&self.ssh_url_to_repo)
    }
}