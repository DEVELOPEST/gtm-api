use diesel::PgConnection;
use crate::models::interval::{IntervalJson, ActivityJson};
use crate::db;
use crate::mappers::timeline::{map_timeline, map_activity};

pub fn get_timeline(
    conn: &PgConnection,
    group_name: &str,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<IntervalJson> {
    let timeline = db::timelines::fetch_timeline(conn, group_name, start, end);
    map_timeline(timeline, start, end, timezone, interval)
}

pub fn get_activity_timeline(
    conn: &PgConnection,
    group_name: &str,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<ActivityJson> {
    let data = db::files::fetch_file_edits(conn, group_name, start, end);
    map_activity(data, timezone, interval)
}