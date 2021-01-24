use chrono_tz::Tz;
use chrono::{DateTime, Utc, Timelike, Datelike};

use std::time::{UNIX_EPOCH, Duration};
use crate::timeline::dwh::{TimelineDWH, FileEditDWH};
use crate::timeline::resources::{IntervalJson, ActivityJson};
use crate::timeline::helper::{generate_intervals, generate_activity_interval};

pub fn map_timeline(
    data: Vec<TimelineDWH>,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<IntervalJson> {
    let tz: Tz = timezone.parse().unwrap();
    let start_tz: DateTime<Tz> = get_datetime_tz_from_seconds(start, &tz);
    let end_tz = get_datetime_tz_from_seconds(end, &tz);
    let mut intervals = generate_intervals(start_tz, end_tz, interval);
    for item in data {
        for i in 0..intervals.len() {
            if intervals[i].start.timestamp() <= item.timestamp && item.timestamp < intervals[i].end.timestamp() {
                intervals[i].time += item.time;
                if !intervals[i].users.contains(&item.user) {
                    intervals[i].users.push(item.user.to_string());
                }
                break;
            }
        }
    }
    
    intervals.into_iter().map(|x| x.attach()).collect()
}

pub fn map_activity(
    data: Vec<FileEditDWH>,
    timezone: &str,
    interval: &str,
) -> Vec<ActivityJson> {
    let tz: Tz = timezone.parse().unwrap();
    let interval = &*interval.to_lowercase();
    let mut intervals = generate_activity_interval(interval);

    for item in data {
        let time_point = get_datetime_tz_from_seconds(item.timestamp, &tz);
        let i = intervals.iter().position(|a| {
            a.id == match interval {
                "day" => time_point.hour() as i32,
                "week" => time_point.weekday().number_from_monday() as i32,
                "month" => time_point.month0() as i32,
                _ => 0,
            }
        }).unwrap();
        intervals[i].time += item.time;
        intervals[i].lines_added += item.lines_added;
        intervals[i].lines_removed += item.lines_deleted;
        if !intervals[i].users.contains(&item.user) {
            intervals[i].users.push(item.user);
        }
    }

    intervals.into_iter().map(|x| x.attach()).collect()
}

pub fn get_datetime_tz_from_seconds(seconds: i64, timezone: &Tz) -> DateTime<Tz> {
    DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(seconds as u64)).with_timezone(timezone)
}
