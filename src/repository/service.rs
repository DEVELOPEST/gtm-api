use crate::commit::routes::NewCommitData;
use crate::db::Conn;
use crate::errors::Errors;
use crate::{repository, group};
use crate::repository::model::RepositoryJson;
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
) -> Result<RepositoryJson, Errors> {
    match repository::db::find(conn, user, provider, repo){
        Some(old_repo) => {
            if api_key.key != old_repo.access_token {
                return Err(Errors::new(&[("unauthorized", "Invalid API-key!")]));
            }
        }
        None => {
            return Err(Errors::new(&[("invalid_repo", "No repository to update!")]))
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
    let old_repo = repository::db::find(conn, user, provider, repo);
    if old_repo.is_some() && api_key.key != old_repo.unwrap().access_token {
        return Err(Errors::new(&[("unauthorized", "Invalid API-key!")]));
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