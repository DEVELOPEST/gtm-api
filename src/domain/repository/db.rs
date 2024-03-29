use diesel;
use diesel::{Insertable, sql_query, sql_types};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::domain::commit;
use crate::domain::commit::routes::NewCommitData;
use crate::errors::Error;
use crate::domain::repository;
use crate::domain::repository::model::Repository;
use crate::domain::repository::resource::RepositoryJson;
use crate::schema::repositories;

#[derive(Insertable)]
#[table_name = "repositories"]
struct NewRepository<'a> {
    group: &'a i32,
    user: &'a str,
    provider: &'a str,
    repo: &'a str,
    sync_client: i32,
}

pub fn update(
    conn: &PgConnection,
    user: &str,
    provider: &str,
    repo: &str,
    sync_client: i32,
    commits: Vec<NewCommitData>,
) -> Result<RepositoryJson, Error> {
    let repository = repository::db::find(&conn, &user, &provider, &repo)?;

    if repository.sync_client.is_none() {
        diesel::update(&repository).set(
            repositories::sync_client.eq(sync_client)
        ).execute(conn)?;
    } else {
        if repository.sync_client.unwrap() != sync_client {
            return Err(Error::AuthorizationError("Illegal repository update!"));
        }
    }

    let commits_vec = commit::db::create_all(
        &conn,
        commits,
        repository.id,
    )?;

    Ok(repository.attach(commits_vec))
}

pub fn create(
    conn: &PgConnection,
    group: &i32,
    user: &str,
    provider: &str,
    repo: &str,
    sync_client: i32,
    commits: Vec<NewCommitData>,
) -> Result<RepositoryJson, Error> {
    let new_repository = &NewRepository {
        group,
        user,
        provider,
        repo,
        sync_client
    };

    if exists(conn, user, provider, repo) {
        remove_repo(conn, user, provider, repo)?;
    }

    let repository = diesel::insert_into(repositories::table)
        .values(new_repository)
        .get_result::<Repository>(conn)
        .expect("Error creating repository");

    let commits_vec = commit::db::create_all(
        &conn,
        commits,
        repository.id
    )?;
    Ok(repository.attach(commits_vec))
}

pub fn exists(conn: &PgConnection, user: &str, provider: &str, repo: &str) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(repositories::table
        .filter(repositories::user.eq(user)
            .and(repositories::provider.eq(provider)
                .and(repositories::repo.eq(repo))))))
        .get_result(conn)
        .expect("Error loading repository")
}

pub fn find(conn: &PgConnection, user: &str, provider: &str, repo: &str) -> Result<Repository, Error> {
    repositories::table
        .filter(repositories::user.eq(user)
            .and(repositories::provider.eq(provider)
                .and(repositories::repo.eq(repo))))
        .get_result::<Repository>(conn)
        .map_err(Error::DatabaseError)
}

pub fn remove_repo(conn: &PgConnection, user: &str, provider: &str, repo: &str) -> Result<usize, Error> {
    let count = diesel::delete(repositories::table.filter(repositories::user.eq(user)
        .and(repositories::provider.eq(provider)
            .and(repositories::repo.eq(repo)))))
        .execute(conn)?;
    Ok(count)
}


pub fn delete_repo(conn: &PgConnection, repo_id: i32) -> Result<usize, Error> {
    let count = diesel::delete(
        repositories::table.filter(repositories::id.eq(repo_id)))
        .execute(conn)?;
    Ok(count)
}


pub fn find_all_repository_ids_in_group(conn: &PgConnection, name: &str) -> Result<Vec<Repository>, Error> {
    let res = sql_query("
    WITH RECURSIVE q AS
        (
        SELECT  group_group_members.child, 0 AS depth
        FROM    group_group_members
        WHERE   group_group_members.parent = (
            SELECT groups.id
            FROM groups
            WHERE groups.name = $1)
        UNION
        SELECT  m.child, q.depth + 1
        FROM    group_group_members m
        JOIN    q
        ON      m.parent = q.child
        WHERE   q.depth < 100
        )
    SELECT repositories.id FROM repositories
    WHERE repositories.group IN (
        SELECT  q.child
        FROM    q
        UNION (
            SELECT g.id
            FROM groups g
            WHERE g.name = $1))")
        .bind::<sql_types::Text, _>(name)
        .load(conn)?;
    Ok(res)
}
