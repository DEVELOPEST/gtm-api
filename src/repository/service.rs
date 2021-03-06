use rocket::http::Status;

use crate::{group, repository};
use crate::commit::routes::NewCommitData;
use crate::db::Conn;
use crate::errors::Errors;
use crate::repository::model::RepositoryJson;
use crate::security::api_key::ApiKey;
use crate::security;

pub fn update_repo(
    conn: &Conn,
    api_key: &ApiKey,
    user: &String,
    provider: &String,
    repo: &String,
    sync_url: &String,
    access_token: &String,
    commits: Vec<NewCommitData>,
) -> Result<RepositoryJson, Errors> {
    match repository::db::find(conn, user, provider, repo){
        Some(_) => {
            if api_key.key != *security::config::API_KEY.read().unwrap() {
                return Err(Errors::new(&[("unauthorized", "Invalid API-key!")],
                                       Option::from(Status::Unauthorized))
                );
            }
        }
        None => {
            return Err(Errors::new(&[("invalid_repo", "No repository to update!")], None));
        }
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

    Ok(repository)
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
) -> Result<RepositoryJson, Errors> {
    if api_key.key != *security::config::API_KEY.read().unwrap() {
        return Err(Errors::new(&[("unauthorized", "Invalid API-key!")],
                               Option::from(Status::Unauthorized))
        );
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