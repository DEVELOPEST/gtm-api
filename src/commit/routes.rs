use serde::Deserialize;
use validator::Validate;
use rocket_contrib::json::{JsonValue};
use crate::file::routes::NewFileData;
use crate::db::Conn;
use crate::repository;
use crate::commit;

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
    let repository = repository::db::find(&conn, &user, &provider, &repo);
    let last_commit = commit::db::find_last_by_repository_id(&conn, repository.id);
    let hashes = commit::db::find_all_by_repository_id(&conn, repository.id);
    json!({
    "hash": last_commit.hash ,
    "timestamp": last_commit.time,
    "tracked_commit_hashes": hashes
    })
}

