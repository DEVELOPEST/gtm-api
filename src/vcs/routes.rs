use rocket::request::Form;
use rocket_contrib::json::Json;
use schemars::JsonSchema;
use serde::Deserialize;
use validator::Validate;

use crate::domain::db::Conn;
use crate::errors::{Error, FieldValidator};
use crate::domain::user::model::AuthUser;
use crate::vcs::resource::{TrackedRepository, VcsRepository};
use crate::vcs::service::{fetch_accessible_repositories, start_tracking_repository};

#[derive(FromForm, Default, Deserialize, JsonSchema)]
pub struct VcsReposParams {
    name: Option<String>,
}

#[openapi]
#[get("/vcs/repositories?<params..>")]
pub fn get_accessible_repositories(
    auth_user: AuthUser,
    conn: Conn,
    params: Form<VcsReposParams>,
) -> Result<Json<Vec<VcsRepository>>, Error> {
    let params = params.into_inner();
    let repo_name = params.name.as_ref().map(|s| &**s);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let repos = rt.block_on(
        fetch_accessible_repositories(
            &conn,
            auth_user.user_id,
            repo_name,
        )
    )?;
    Ok(Json(repos))
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
) -> Result<Json<TrackedRepository>, Error> {
    let repo = repo.into_inner();
    let mut extractor = FieldValidator::validate(&repo);
    let clone_url = extractor.extract("clone_url", repo.clone_url);
    extractor.check()?;

    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(start_tracking_repository(&conn, &clone_url, auth_user.user_id))?;
    Ok(Json(res))
}
