use rocket_contrib::json::JsonValue;
use serde::Deserialize;
use validator::Validate;

use crate::commit;
use crate::db::Conn;
use crate::file::routes::NewFileData;
use crate::errors::Errors;

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
) -> Result<JsonValue, Errors> {
    Ok(json!(commit::service::find_last_commit_hash(&conn, &provider, &user, &repo)?))
}

