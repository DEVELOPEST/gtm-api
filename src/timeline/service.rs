use crate::timeline::resources::{IntervalJson, ActivityJson, SubdirLevelTimelineJsonWrapper};
use crate::timeline::mapper::{map_timeline, map_activity, map_subdir_level_timeline, cut_path};
use crate::timeline::db::{fetch_timeline};
use crate::file::db::{fetch_pathless_file_edits, fetch_file_edits};
use diesel::PgConnection;
use crate::errors::Error;

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
) -> Result<Vec<ActivityJson>, Error> {
    let data = fetch_pathless_file_edits(conn, group_name, start, end)?;
    Ok(map_activity(data, timezone, interval))
}

pub fn get_subdir_level_timeline(
    conn: &PgConnection,
    group_name: &str,
    depth: i32,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Result<SubdirLevelTimelineJsonWrapper, Error> {
    let file_edits_data = fetch_file_edits(conn, group_name, start, end)?;
    let mut paths = file_edits_data.iter()
        .map(|e| cut_path(&e.path, depth))
        .filter(|p| !p.ends_with(".app"))
        .collect::<Vec<String>>();
    let data = map_subdir_level_timeline(file_edits_data, depth, start, end, timezone, interval);
    paths.sort();
    paths.dedup();

    Ok(SubdirLevelTimelineJsonWrapper {
        paths,
        data,
    })
}