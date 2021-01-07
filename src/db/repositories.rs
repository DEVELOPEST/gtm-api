use crate::models::repository::{Repository, RepositoryJson};
use crate::schema::repositories;
use crate::routes::commits::NewCommitData;
use crate::db;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};


#[derive(Insertable)]
#[table_name = "repositories"]
struct NewRepository<'a> {
    username: &'a str,
    provider: &'a str,
    repo: &'a str,
    sync_url: &'a str,
    access_token: &'a str,
}

pub fn update(
    conn: &PgConnection,
    username: &str,
    provider: &str,
    repo: &str,
    sync_url: &str,
    access_token: &str,
    commits: Vec<NewCommitData>,
) -> RepositoryJson {

    let repository = db::repositories::find(&conn, &username, &provider, &repo);

    let commits_vec = db::commits::create_all(
        &conn,
        commits,
        repository.id
    );

    // TODO: Update sync_url and access_token
    repository.attach(commits_vec)
}

pub fn create(
    conn: &PgConnection,
    username: &str,
    provider: &str,
    repo: &str,
    sync_url: &str,
    access_token: &str,
    commits: Vec<NewCommitData>,
) -> RepositoryJson {
    let new_repository = &NewRepository {
        username,
        provider,
        repo,
        sync_url,
        access_token,
    };

    if exists(conn, username, provider, repo) {
        remove_repo(conn, username, provider, repo);
    }

    let repository = diesel::insert_into(repositories::table)
        .values(new_repository)
        .get_result::<Repository>(conn)
        .expect("Error creating repository");

    let commits_vec = db::commits::create_all(
        &conn,
        commits,
        repository.id
    );
    repository.attach(commits_vec)
}

pub fn exists(conn: &PgConnection, username: &str, provider: &str, repo: &str) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(repositories::table
        .filter(repositories::username.eq(username)
            .and(repositories::provider.eq(provider)
                .and(repositories::repo.eq(repo))))))
        .get_result(conn)
        .expect("Error loading favorited")
}

pub fn find(conn: &PgConnection, username: &str, provider: &str, repo: &str) -> Repository {
    repositories::table
        .filter(repositories::username.eq(username)
            .and(repositories::provider.eq(provider)
                .and(repositories::repo.eq(repo))))
        .get_result::<Repository>(conn)
        .expect("Cannot load repository")
}

pub fn remove_repo(conn: &PgConnection, username: &str, provider: &str, repo: &str) {
    diesel::delete(repositories::table.filter(repositories::username.eq(username)
        .and(repositories::provider.eq(provider)
            .and(repositories::repo.eq(repo)))))
        .execute(conn)
        .expect("Cannot delete");
}