use chrono::{DateTime, Utc, TimeZone, Datelike};
use chrono_tz::Tz;

use crate::models::interval::Interval;
use std::time::{UNIX_EPOCH, Duration};

pub fn generate_intervals<Tz: TimeZone>(
    start: i64,
    end: i64,
    timezone: &Tz,
    interval: &str,
) -> Vec<Interval<Tz>> {
    // TODO Validation for timezone and range
    let d_start = UNIX_EPOCH + Duration::from_secs(start as u64);
    let d_end = UNIX_EPOCH + Duration::from_secs(end as u64);
    let start_date = DateTime::<Utc>::from(d_start);
    let end_date = DateTime::<Utc>::from(d_end);

    let start_tz = start_date.with_timezone(timezone);
    let end_tz = end_date.with_timezone(timezone);

    let mut intervals = Vec::new();
    let mut current_start_tz = start_tz.clone();
    let mut current_end_tz = get_next_interval_start(start_tz, interval);

    while end_tz.ge(&current_start_tz) {
        intervals.push(Interval {
            start: current_start_tz.clone(),
            end: current_end_tz.clone() + chrono::Duration::seconds(-1),
            time: 0,
            users: Vec::new(),
        });

        current_start_tz = get_next_interval_start(current_start_tz, interval);
        current_end_tz = get_next_interval_start(current_end_tz, interval);
    }

    intervals
}

fn get_next_interval_start<Tz: TimeZone>(
    date_time_tz: DateTime<Tz>,
    interval: &str
) -> DateTime<Tz> {
    if interval == "HOUR" || interval == "DAY" || interval == "WEEK" {
        return date_time_tz.clone() + chrono::Duration::seconds(step_from_interval(interval));
    }
    get_next_month(date_time_tz)
}

fn get_next_month<Tz: TimeZone>(
    date_time_tz: DateTime<Tz>,
) -> DateTime<Tz> {
    date_time_tz.with_month(date_time_tz.month() + 1).unwrap_or(
        date_time_tz.with_year(date_time_tz.year() + 1).unwrap().with_month(1).unwrap()
    )
}

fn step_from_interval(interval: &str) -> i64 {
    return match interval {
        "HOUR" => 60 * 60,
        "DAY" => 60 * 60 * 24,
        _ => 60 * 60 * 24 * 7
    }
}
