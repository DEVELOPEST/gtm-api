use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::time::{Duration, UNIX_EPOCH};

use chrono::{Datelike, DateTime, Timelike, Utc};
use chrono_tz::Tz;

use crate::domain::timeline::dwh::{FileEditDWH, PathlessFileEditDWH, TimelineDWH};
use crate::domain::timeline::helper::{generate_activity_interval, generate_intervals};
use crate::domain::timeline::resources::{ActivityJson, Interval, IntervalJson, SubdirLevelTimeline, SubdirLevelTimelineEntry, SubdirLevelTimelineJson};

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
    let mut intervals = generate_intervals(
        start_tz, end_tz, interval, |s, e| Interval {
            start: s,
            end: e,
            time: 0,
            users: vec![],
        });
    for item in data {
        for i in 0..intervals.len() {
            if (intervals[i].start.timestamp() <= item.timestamp || cumulative)
                && item.timestamp < intervals[i].end.timestamp() {
                intervals[i].time += item.time;
                if !intervals[i].users.contains(&item.user) {
                    intervals[i].users.push(item.user.to_string());
                }
                if !cumulative { break; }
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
    let mut intervals = generate_intervals(
        start_tz, end_tz, interval, |s, e| SubdirLevelTimeline {
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
                && item.timestamp < intervals[i].end.timestamp() {
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
                    intervals[i].directories.insert(cut_path.clone(), SubdirLevelTimelineEntry {
                        path: cut_path,
                        time: item.time,
                        commits: HashSet::from_iter(std::iter::repeat(item.commit_hash.clone()).take(1)),
                        lines_added: item.lines_added,
                        lines_removed: item.lines_deleted,
                        users: HashSet::from_iter(std::iter::repeat(item.user.clone()).take(1)),
                    });
                }
                if !cumulative { break };
            }
        }
    }

    intervals.into_iter().map(|x| x.attach()).collect()
}

pub fn get_datetime_tz_from_seconds(seconds: i64, timezone: &Tz) -> DateTime<Tz> {
    DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(seconds as u64)).with_timezone(timezone)
}

pub fn cut_path(path: &str, depth: i32) -> String {
    let mut new_path = path.trim_start_matches("./").split("/")
        .into_iter()
        .take(depth as usize)
        .fold(String::new(), |a, b| a + b + "/");
    new_path = format!("/{}", new_path.trim_end_matches("/").to_string());
    new_path
}
