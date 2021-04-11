use rocket_contrib::json::{Json, JsonValue};
use schemars::JsonSchema;
use serde::Deserialize;
use validator::Validate;

use crate::db::Conn;
use crate::errors::{Error, FieldValidator};
use crate::user::model::AuthUser;
use crate::vcs::service::{fetch_accessible_repositories, start_tracking_repository};

#[openapi]
#[get("/vcs/repositories")]
pub fn get_accessible_repositories(
    auth_user: AuthUser,
    conn: Conn
) -> Result<JsonValue, Error> {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let repos = rt.block_on(fetch_accessible_repositories(&conn, auth_user.user_id))?;
    Ok(json!(repos))
}

#[derive(Deserialize, Validate, JsonSchema)]
pub struct NewTrackedRepository {
    #[validate(length(min = 1))]
    pub clone_url: Option<String>,
}

#[openapi]
#[post("/vcs/repositories", format = "json", data = "<repo>")]
pub fn post_start_tracking_repository(
    auth_user: AuthUser,
    conn: Conn,
    repo: Json<NewTrackedRepository>,
) -> Result<JsonValue, Error> {
    let repo = repo.into_inner();
    let mut extractor = FieldValidator::validate(&repo);
    let clone_url = extractor.extract("clone_url", repo.clone_url);
    extractor.check()?;

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(start_tracking_repository(&conn, &clone_url, auth_user.user_id))?;
    Ok(json!(res))
}
