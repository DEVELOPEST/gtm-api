use std::collections::HashMap;

use diesel::PgConnection;

use crate::domain::file::db::{fetch_file_edits, fetch_pathless_file_edits};
use crate::domain::repository;
use crate::domain::timeline::db::{fetch_timeline, fetch_timeline_comparison};
use crate::domain::timeline::mapper::{
    map_activity, map_subdir_level_timeline, map_timeline, map_timeline_comparison,
};
use crate::domain::timeline::resources::{
    ActivityJson, ComparisonJsonWrapper, IntervalJson, SubdirLevelTimelineJson,
    SubdirLevelTimelineJsonEntry, SubdirLevelTimelineJsonWrapper,
};
use crate::errors::Error;

pub fn get_timeline(
    conn: &PgConnection,
    group_name: &str,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
    cumulative: bool,
) -> Vec<IntervalJson> {
    let timeline = fetch_timeline(conn, group_name, start, end);
    map_timeline(timeline, start, end, timezone, interval, cumulative)
}

pub fn get_activity_timeline(
    conn: &PgConnection,
    group_name: &str,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
    cumulative: bool,
) -> Result<Vec<ActivityJson>, Error> {
    let data = fetch_pathless_file_edits(conn, group_name, start, end)?;
    Ok(map_activity(data, timezone, interval, cumulative))
}

pub fn get_subdir_level_timeline(
    conn: &PgConnection,
    group_name: &str,
    depth: i32,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
    time_threshold: f64,
    lines_threshold: i64,
    cumulative: bool,
) -> Result<SubdirLevelTimelineJsonWrapper, Error> {
    let file_edits_data = fetch_file_edits(conn, group_name, start, end)?;
    let mut users: Vec<&String> = file_edits_data.iter().map(|e| &e.user).collect();
    users.sort();
    users.dedup();
    let user_count = users.len();
    let time_threshold = time_threshold * (user_count as f64).sqrt();
    let lines_threshold = lines_threshold * ((user_count as f64).sqrt() as i64);

    let data: Vec<SubdirLevelTimelineJson> = map_subdir_level_timeline(
        file_edits_data,
        depth,
        start,
        end,
        timezone,
        interval,
        cumulative,
    )
    .into_iter()
    .map(|mut entry| {
        let dirs = entry.directories;
        entry.directories = dirs
            .iter()
            .filter(|(_, data)| {
                data.time > time_threshold
                    || data.lines_added + data.lines_removed > lines_threshold
            })
            .map(|(path, data)| (path.clone(), data.clone()))
            .collect::<HashMap<String, SubdirLevelTimelineJsonEntry>>();
        let filtered = dirs
            .into_iter()
            .map(|(_, data)| data)
            .filter(|data| {
                !(data.time > time_threshold
                    || data.lines_added + data.lines_removed > lines_threshold)
            })
            .reduce(|mut a, b| {
                a.time += b.time;
                a.lines_added += b.lines_added;
                a.lines_removed += b.lines_removed;
                a.commits += b.commits;
                a.users = if a.users > b.users { a.users } else { b.users }; // TODO: Some better formula?
                a
            });

        if filtered.is_some() {
            let mut other_entry = filtered.unwrap();
            other_entry.time = (other_entry.time * 10.0).round() / 10.0;
            other_entry.path = "other".to_string();
            entry.directories.insert("other".to_string(), other_entry);
        }
        entry
    })
    .collect();
    let mut paths: Vec<String> = data
        .iter()
        .map(|e| {
            e.directories
                .iter()
                .map(|(k, _)| k.clone())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect::<Vec<String>>();
    paths.push("other".to_string());
    paths.sort();
    paths.dedup();

    Ok(SubdirLevelTimelineJsonWrapper { paths, data })
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
    let mut repositories: Vec<i32> = group_names
        .iter()
        .flat_map(|g| repository::db::find_all_repository_ids_in_group(&conn, g).unwrap_or(vec![]))
        .map(|r| r.id)
        .collect();

    repositories.sort();
    repositories.dedup();

    let raw_data = fetch_timeline_comparison(&conn, &repositories, start, end);
    let data = map_timeline_comparison(
        raw_data, start, end, timezone, interval, repos, branches, users,
    );
    Ok(data)
}
