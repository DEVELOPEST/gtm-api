use chrono_tz::Tz;
use chrono::{DateTime, Utc, Timelike};

use std::time::{UNIX_EPOCH, Duration};

use crate::helpers::timeline::generate_intervals;
use crate::models::interval::{IntervalJson, ActivityJson, Activity};
use crate::models::timeline_dwh::TimelineDWH;

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
    data: Vec<TimelineDWH>,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<ActivityJson> {
    let tz: Tz = timezone.parse().unwrap();
    let start_tz: DateTime<Tz> = get_datetime_tz_from_seconds(start, &tz);
    let end_tz = get_datetime_tz_from_seconds(end, &tz);

    let mut interval : Vec<Activity> = vec![];
    for i in 0..24 {
        interval.push(Activity{
            id: i,
            label: format!("{}", i),
            time: 0,
            lines_added: 0,
            lines_removed: 0,
            users: vec![]
        })
    }

    for item in data {
        let time_point = get_datetime_tz_from_seconds(item.timestamp, &tz);
        let i = interval.iter().position(|a| a.id == time_point.hour() as i32).unwrap();
        interval[i].time += item.time;
        if !interval[i].users.contains(&item.user) {
            interval[i].users.push(item.user);
        }
    }

    interval.into_iter().map(|x| x.attach()).collect()
}

pub fn get_datetime_tz_from_seconds(seconds: i64, timezone: &Tz) -> DateTime<Tz> {
    DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(seconds as u64)).with_timezone(timezone)
}
