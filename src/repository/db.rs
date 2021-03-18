use diesel;
use diesel::{Insertable, sql_query, sql_types};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::commit;
use crate::commit::routes::NewCommitData;
use crate::repository;
use crate::repository::model::{Repository, RepositoryJson};
use crate::schema::repositories;
use crate::errors::Error;

#[derive(Insertable)]
#[table_name = "repositories"]
struct NewRepository<'a> {
    group: &'a i32,
    user: &'a str,
    provider: &'a str,
    repo: &'a str,
    sync_url: &'a str,
    access_token: &'a str,
}

pub fn update(
    conn: &PgConnection,
    user: &str,
    provider: &str,
    repo: &str,
    sync_url: &str,
    access_token: &str,
    commits: Vec<NewCommitData>,
) -> Result<RepositoryJson, Error> {
    let repository = repository::db::find(&conn, &user, &provider, &repo)?;

    let commits_vec = commit::db::create_all(
        &conn,
        commits,
        repository.id,
    )?;

    let _ = diesel::update(&repository).set((
        repositories::sync_url.eq(sync_url),
        repositories::access_token.eq(access_token)
    )).execute(conn);

    Ok(repository.attach(commits_vec))
}

pub fn create(
    conn: &PgConnection,
    group: &i32,
    user: &str,
    provider: &str,
    repo: &str,
    sync_url: &str,
    access_token: &str,
    commits: Vec<NewCommitData>,
) -> Result<RepositoryJson, Error> {
    let new_repository = &NewRepository {
        group,
        user,
        provider,
        repo,
        sync_url,
        access_token,
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

pub fn remove_repo(conn: &PgConnection, user: &str, provider: &str, repo: &str) -> Result<usize, Error>{
    let count = diesel::delete(repositories::table.filter(repositories::user.eq(user)
        .and(repositories::provider.eq(provider)
            .and(repositories::repo.eq(repo)))))
        .execute(conn)?;
    Ok(count)
}

pub fn find_all_repositories_in_group(conn: &PgConnection, name: &str) -> Result<Vec<Repository>, Error> {
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
    SELECT * FROM repositories
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