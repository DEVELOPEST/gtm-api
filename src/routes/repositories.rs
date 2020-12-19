// use crate::auth::Auth;
use crate::db;
use crate::routes::commits::NewCommitData;
use crate::errors::{Errors, FieldValidator};
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;


#[derive(Deserialize)]
pub struct NewRepository {
    repository: NewRepositoryData,
}

#[derive(Deserialize, Validate)]
pub struct NewRepositoryData {
    username: Option<String>,
    provider: Option<String>,
    repo: Option<String>,
    #[validate(length(min = 1))]
    sync_url: Option<String>,
    #[validate(length(min = 1))]
    access_token: Option<String>,
    #[serde(rename = "commits")]
    commits: Vec<NewCommitData>,
}

#[post("/repositories", format = "json", data = "<new_repository>")]
pub fn post_repository(
    //auth: Auth,
    new_repository: Json<NewRepository>,
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let new_repository = new_repository.into_inner().repository;

    let mut extractor = FieldValidator::validate(&new_repository);
    let username = extractor.extract("username", new_repository.username);
    let provider = extractor.extract("provider", new_repository.provider);
    let repo = extractor.extract("repo", new_repository.repo);
    let sync_url = extractor.extract("sync_url", new_repository.sync_url);
    let access_token = extractor.extract("access_token", new_repository.access_token);
    extractor.check()?;

    let repository = db::repositories::create(
        &conn,
        &username,
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
    //auth: Auth,
    new_repository: Json<NewRepository>,
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let new_repository = new_repository.into_inner().repository;

    let mut extractor = FieldValidator::validate(&new_repository);
    let username = extractor.extract("username", new_repository.username);
    let provider = extractor.extract("provider", new_repository.provider);
    let repo = extractor.extract("repo", new_repository.repo);
    let sync_url = extractor.extract("sync_url", new_repository.sync_url);
    let access_token = extractor.extract("access_token", new_repository.access_token);
    extractor.check()?;

    let repository = db::repositories::update(
        &conn,
        &username,
        &provider,
        &repo,
        &sync_url,
        &access_token,
        new_repository.commits,
    );
    Ok(json!({ "repository": repository }))
}