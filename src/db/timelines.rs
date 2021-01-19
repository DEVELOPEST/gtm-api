use crate::models::timeline::{Timeline, TimelineJson};
use crate::models::interval::{IntervalJson};
use crate::models::timeline_dwh::{TimelineDWH};
use crate::schema::timeline;
use crate::errors::{FieldValidator};
use crate::routes::timelines::NewTimelineData;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable, sql_query, sql_types};
use crate::mappers::timeline::{map_timeline};



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

pub fn get_timeline(
    conn: &PgConnection,
    group_name: &str,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<IntervalJson> {
    let day_timeline: Vec<TimelineDWH> = sql_query("
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
    SELECT repositories.user, timeline.time, timeline.timestamp FROM timeline
    INNER JOIN files ON timeline.file = files.id
    INNER JOIN commits ON files.commit = commits.id
    INNER JOIN repositories ON commits.repository_id = repositories.id
    WHERE repositories.group IN (
        SELECT  q.child
        FROM    q
        UNION (
            SELECT g.id
            FROM groups g
            WHERE g.name = $1))
        AND timeline.timestamp >= $2
        AND timeline.timestamp < $3")
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)
        .expect("Error loading timeline for group");

    println!("{:?}", day_timeline);
    map_timeline(day_timeline, start, end, timezone, interval)
}
