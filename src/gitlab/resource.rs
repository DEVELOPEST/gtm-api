use serde::Deserialize;

use crate::common;
use crate::common::git::RepoCredentials;

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
pub struct GitlabRepo {
    pub ssh_url_to_repo: String
}

impl common::git::GitRepo for GitlabRepo {
    fn get_repo_credentials(&self) -> Option<RepoCredentials> {
        common::git::generate_credentials_from_clone_url(&self.ssh_url_to_repo)
    }
}