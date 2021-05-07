use std::collections::HashMap;

use diesel::PgConnection;

use crate::domain::file::db::{fetch_file_edits, fetch_pathless_file_edits};
use crate::domain::timeline::db::fetch_timeline;
use crate::domain::timeline::mapper::{cut_path, map_activity, map_subdir_level_timeline, map_timeline};
use crate::domain::timeline::resources::{ActivityJson, IntervalJson, SubdirLevelTimelineJsonEntry, SubdirLevelTimelineJsonWrapper};
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
    let mut paths = file_edits_data.iter()
        .map(|e| cut_path(&e.path, depth))
        .filter(|p| !p.ends_with(".app"))
        .collect::<Vec<String>>();
    paths.push("other".to_string());
    let data =
        map_subdir_level_timeline(file_edits_data, depth, start, end, timezone, interval, cumulative)
            .into_iter()
            .map(|mut entry| {
                let dirs = entry.directories;
                entry.directories = dirs.iter()
                    .filter(|(_, data)| {
                        data.time > time_threshold
                            || data.lines_added + data.lines_removed > lines_threshold
                    })
                    .map(|(path, data)| (path.clone(), data.clone()))
                    .collect::<HashMap<String, SubdirLevelTimelineJsonEntry>>();
                let filtered = dirs.into_iter()
                    .map(|(_, data)| data)
                    .filter(|data| !(data.time > time_threshold
                        || data.lines_added + data.lines_removed > lines_threshold))
                    .reduce(|mut a, b| {
                        a.time += b.time;
                        a.lines_added += b.lines_added;
                        a.lines_removed += b.lines_removed;
                        a.commits += b.commits;
                        a.users = if a.users > b.users { a.users } else { b.users };  // TODO: Some better formula?
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
    paths.sort();
    paths.dedup();

    Ok(SubdirLevelTimelineJsonWrapper {
        paths,
        data,
    })
}