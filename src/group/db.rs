use diesel;
use diesel::{Insertable, sql_query, sql_types};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::common::sql::GROUP_REPOS_QUERY;
use crate::group::dwh::GroupRepoStats;
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

pub fn fetch_group_repositories_stats(conn: &PgConnection, group_name: &str, start: i64, end: i64) -> Vec<GroupRepoStats> {
    let stats: Vec<GroupRepoStats> = sql_query(format!("
        {}
        SELECT gr.name                                    AS name,
            coalesce(sum(files.time)::bigint, 0)          AS total_time,
            coalesce(sum(files.lines_added)::bigint, 0)   AS lines_added,
            coalesce(sum(files.lines_deleted)::bigint, 0) AS lines_removed,
            coalesce(count(commits.timestamp)::bigint, 0) AS commits
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
        ORDER BY total_time DESC;", GROUP_REPOS_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)
        .expect("Error loading repo stats for group");

    stats
}
