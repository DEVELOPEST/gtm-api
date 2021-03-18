use diesel;
use diesel::{Insertable, sql_query, sql_types};
use diesel::prelude::*;

use crate::common::sql::GROUP_CHILDREN_QUERY;
use crate::errors::{FieldValidator, Error};
use crate::schema::timeline;
use crate::timeline::dwh::TimelineDWH;
use crate::timeline::model::{Timeline};
use crate::timeline::routes::NewTimelineData;

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
           timeline.time,
           timeline.timestamp
    FROM timeline
        INNER JOIN files ON timeline.file = files.id
        INNER JOIN commits ON files.commit = commits.id
        INNER JOIN repositories ON commits.repository_id = repositories.id
        LEFT JOIN emails ON commits.email = emails.email
        LEFT JOIN users ON emails.user = users.id
    WHERE repositories.group IN (
        SELECT  group_repos_query.child
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
