use serde::Deserialize;

use crate::common;
use crate::common::git::RepoCredentials;
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct GithubUser {
    pub login: String,
    // pub id: i64,
    pub node_id: String,
}

#[derive(Deserialize)]
pub struct GithubEmail {
    pub email: String,
    pub verified: bool,
}

#[derive(Deserialize)]
pub struct GithubRepo {
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub html_url: String,
    pub updated_at: DateTime<Utc>,
    pub ssh_url: String,
    pub size: i32,
    pub stargazers_count: i32,
}

impl common::git::GitRepo for GithubRepo {
    fn get_repo_credentials(&self) -> Option<RepoCredentials> {
        common::git::generate_credentials_from_clone_url(&self.ssh_url)
    }
}
