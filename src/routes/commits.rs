use serde::Deserialize;
use validator::Validate;
use crate::routes::files::NewFileData;
use crate::db;
use rocket_contrib::json::{JsonValue};

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
    conn: db::Conn,
) -> JsonValue {
    let repository = db::repositories::find(&conn, &user, &provider, &repo);
    let last_commit = db::commits::find_last_by_repository_id(&conn, repository.id);
    json!({ "hash": last_commit.hash })
}