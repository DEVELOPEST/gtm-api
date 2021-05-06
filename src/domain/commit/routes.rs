use rocket_contrib::json::Json;
use rocket_okapi::{JsonSchema, openapi};
use serde::Deserialize;
use validator::Validate;

use crate::domain::commit;
use crate::domain::commit::resource::LastCommitHash;
use crate::domain::db::Conn;
use crate::domain::file::resource::NewFileData;
use crate::errors::Error;
use crate::security::api_key::ApiKey;

#[derive(Deserialize, Validate, JsonSchema)]
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

#[derive(FromForm, Default, Validate, Deserialize, JsonSchema)]
pub struct CommitHashParams {
    provider: String,
    user: String,
    repo: String,
}

#[openapi]
#[get("/commits/hash?<params..>")]
pub fn get_commit_hash(
    conn: Conn,
    api_key: ApiKey,
    params: CommitHashParams,
) -> Result<Json<LastCommitHash>, Error> {
    Ok(Json(commit::service::find_last_commit_hash(
        &conn,
        &api_key,
        &params.user,
        &params.provider,
        &params.repo)?
    ))
}

