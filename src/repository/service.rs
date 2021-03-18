use crate::{group, repository};
use crate::commit::routes::NewCommitData;
use crate::db::Conn;
use crate::errors::{Error};
use crate::repository::model::RepositoryJson;
use crate::security;
use crate::security::api_key::ApiKey;

pub fn update_repo(
    conn: &Conn,
    api_key: &ApiKey,
    user: &String,
    provider: &String,
    repo: &String,
    sync_url: &String,
    access_token: &String,
    commits: Vec<NewCommitData>,
) -> Result<RepositoryJson, Error> {
    repository::db::find(conn, user, provider, repo)
        .map_err(|_| Error::BadRequest("Repository not found!".to_string()))?;

    if api_key.key != *security::config::API_KEY.read().unwrap() {
        return Err(Error::AuthorizationError("Invalid API key!".to_string()));
    }

    let repository = repository::db::update(
        &conn,
        &user,
        &provider,
        &repo,
        &sync_url,
        &access_token,
        commits,
    );

    repository
}

pub fn create_repo(
    conn: &Conn,
    api_key: &ApiKey,
    user: &String,
    provider: &String,
    repo: &String,
    sync_url: &String,
    access_token: &String,
    commits: Vec<NewCommitData>,
) -> Result<RepositoryJson, Error> {
    if api_key.key != *security::config::API_KEY.read().unwrap() {
        return Err(Error::AuthorizationError("Invalid API key!".to_string()));
    }

    let group_name = format!("{}-{}-{}", provider, user.replace("/", "-"), repo);
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
        commits,
    );

    Ok(repository)
}