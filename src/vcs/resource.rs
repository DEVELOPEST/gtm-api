use serde::{Serialize};
use schemars::JsonSchema;
use crate::common::git::RepoCredentials;
use chrono::{DateTime, Utc};

#[derive(Serialize, JsonSchema)]
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
    pub id: Option<i32>,
}

#[derive(Serialize, JsonSchema)]
pub struct TrackedRepository {
    pub sync_url: String,
}