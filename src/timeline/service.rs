use crate::timeline::resources::{IntervalJson, ActivityJson, SubdirLevelTimelineJson};
use crate::timeline::mapper::{map_timeline, map_activity, map_subdir_level_timeline};
use crate::timeline::db::{fetch_timeline};
use crate::file::db::{fetch_pathless_file_edits, fetch_file_edits};
use diesel::PgConnection;

pub fn get_timeline(
    conn: &PgConnection,
    group_name: &str,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<IntervalJson> {
    let timeline = fetch_timeline(conn, group_name, start, end);
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
    let data = fetch_pathless_file_edits(conn, group_name, start, end);
    map_activity(data, timezone, interval)
}

pub fn get_subdir_level_timeline(
    conn: &PgConnection,
    group_name: &str,
    depth: i32,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<SubdirLevelTimelineJson> {
    let data = fetch_file_edits(conn, group_name, start, end);
    map_subdir_level_timeline(data, depth, start, end, timezone, interval)
}