use std::collections::HashMap;

use diesel::PgConnection;

use crate::errors::Error;
use crate::file::db::{fetch_file_edits, fetch_pathless_file_edits};
use crate::repository;
use crate::timeline::db::{fetch_timeline, fetch_timeline_comparison};
use crate::timeline::mapper::{cut_path, map_activity, map_subdir_level_timeline, map_timeline, map_timeline_comparison};
use crate::timeline::resources::{ActivityJson, ComparisonJsonWrapper, IntervalJson, SubdirLevelTimelineJsonEntry, SubdirLevelTimelineJsonWrapper};

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
    time_threshold: f32,
    lines_threshold: i32,
) -> Result<SubdirLevelTimelineJsonWrapper, Error> {
    let file_edits_data = fetch_file_edits(conn, group_name, start, end)?;
    let mut paths = file_edits_data.iter()
        .map(|e| cut_path(&e.path, depth))
        .filter(|p| !p.ends_with(".app"))
        .collect::<Vec<String>>();
    let data =
        map_subdir_level_timeline(file_edits_data, depth, start, end, timezone, interval)
            .into_iter()
            .map(|mut entry| {
                entry.directories = entry.directories.into_iter()
                    .filter(|(_, data)| {
                        data.time > time_threshold as f64
                            && data.lines_added + data.lines_removed > lines_threshold as i64
                    })
                    .collect::<HashMap<String, SubdirLevelTimelineJsonEntry>>();
                entry
            })
            .collect();
    paths.sort();
    paths.dedup();

    Ok(SubdirLevelTimelineJsonWrapper {
        paths,
        data,
    })
}

pub fn get_timeline_comparison(
    conn: &PgConnection,
    group_names: &Vec<String>,
    repos: &Vec<i32>,
    branches: &Vec<String>,
    users: &Vec<String>,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Result<ComparisonJsonWrapper, Error> {
    let mut repositories: Vec<i32> = group_names.iter()
        .flat_map(|g| repository::db::find_all_repository_ids_in_group(&conn, g)
            .unwrap_or(vec![]))
        .map(|r| r.id)
        .collect();

    repositories.sort();
    repositories.dedup();

    let raw_data = fetch_timeline_comparison(&conn, &repositories, start, end);
    let data = map_timeline_comparison(
        raw_data,
        start,
        end,
        timezone,
        interval,
        repos,
        branches,
        users
    );
    Ok(data)
}