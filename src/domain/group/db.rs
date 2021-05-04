use diesel;
use diesel::{Insertable, sql_query, sql_types};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::common::sql::GROUP_CHILDREN_QUERY;
use crate::domain::group::dwh::{GroupUserStats, GroupFileStats, GroupExportData};
use crate::domain::group::model::Group;
use crate::schema::groups;
use crate::errors::Error;

#[derive(Insertable)]
#[table_name = "groups"]
struct NewGroup<'a> {
    name: &'a str,
}

pub fn create(
    conn: &PgConnection,
    name: &str,
) -> Result<Group, Error> {
    let new_group = &NewGroup {
        name,
    };

    let group = diesel::insert_into(groups::table)
        .values(new_group)
        .get_result::<Group>(conn)?;

    Ok(group)
}


pub fn exists(conn: &PgConnection, name: &str) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(groups::table
        .filter(groups::name.eq(name))))
        .get_result(conn)
        .expect("Error finding group")
}

pub fn find(conn: &PgConnection, name: &str) -> Result<Group, Error> {
    Ok(groups::table
        .filter(groups::name.eq(name))
        .first::<Group>(conn)?)
}

pub fn find_all(conn: &PgConnection) -> Result<Vec<Group>, Error> {
    Ok(groups::table.load::<Group>(conn)?)
}

pub fn fetch_group_user_stats(
    conn: &PgConnection,
    group_name: &str,
    start: i64,
    end: i64
) -> Result<Vec<GroupUserStats>, Error> {
    let stats: Vec<GroupUserStats> = sql_query(format!("
        {}
        SELECT coalesce(users.username, commits.email)        AS name,
            coalesce(sum(files.time)::bigint, 0)              AS total_time,
            coalesce(sum(files.lines_added)::bigint, 0)       AS lines_added,
            coalesce(sum(files.lines_deleted)::bigint, 0)     AS lines_removed,
            coalesce(count(DISTINCT commits.hash)::bigint, 0) AS commits
        FROM groups gr
            LEFT JOIN repositories on gr.id = repositories.group
            LEFT JOIN commits ON commits.repository_id = repositories.id
            LEFT JOIN files ON files.commit = commits.id
            LEFT JOIN emails ON commits.email = emails.email
            LEFT JOIN users ON emails.user = users.id
        WHERE repositories.group IN (
            SELECT DISTINCT group_repos_query.child
            FROM group_repos_query
            UNION
            (
                SELECT g.id
                FROM groups g
                WHERE g.name = $1))
            AND commits.timestamp >= $2
            AND commits.timestamp < $3
        GROUP BY coalesce(users.username, commits.email)
        ORDER BY total_time DESC;", GROUP_CHILDREN_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)?;

    Ok(stats)
}

pub fn fetch_group_file_stats(conn: &PgConnection, group_name: &str, start: i64, end: i64) -> Result<Vec<GroupFileStats>, Error> {
    let stats: Vec<GroupFileStats> = sql_query(format!("
        {}
        SELECT files.path                                     AS path,
            coalesce(sum(files.time)::bigint, 0)              AS total_time,
            coalesce(sum(files.lines_added)::bigint, 0)       AS lines_added,
            coalesce(sum(files.lines_deleted)::bigint, 0)     AS lines_removed,
            coalesce(count(DISTINCT commits.hash)::bigint, 0) AS commits,
            coalesce(users.username, commits.email)           AS user
        FROM groups gr
            LEFT JOIN repositories on gr.id = repositories.group
            LEFT JOIN commits ON commits.repository_id = repositories.id
            LEFT JOIN files ON files.commit = commits.id
            LEFT JOIN emails ON commits.email = emails.email
            LEFT JOIN users ON emails.user = users.id
        WHERE repositories.group IN (
            SELECT DISTINCT group_repos_query.child
            FROM group_repos_query
            UNION
            (
                SELECT g.id
                FROM groups g
                WHERE g.name = $1))
            AND commits.timestamp >= $2
            AND commits.timestamp < $3
            AND files.path IS NOT NULL
        GROUP BY files.path, coalesce(users.username, commits.email);", GROUP_CHILDREN_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)?;

    Ok(stats)
}

pub fn fetch_group_export_data(conn: &PgConnection, group_name: &str, start: i64, end: i64) -> Result<Vec<GroupExportData>, Error> {
    let stats: Vec<GroupExportData> = sql_query(format!("
        {}
        SELECT
            coalesce(users.username, commits.email)       AS user_name,
            repositories.user                             AS user,
            repositories.provider                         AS provider,
            repositories.repo                             AS repository,
            files.path                                    AS path,
            commits.timestamp                             AS timestamp,
            commits.message                               AS message,
            coalesce(sum(files.time)::bigint, 0)          AS total_time,
            coalesce(sum(files.lines_added)::bigint, 0)   AS lines_added,
            coalesce(sum(files.lines_deleted)::bigint, 0) AS lines_removed
        FROM groups gr
            LEFT JOIN repositories on gr.id = repositories.group
            LEFT JOIN commits ON commits.repository_id = repositories.id
            LEFT JOIN files ON files.commit = commits.id
            LEFT JOIN emails ON commits.email = emails.email
            LEFT JOIN users ON emails.user = users.id
        WHERE repositories.group IN (
            SELECT DISTINCT group_repos_query.child
            FROM group_repos_query
            UNION
            (
                SELECT g.id
                FROM groups g
                WHERE g.name = $1))
            AND commits.timestamp >= $2
            AND commits.timestamp < $3
            AND files.path IS NOT NULL
        GROUP BY
            files.path,
            coalesce(users.username, commits.email),
            repositories.provider,
            repositories.repo,
            repositories.user,
            commits.timestamp,
            commits.message;", GROUP_CHILDREN_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)?;

    Ok(stats)
}


pub fn fetch_group_children(conn: &PgConnection, group_id: i32) -> Result<Vec<Group>, Error> {
    let groups: Vec<Group> = sql_query(format!("
    WITH RECURSIVE group_repos_query AS
                   (
                       SELECT  group_group_members.child, 0 AS depth
                       FROM    group_group_members
                       WHERE   group_group_members.parent = (
                           SELECT groups.id
                           FROM groups
                           WHERE groups.id = $1)
                       UNION
                       SELECT  m.child, group_repos_query.depth + 1
                       FROM    group_group_members m
                                   JOIN    group_repos_query
                                           ON      m.parent = group_repos_query.child
                       WHERE   group_repos_query.depth < 100)
    SELECT * FROM groups
    WHERE groups.id in (SELECT DISTINCT group_repos_query.child
                    FROM group_repos_query
                    UNION
                    (SELECT $1) )"))
        .bind::<sql_types::Integer, _>(group_id)
        .load(conn)?;
    Ok(groups)
}
