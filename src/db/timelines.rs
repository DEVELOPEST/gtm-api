use diesel;
use diesel::{Insertable, sql_query, sql_types};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::db;
use crate::errors::FieldValidator;
use crate::models::timeline::{Timeline, TimelineJson};
use crate::routes::timelines::NewTimelineData;
use crate::schema::timeline;
use crate::db::dwh::timeline::TimelineDWH;

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
) -> Vec<TimelineJson> {
    let mut vec = Vec::new();
    for var in files {
        let mut extractor = FieldValidator::validate(&var);
        let timestamp = extractor.extract("timestamp", var.timestamp);
        let time = extractor.extract("time", var.time);

        let new_timeline = &NewTimeline {
            file,
            timestamp,
            time,
        };

        let timeline_json = diesel::insert_into(timeline::table)
            .values(new_timeline)
            .get_result::<Timeline>(conn)
            .expect("Error creating timeline")
            .attach();

        vec.push(timeline_json)
    }
    vec
}

pub fn fetch_timeline(conn: &PgConnection, group_name: &str, start: i64, end: i64) -> Vec<TimelineDWH> {
    let day_timeline: Vec<TimelineDWH> = sql_query(format!("
    {}
    SELECT repositories.user, timeline.time, timeline.timestamp FROM timeline
    INNER JOIN files ON timeline.file = files.id
    INNER JOIN commits ON files.commit = commits.id
    INNER JOIN repositories ON commits.repository_id = repositories.id
    WHERE repositories.group IN (
        SELECT  group_repos_query.child
        FROM    group_repos_query
        UNION (
            SELECT g.id
            FROM groups g
            WHERE g.name = $1))
        AND timeline.timestamp >= $2
        AND timeline.timestamp < $3", db::queries::GROUP_REPOS_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)
        .expect("Error loading timeline for group");
    day_timeline
}
