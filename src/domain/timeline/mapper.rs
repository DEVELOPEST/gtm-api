use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use itertools::Itertools;

use crate::domain::timeline::dwh::{ComparisonDWH, FileEditDWH, PathlessFileEditDWH, TimelineDWH};
use crate::domain::timeline::helper::{generate_activity_interval, generate_intervals};
use crate::domain::timeline::resources::{
    ActivityJson, ComparisonJsonWrapper, ComparisonStatJson, ComparisonStatJsonEntry, Interval,
    IntervalJson, SubdirLevelTimeline, SubdirLevelTimelineEntry, SubdirLevelTimelineJson,
    TimelineComparisonEntry, TimelineComparisonJsonEntry
};

pub fn map_timeline(
    data: Vec<TimelineDWH>,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
    cumulative: bool,
) -> Vec<IntervalJson> {
    let tz: Tz = timezone.parse().unwrap();
    let start_tz: DateTime<Tz> = get_datetime_tz_from_seconds(start, &tz);
    let end_tz = get_datetime_tz_from_seconds(end, &tz);
    let mut intervals = generate_intervals(start_tz, end_tz, interval, |s, e| Interval {
        start: s,
        end: e,
        time: 0,
        users: vec![],
    });
    for item in data {
        for i in 0..intervals.len() {
            if (intervals[i].start.timestamp() <= item.timestamp || cumulative)
                && item.timestamp < intervals[i].end.timestamp()
            {
                intervals[i].time += item.time;
                if !intervals[i].users.contains(&item.user) {
                    intervals[i].users.push(item.user.to_string());
                }
                if !cumulative {
                    break;
                }
            }
        }
    }

    intervals.into_iter().map(|x| x.attach()).collect()
}

pub fn map_activity(
    data: Vec<PathlessFileEditDWH>,
    timezone: &str,
    interval: &str,
    cumulative: bool,
) -> Vec<ActivityJson> {
    let tz: Tz = timezone.parse().unwrap();
    let interval = &*interval.to_lowercase();
    let mut intervals = generate_activity_interval(interval);

    for item in data {
        let time_point = get_datetime_tz_from_seconds(item.timestamp, &tz);
        for activity in intervals.iter_mut() {
            let id = match interval {
                "day" => time_point.hour() as i32,
                "week" => time_point.weekday().number_from_monday() as i32,
                "month" => time_point.day0() as i32,
                "year" => time_point.month0() as i32,
                _ => 0,
            };
            if !cumulative && activity.id == id || cumulative && activity.id >= id {
                activity.time += item.time;
                activity.lines_added += item.lines_added;
                activity.lines_removed += item.lines_deleted;
                activity.users.insert(item.user.clone());
            }
        }
    }
    intervals.into_iter().map(|x| x.attach()).collect()
}

pub fn map_subdir_level_timeline(
    data: Vec<FileEditDWH>,
    depth: i32,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
    cumulative: bool,
) -> Vec<SubdirLevelTimelineJson> {
    let tz: Tz = timezone.parse().unwrap();
    let start_tz: DateTime<Tz> = get_datetime_tz_from_seconds(start, &tz);
    let end_tz = get_datetime_tz_from_seconds(end, &tz);
    let mut intervals =
        generate_intervals(start_tz, end_tz, interval, |s, e| SubdirLevelTimeline {
            start: s,
            end: e,
            directories: HashMap::new(),
        });
    for item in data {
        if item.path.ends_with(".app") {
            continue;
        }
        for i in 0..intervals.len() {
            if (intervals[i].start.timestamp() <= item.timestamp || cumulative)
                && item.timestamp < intervals[i].end.timestamp()
            {
                let cut_path = cut_path(&item.path, depth);
                let entry = intervals[i].directories.get_mut(&cut_path);
                if entry.is_some() {
                    let entry = entry.unwrap();
                    entry.time += item.time;
                    entry.lines_added += item.lines_added;
                    entry.lines_removed += item.lines_deleted;
                    entry.commits.insert(item.commit_hash.clone());
                    entry.users.insert(item.user.to_string());
                } else {
                    intervals[i].directories.insert(
                        cut_path.clone(),
                        SubdirLevelTimelineEntry {
                            path: cut_path,
                            time: item.time,
                            commits: HashSet::from_iter(
                                std::iter::repeat(item.commit_hash.clone()).take(1),
                            ),
                            lines_added: item.lines_added,
                            lines_removed: item.lines_deleted,
                            users: HashSet::from_iter(std::iter::repeat(item.user.clone()).take(1)),
                        },
                    );
                }
                if !cumulative {
                    break;
                };
            }
        }
    }

    intervals.into_iter().map(|x| x.attach()).collect()
}

