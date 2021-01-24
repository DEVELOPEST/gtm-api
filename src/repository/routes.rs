use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

use crate::errors::{Errors, FieldValidator};
use crate::commit::routes::NewCommitData;
use crate::group;
use crate::repository;
use crate::db::Conn;

#[derive(Deserialize)]
pub struct NewRepository {
    repository: NewRepositoryData,
}

#[derive(Deserialize, Validate)]
pub struct NewRepositoryData {
    #[validate(length(min = 1))]
    user: Option<String>,
    #[validate(length(min = 1))]
    provider: Option<String>,
    #[validate(length(min = 1))]
    repo: Option<String>,
    sync_url: Option<String>,
    access_token: Option<String>,
    #[serde(rename = "commits")]
    commits: Vec<NewCommitData>,
}

#[post("/repositories", format = "json", data = "<new_repository>")]
pub fn post_repository(
    new_repository: Json<NewRepository>,
    conn: Conn,
) -> Result<JsonValue, Errors> {
    let new_repository = new_repository.into_inner().repository;

    let mut extractor = FieldValidator::validate(&new_repository);
    let user = extractor.extract("user", new_repository.user);
    let provider = extractor.extract("provider", new_repository.provider);
    let repo = extractor.extract("repo", new_repository.repo);
    let sync_url = extractor.extract("sync_url", new_repository.sync_url);
    let access_token = extractor.extract("access_token", new_repository.access_token);
    extractor.check()?;

    let group_name = format!("{}-{}-{}", provider, user, repo);
    if !group::db::exists(&conn, &group_name) {
        group::db::create(&conn, &group_name);
    }
    let group = group::db::find(&conn, &group_name).unwrap();

    let repository = repository::db::create(
        &conn,
        &group.id,
        &user,
        &provider,
        &repo,
        &sync_url,
        &access_token,
        new_repository.commits,
    );
    Ok(json!({ "repository": repository }))
}

#[put("/repositories", format = "json", data = "<new_repository>")]
pub fn put_repository(
    // auth: AuthUser,
    new_repository: Json<NewRepository>,
    conn: Conn,
) -> Result<JsonValue, Errors> {
    let new_repository = new_repository.into_inner().repository;

    let mut extractor = FieldValidator::validate(&new_repository);
    let user = extractor.extract("user", new_repository.user);
    let provider = extractor.extract("provider", new_repository.provider);
    let repo = extractor.extract("repo", new_repository.repo);
    let sync_url = extractor.extract("sync_url", new_repository.sync_url);
    let access_token = extractor.extract("access_token", new_repository.access_token);
    extractor.check()?;

    let repository = repository::db::update(
        &conn,
        &user,
        &provider,
        &repo,
        &sync_url,
        &access_token,
        new_repository.commits,
    );
    Ok(json!({ "repository": repository }))
}

#[derive(Deserialize)]
pub struct Repository {
    repository: RepositoryData,
}

#[derive(Deserialize, Validate)]
pub struct RepositoryData {
    #[validate(length(min = 1))]
    user: Option<String>,
    #[validate(length(min = 1))]
    provider: Option<String>,
    #[validate(length(min = 1))]
    repo: Option<String>,
}
