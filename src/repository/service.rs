use crate::{group, repository};
use crate::commit::routes::NewCommitData;
use crate::common::git;
use crate::db::Conn;
use crate::errors::{Error};
use crate::repository::resource::RepositoryJson;
use crate::security::api_key::ApiKey;
use crate::sync;

pub fn update_repo(
    conn: &Conn,
    api_key: &ApiKey,
    user: &String,
    provider: &String,
    repo: &String,
    commits: Vec<NewCommitData>,
) -> Result<RepositoryJson, Error> {
    let client = sync::db::find_by_api_key(conn, &api_key.key)
        .map_err(|_| Error::AuthorizationError("Unauthorized repository update!"))?;

    repository::db::find(conn, user, provider, repo)
        .map_err(|_| Error::BadRequest("Repository not found!"))?;

    let repository = repository::db::update(
        &conn,
        &user,
        &provider,
        &repo,
        client.id,
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
    commits: Vec<NewCommitData>,
) -> Result<RepositoryJson, Error> {
    let client = sync::db::find_by_api_key(conn, &api_key.key)
        .map_err(|_| Error::AuthorizationError("Unauthorized repository update!"))?;

    let group_name = git::generate_group_name(provider, user, repo);
    if !group::db::exists(&conn, &group_name) {
        group::db::create(&conn, &group_name)?;
    }
    let group = group::db::find(&conn, &group_name).unwrap();


    let repository = repository::db::create(
        &conn,
        &group.id,
        &user,
        &provider,
        &repo,
        client.id,
        commits,
    )?;

    Ok(repository)
}