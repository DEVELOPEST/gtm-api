use rocket::http::Status;
use serde::Serialize;

use crate::{commit, repository};
use crate::db::Conn;
use crate::errors::Errors;
use crate::security::api_key::ApiKey;
use crate::security;

#[derive(Serialize)]
pub struct LastCommitHash {
    pub hash: String,
    pub timestamp: i64,
    pub tracked_commit_hashes: Vec<String>,
}

pub fn find_last_commit_hash(
    conn: &Conn,
    api_key: &ApiKey,
    user: &str,
    provider: &str,
    repo: &str,
) -> Result<LastCommitHash, Errors> {
    let repository = match repository::db::find(&conn, &user, &provider, &repo) {
        Some(r) => r,
        None => return Err(Errors::new(&[("repository_not_found", "Repository not found!")],
                                       Option::from(Status::BadRequest)))
    };

    if api_key.key != *security::config::API_KEY.read().unwrap() {
        return Err(Errors::new(&[("invalid_token", "Invalid token!")],
                               Option::from(Status::Unauthorized))
        );
    }

    let last_commit = commit::db::find_last_by_repository_id(&conn, repository.id);
    let hashes = commit::db::find_all_by_repository_id(&conn, repository.id);
    if last_commit.is_none() {
        return Ok(LastCommitHash {
            hash: "".to_string(),
            timestamp: 0,
            tracked_commit_hashes: vec![],
        });
    }
    let last_commit = last_commit.unwrap();
    Ok(LastCommitHash {
        hash: last_commit.hash,
        timestamp: last_commit.time,
        tracked_commit_hashes: hashes,
    })
}