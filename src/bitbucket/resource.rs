use serde::Deserialize;

use crate::common;
use crate::common::git::RepoCredentials;
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct BitbucketUser {
    pub username: String,
    pub account_id: String,
}

#[derive(Deserialize)]
pub struct BitbucketEmail {
    pub email: String,
    pub is_confirmed: bool,
}

#[derive(Deserialize)]
pub struct BitbucketCloneLink {
    pub href: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct BitbucketHtmlLink {
    pub href: String,
}

#[derive(Deserialize)]
pub struct BitbucketBranch {
    pub name: String,
}

#[derive(Deserialize)]
pub struct BitbucketLinks {
    pub clone: Vec<BitbucketCloneLink>,
    pub html: BitbucketHtmlLink,
}

#[derive(Deserialize)]
pub struct BitbucketRepo {
    pub scm: String,
    pub full_name: String,
    pub links: BitbucketLinks,
    pub mainbranch: BitbucketBranch,
    pub description: String,
    pub updated_on: DateTime<Utc>,
    pub size: i64,
    pub is_private: bool,
}

impl common::git::GitRepo for BitbucketRepo {
    fn get_repo_credentials(&self) -> Option<RepoCredentials> {
        if self.scm != "git" { return None; }
        let clone_url = self.links.clone.iter()
            .find(|&c| c.name == "ssh")?;  //  || c.name == "https")?;
        common::git::generate_credentials_from_clone_url(&clone_url.href.clone())
    }
}