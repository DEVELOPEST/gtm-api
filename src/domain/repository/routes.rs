use rocket_contrib::json::Json;
use schemars::JsonSchema;
use serde::Deserialize;
use validator::Validate;

use crate::domain::commit::routes::NewCommitData;
use crate::domain::db::Conn;
use crate::domain::repository;
use crate::domain::repository::resource::RepositoryJson;
use crate::domain::user::model::AuthUser;
use crate::errors::{Error, FieldValidator};
use crate::security::api_key::ApiKey;

#[derive(Deserialize, JsonSchema)]
pub struct NewRepository {
    repository: NewRepositoryData,
}

#[derive(Deserialize, Validate, JsonSchema)]
pub struct NewRepositoryData {
    #[validate(length(min = 1))]
    user: Option<String>,
    #[validate(length(min = 1))]
    provider: Option<String>,
    #[validate(length(min = 1))]
    repo: Option<String>,
    #[serde(rename = "commits")]
    commits: Vec<NewCommitData>,
}

#[openapi]
#[post("/repositories", format = "json", data = "<new_repository>")]
pub fn post_repository(
    conn: Conn,
    api_key: ApiKey,
    new_repository: Json<NewRepository>,
) -> Result<Json<RepositoryJson>, Error> {
    let new_repository = new_repository.into_inner().repository;

    let mut extractor = FieldValidator::validate(&new_repository);
    let user = extractor.extract("user", new_repository.user);
    let provider = extractor.extract("provider", new_repository.provider);
    let repo = extractor.extract("repo", new_repository.repo);
    extractor.check()?;

    let repository = repository::service::create_repo(
        &conn,
        &api_key,
        &user,
        &provider,
        &repo,
        new_repository.commits,
    )?;

    Ok(Json(repository))
}

#[openapi]
#[put("/repositories", format = "json", data = "<new_repository>")]
pub fn put_repository(
    api_key: ApiKey,
    new_repository: Json<NewRepository>,
    conn: Conn,
) -> Result<Json<RepositoryJson>, Error> {
    let new_repository = new_repository.into_inner().repository;

    let mut extractor = FieldValidator::validate(&new_repository);
    let user = extractor.extract("user", new_repository.user);
    let provider = extractor.extract("provider", new_repository.provider);
    let repo = extractor.extract("repo", new_repository.repo);
    extractor.check()?;

    let repository = repository::service::update_repo(
        &conn,
        &api_key,
        &user,
        &provider,
        &repo,
        new_repository.commits,
    )?;

    Ok(Json(repository))
}

#[openapi]
#[delete("/repositories/<repository_id>")]
pub fn delete_repository(
    auth_user: AuthUser,
    conn: Conn,
    repository_id: i32,
) -> Result<(), Error> {
    repository::service::delete_repo(&conn, &auth_user, repository_id)?;
    Ok(())
}