pub fn map_timeline_comparison(
    data: Vec<ComparisonDWH>,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
    repos: &Vec<i32>,
    branches: &Vec<String>,
    users: &Vec<String>,
) -> ComparisonJsonWrapper {
    let tz: Tz = timezone.parse().unwrap();
    let start_tz: DateTime<Tz> = get_datetime_tz_from_seconds(start, &tz);
    let end_tz = get_datetime_tz_from_seconds(end, &tz);
    let mut general_intervals =
        generate_intervals(start_tz.clone(), end_tz.clone(), interval, |s, e| {
            TimelineComparisonEntry {
                start: s,
                end: e,
                time: 0,
                lines_added: 0,
                lines_removed: 0,
                commits: Default::default(),
                users: Default::default(),
            }
        });
    let mut filtered_intervals =
        generate_intervals(start_tz, end_tz, interval, |s, e| TimelineComparisonEntry {
            start: s,
            end: e,
            time: 0,
            lines_added: 0,
            lines_removed: 0,
            commits: Default::default(),
            users: Default::default(),
        });
    let mut filtered_data: Vec<ComparisonDWH> = vec![];
    let mut repo_names: HashSet<String> = HashSet::default();
    let mut user_names: HashSet<String> = HashSet::default();
    let mut branch_names: HashSet<String> = HashSet::default();
    for item in &data {
        repo_names.insert(item.repo_name.clone());
        user_names.insert(item.user.clone());
        branch_names.insert(item.branch.clone());
        accumulate_data(&mut general_intervals, &item);
        if repos.contains(&item.repo)
            && branches.contains(&item.branch)
            && users.contains(&item.user)
        {
            accumulate_data(&mut filtered_intervals, &item);
            filtered_data.push(item.clone());
        }
    }

    ComparisonJsonWrapper {
        branches: branch_names.into_iter().filter(|b| b.len() > 0).collect(),
        users: user_names.into_iter().filter(|u| u.len() > 0).collect(),
        repos: repo_names.into_iter().filter(|r| r.len() > 0).collect(),
        time: get_comparison_stats(|e| e.time as f64 / 60.0 / 60.0, &data, &filtered_data),
        commits: get_comparison_stats(|e| 1.0, &data, &filtered_data), // TODO:  validate group by
        lines_added: get_comparison_stats(|e| e.lines_added as f64, &data, &filtered_data),
        lines_removed: get_comparison_stats(|e| e.lines_removed as f64, &data, &filtered_data),
        timeline: general_intervals
            .into_iter()
            .map(TimelineComparisonJsonEntry::from)
            .collect(),
        filtered_timeline: filtered_intervals
            .into_iter()
            .map(TimelineComparisonJsonEntry::from)
            .collect(),
    }
}

fn get_comparison_stats<F>(
    mut key_data: F,
    general_data: &Vec<ComparisonDWH>,
    filtered_data: &Vec<ComparisonDWH>,
) -> ComparisonStatJson
where
    F: FnMut(&ComparisonDWH) -> f64,
    Tz: TimeZone,
{
    let mut data: Vec<(String, f64)> = general_data
        .iter()
        .map(|e| (e.user.clone(), key_data(&e)))
        .collect();
    data.sort_by_key(|(u, _)| u.clone());
    let mut data: Vec<(String, f64)> = data
        .into_iter()
        .group_by(|e| e.0.clone())
        .into_iter()
        .map(|(id, vals)| (id, vals.map(|i| i.1).sum::<f64>()))
        .collect();

    for (n, v) in &data {
        println!("{} - {}", n, v);
    }

    data.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    data.reverse();

    let filtered: Vec<f64> = filtered_data.iter().map(|e| key_data(&e)).collect();

    let highlighted_total = filtered.iter().sum();
    ComparisonStatJson {
        total: data.iter().map(|e| e.1).sum(),
        highlighted: highlighted_total,
        rank: data.iter().filter(|(_, v)| highlighted_total < *v).count() as i32,
        data: data
            .into_iter()
            .enumerate()
            .map(|(i, (_, v))| ComparisonStatJsonEntry {
                rank: i as i32,
                value: v,
            })
            .collect(),
    }
}

fn accumulate_data<Tz: TimeZone>(
    filtered_intervals: &mut Vec<TimelineComparisonEntry<Tz>>,
    item: &ComparisonDWH,
) {
    for i in 0..filtered_intervals.len() {
        if filtered_intervals[i].start.timestamp() <= item.timestamp
            && item.timestamp < filtered_intervals[i].end.timestamp()
        {
            filtered_intervals[i].time += item.time;
            filtered_intervals[i].lines_added += item.lines_added;
            filtered_intervals[i].lines_removed += item.lines_removed;
            filtered_intervals[i]
                .commits
                .insert(item.commit_hash.clone());
            filtered_intervals[i].users.insert(item.user.clone());
            break;
        }
    }
}

pub fn get_datetime_tz_from_seconds(seconds: i64, timezone: &Tz) -> DateTime<Tz> {
    DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(seconds as u64)).with_timezone(timezone)
}

pub fn cut_path(path: &str, depth: i32) -> String {
    let mut new_path = path
        .trim_start_matches("./")
        .split("/")
        .into_iter()
        .take(depth as usize)
        .fold(String::new(), |a, b| a + b + "/");
    new_path = format!("/{}", new_path.trim_end_matches("/").to_string());
    new_path
}
