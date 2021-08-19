use diesel;
use diesel::{Insertable, sql_query, sql_types};
use diesel::prelude::*;

use crate::common::sql::GROUP_CHILDREN_QUERY;
use crate::errors::{Error, FieldValidator};
use crate::schema::timeline;
use crate::domain::timeline::dwh::{TimelineDWH, ComparisonDWH};
use crate::domain::timeline::model::{Timeline};
use crate::domain::timeline::routes::NewTimelineData;

#[derive(Insertable)]
#[table_name = "timeline"]
struct NewTimeline<> {
    file: i32,
    timestamp: i64,
    time: i64,
}

pub fn create_all(
    conn: &PgConnection,
    files: Vec<NewTimelineData>,
    file: i32
) -> Result<Vec<Timeline>, Error> {
    let mut vec = Vec::new();
    for var in files {
        let mut extractor = FieldValidator::validate(&var);
        let timestamp = extractor.extract("timestamp", var.timestamp);
        let time = extractor.extract("time", var.time);
        extractor.check()?;

        let new_timeline = &NewTimeline {
            file,
            timestamp,
            time,
        };

        let timeline_json = diesel::insert_into(timeline::table)
            .values(new_timeline)
            .get_result::<Timeline>(conn)?;
        vec.push(timeline_json)
    }
    Ok(vec)
}

pub fn fetch_timeline(conn: &PgConnection, group_name: &str, start: i64, end: i64) -> Vec<TimelineDWH> {
    let day_timeline: Vec<TimelineDWH> = sql_query(format!("
    {}
    SELECT coalesce(users.username, commits.email) AS user,
           timeline.time                           AS time,
           timeline.timestamp                      AS timestamp,
           files.lines_added                       AS lines_added,
           files.lines_deleted                     AS lines_removed
    FROM timeline
        INNER JOIN files ON timeline.file = files.id
        INNER JOIN commits ON files.commit = commits.id
        INNER JOIN repositories ON commits.repository_id = repositories.id
        LEFT JOIN emails ON commits.email = emails.email
        LEFT JOIN users ON emails.user = users.id
    WHERE repositories.group IN (
        SELECT DISTINCT group_repos_query.child
        FROM    group_repos_query
        UNION (
            SELECT g.id
            FROM groups g
            WHERE g.name = $1))
        AND timeline.timestamp >= $2
        AND timeline.timestamp < $3", GROUP_CHILDREN_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)
        .expect("Error loading timeline for group");
    day_timeline
}

pub fn fetch_timeline_comparison(conn: &PgConnection, repos: &Vec<i32>, start: i64, end: i64) -> Vec<ComparisonDWH> {
    let data: Vec<ComparisonDWH> = sql_query("
        SELECT coalesce(users.username, commits.email)        AS user,
                repositories.id                               AS repo,
                repositories.repo                             AS repo_name,
                commits.hash                                  AS commit_hash,
                commits.branch                                AS branch,
                timeline.timestamp                            AS timestamp,
                coalesce(sum(timeline.time )::bigint, 0)      AS time,
                coalesce(sum(files.lines_added)::bigint, 0)   AS lines_added,
                coalesce(sum(files.lines_deleted)::bigint, 0) AS lines_removed
        FROM repositories
                RIGHT JOIN commits ON commits.repository_id = repositories.id
                RIGHT JOIN files ON files.commit = commits.id
                RIGHT JOIN timeline ON timeline.file = files.id
                LEFT JOIN emails ON commits.email = emails.email
                LEFT JOIN users ON emails.user = users.id
        WHERE repositories.id = ANY ($1)
        AND commits.timestamp >= $2
        AND commits.timestamp < $3
        GROUP BY files.path,
                coalesce(users.username, commits.email),
                repositories.id,
                repositories.repo,
                commits.hash,
                commits.branch,
                timeline.timestamp;")
        .bind::<sql_types::Array<sql_types::Integer>, _>(repos)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)
        .expect("Error loading timeline for group");
    data
}
