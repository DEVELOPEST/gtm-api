use crate::domain::commit;
use crate::domain::commit::resource::LastCommitHash;
use crate::domain::db::Conn;
use crate::domain::repository;
use crate::domain::sync;
use crate::errors::Error;
use crate::security::api_key::ApiKey;

pub fn find_last_commit_hash(
    conn: &Conn,
    api_key: &ApiKey,
    user: &str,
    provider: &str,
    repo: &str,
) -> Result<LastCommitHash, Error> {
    let repository = repository::db::find(&conn, &user, &provider, &repo)?;

    let client = sync::db::find_by_api_key(conn, &api_key.key)
        .map_err(|_| Error::AuthorizationError("Unauthorized repository update!"))?;
    if repository.sync_client.is_some() && repository.sync_client.unwrap() != client.id {
        return Err(Error::AuthorizationError("Unauthorized repository update!"));
    }

    let last_commit = commit::db::find_last_by_repository_id(&conn, repository.id);
    let hashes = commit::db::find_all_by_repository_id(&conn, repository.id)?;
    if last_commit.is_err() {
        return Ok(LastCommitHash {
            hash: "".to_string(),
            timestamp: 0,
            tracked_commit_hashes: vec![],
        });
    }
    let last_commit = last_commit?;
    Ok(LastCommitHash {
        hash: last_commit.hash,
        timestamp: last_commit.time,
        tracked_commit_hashes: hashes,
    })
}