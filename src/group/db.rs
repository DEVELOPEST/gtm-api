use diesel;
use diesel::{Insertable, sql_query, sql_types};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::common::sql::GROUP_CHILDREN_QUERY;
use crate::group::dwh::{GroupUserStats, GroupFileStats};
use crate::group::model::Group;
use crate::schema::groups;

#[derive(Insertable)]
#[table_name = "groups"]
struct NewGroup<'a> {
    name: &'a str,
}

pub fn create(
    conn: &PgConnection,
    name: &str,
) -> Group {
    let new_group = &NewGroup {
        name,
    };

    let group = diesel::insert_into(groups::table)
        .values(new_group)
        .get_result::<Group>(conn)
        .expect("Error creating  group");

    group
}


pub fn exists(conn: &PgConnection, name: &str) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(groups::table
        .filter(groups::name.eq(name))))
        .get_result(conn)
        .expect("Error finding  group")
}

pub fn find(conn: &PgConnection, name: &str) -> Option<Group> {
    groups::table
        .filter(groups::name.eq(name))
        .first::<Group>(conn)
        .ok()
}

pub fn find_all(conn: &PgConnection) -> Vec<Group> {
    groups::table
        .load::<Group>(conn)
        .expect("Unable to load groups")
}

pub fn fetch_group_user_stats(conn: &PgConnection, group_name: &str, start: i64, end: i64) -> Vec<GroupUserStats> {
    let stats: Vec<GroupUserStats> = sql_query(format!("
        {}
        SELECT gr.name                                    AS name,
            coalesce(sum(files.time)::bigint, 0)          AS total_time,
            coalesce(sum(files.lines_added)::bigint, 0)   AS lines_added,
            coalesce(sum(files.lines_deleted)::bigint, 0) AS lines_removed,
            coalesce(count(commits.id)::bigint, 0)        AS commits
        FROM groups gr
            LEFT JOIN repositories on gr.id = repositories.group
            LEFT JOIN commits ON commits.repository_id = repositories.id
            LEFT JOIN files ON files.commit = commits.id
        WHERE repositories.group IN (
            SELECT group_repos_query.child
            FROM group_repos_query
            UNION
            (
                SELECT g.id
                FROM groups g
                WHERE g.name = $1))
            AND commits.timestamp >= $2
            AND commits.timestamp < $3
        GROUP BY gr.name
        ORDER BY total_time DESC;", GROUP_CHILDREN_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)
        .unwrap_or(vec![]);

    stats
}

pub fn fetch_group_file_stats(conn: &PgConnection, group_name: &str, start: i64, end: i64) -> Vec<GroupFileStats> {
    let stats: Vec<GroupFileStats> = sql_query(format!("
        {}
        SELECT files.path                                 AS path,
            coalesce(sum(files.time)::bigint, 0)          AS total_time,
            coalesce(sum(files.lines_added)::bigint, 0)   AS lines_added,
            coalesce(sum(files.lines_deleted)::bigint, 0) AS lines_removed,
            coalesce(count(commits.id)::bigint, 0)        AS commits,
            commits.email                                 AS user
        FROM groups gr
            LEFT JOIN repositories on gr.id = repositories.group
            LEFT JOIN commits ON commits.repository_id = repositories.id
            LEFT JOIN files ON files.commit = commits.id
        WHERE repositories.group IN (
            SELECT group_repos_query.child
            FROM group_repos_query
            UNION
            (
                SELECT g.id
                FROM groups g
                WHERE g.name = $1))
            AND commits.timestamp >= $2
            AND commits.timestamp < $3
            AND files.path IS NOT NULL
        GROUP BY files.path, commits.email;", GROUP_CHILDREN_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)
        .unwrap();

    stats
}

pub fn fetch_group_children(conn: &PgConnection, group_id: i32) -> Vec<Group> {
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
    WHERE groups.id in (SELECT group_repos_query.child
                    FROM group_repos_query
                    UNION
                    (SELECT $1) )"))
        .bind::<sql_types::Integer, _>(group_id)
        .load(conn)
        .unwrap_or(vec![]);

    println!("{:?}", groups.get(0).unwrap());
    groups
}
