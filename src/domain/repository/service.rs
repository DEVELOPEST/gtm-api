use crate::domain::commit::routes::NewCommitData;
use crate::common::git;
use crate::domain::db::Conn;
use crate::domain::{group, repository};
use crate::domain::repository::resource::RepositoryJson;
use crate::domain::role::model::ADMIN;
use crate::domain::sync;
use crate::domain::user::model::AuthUser;
use crate::errors::Error;
use crate::errors::Error::AuthorizationError;
use crate::security::api_key::ApiKey;

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

pub fn delete_repo(conn: &Conn, auth_user: &AuthUser, repository_id: i32) -> Result<(), Error> {
    let has_access: bool = if auth_user.roles.contains(&ADMIN) {
        group::db::find_all(&conn)?.iter()
            .any(|g| g.id == repository_id)
    } else {
        group::service::get_groups_with_access(&conn, auth_user.user_id)?.iter()
            .any(|g| g.id == repository_id)
    };

    if !has_access {
        return Err(AuthorizationError("No group access!"));
    }

    repository::db::delete_repo(conn, repository_id)?;
    Ok(())
}