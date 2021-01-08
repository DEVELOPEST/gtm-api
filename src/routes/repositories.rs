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
    //auth: Auth,
    new_repository: Json<NewRepository>,
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let new_repository = new_repository.into_inner().repository;

    let mut extractor = FieldValidator::validate(&new_repository);
    let user = extractor.extract("user", new_repository.user);
    let provider = extractor.extract("provider", new_repository.provider);
    let repo = extractor.extract("repo", new_repository.repo);
    let sync_url = extractor.extract("sync_url", new_repository.sync_url);
    let access_token = extractor.extract("access_token", new_repository.access_token);
    extractor.check()?;

    let repository = db::repositories::create(
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

#[put("/repositories", format = "json", data = "<new_repository>")]
pub fn put_repository(
    //auth: Auth,
    new_repository: Json<NewRepository>,
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let new_repository = new_repository.into_inner().repository;

    let mut extractor = FieldValidator::validate(&new_repository);
    let user = extractor.extract("user", new_repository.user);
    let provider = extractor.extract("provider", new_repository.provider);
    let repo = extractor.extract("repo", new_repository.repo);
    let sync_url = extractor.extract("sync_url", new_repository.sync_url);
    let access_token = extractor.extract("access_token", new_repository.access_token);
    extractor.check()?;

    let repository = db::repositories::update(
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

#[post("/repositories/<git_group_name>/git_groups", format = "json", data = "<repository>")]
pub fn post_repository_to_group(
    //auth: Auth,
    git_group_name: String,
    repository: Json<Repository>,
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let repository = repository.into_inner().repository;

    let mut extractor = FieldValidator::validate(&repository);
    let user = extractor.extract("user", repository.user);
    let provider = extractor.extract("provider", repository.provider);
    let repo = extractor.extract("repo", repository.repo);
    extractor.check()?;

    if db::repositories::exists(&conn, &user, &provider, &repo) {
        let repository = db::repositories::find(&conn, &user, &provider, &repo);
        if db::git_groups::exists(&conn, &git_group_name) {
            let git_group = db::git_groups::find(&conn, &git_group_name);
        }
        let git_group = db::git_groups::create(&conn, &git_group_name);
        db::git_groups_repositories::create(&conn, repository.id, git_group.id );
    }

    Ok(json!({}))
}