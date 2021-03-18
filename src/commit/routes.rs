use rocket_contrib::json::JsonValue;
use serde::Deserialize;
use validator::Validate;

use crate::commit;
use crate::db::Conn;
use crate::file::routes::NewFileData;
use crate::errors::{Error};
use crate::security::api_key::ApiKey;

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
    conn: Conn,
    api_key: ApiKey,
    provider: String,
    user: String,
    repo: String,
) -> Result<JsonValue, Error> {
    Ok(json!(commit::service::find_last_commit_hash(&conn, &api_key, &user, &provider, &repo)?))
}

