use serde::Deserialize;

use crate::common;
use crate::common::git::RepoCredentials;

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
    pub ssh_url: String
}

impl common::git::GitRepo for GithubRepo {
    fn get_repo_credentials(&self) -> Option<RepoCredentials> {
        common::git::generate_credentials_from_clone_url(&self.ssh_url)
    }
}
