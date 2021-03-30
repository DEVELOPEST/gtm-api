use serde::{Serialize};
use crate::common::git::RepoCredentials;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct VcsRepository {
    pub full_name: String,
    pub description: String,
    pub url: String,
    pub clone_url: String,
    pub repo_credentials: Option<RepoCredentials>,
    pub last_activity: DateTime<Utc>,
    pub size: i64,
    pub stars: i32,
    pub tracked: bool,
    pub private: bool,
}

#[derive(Serialize)]
pub struct TrackedRepository {
    pub sync_url: String,
}