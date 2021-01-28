use rocket_contrib::json::JsonValue;
use serde::Deserialize;
use validator::Validate;

use crate::commit;
use crate::commit::model::Commit;
use crate::db::Conn;
use crate::file::routes::NewFileData;
use crate::repository;

#[derive(Deserialize, Validate)]
pub struct NewCommitData {
    #[validate(length(min = 1))]
    pub author: Option<String>,
    #[validate(length(min = 1))]
    pub branch: Option<String>,
    #[validate(length(min = 1))]
    pub message: Option<String>,
    #[validate(length(min = 1))]
    pub hash: Option<String>,
    pub time: Option<i64>,
    #[serde(rename = "files")]
    pub files: Vec<NewFileData>,
}


#[get("/commits/<provider>/<user>/<repo>/hash")]
pub fn get_commit_hash(
    //auth: Auth,
    provider: String,
    user: String,
    repo: String,
    conn: Conn,
) -> JsonValue {
    let repository = repository::db::find(&conn, &user, &provider, &repo).unwrap();
    let last_commit = commit::db::find_last_by_repository_id(&conn, repository.id)
        .unwrap_or(Commit {
            id: 0,
            repository_id: 0,
            hash: "".to_string(),
            message: "".to_string(),
            author: "".to_string(),
            branch: "".to_string(),
            time: 0,
        });
    let hashes = commit::db::find_all_by_repository_id(&conn, repository.id);
    json!({
    "hash": last_commit.hash ,
    "timestamp": last_commit.time,
    "tracked_commit_hashes": hashes
    })
}

