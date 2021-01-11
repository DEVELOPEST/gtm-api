use chrono::{DateTime, Utc, TimeZone, Datelike};
use chrono_tz::Tz;

use crate::models::interval::Interval;
use std::time::{UNIX_EPOCH, Duration};

pub fn generate_intervals<Tz: TimeZone>(
    start_tz: DateTime<Tz>,
    end_tz: DateTime<Tz>,
    timezone: &Tz,
    interval: &str,
) -> Vec<Interval<Tz>> {
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
        return date_time_tz.clone() + get_interval_duration(interval);
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

fn get_interval_duration(interval: &str) -> chrono::Duration {
    return match interval {
        "HOUR" => chrono::Duration::seconds(60 * 60),
        "DAY" => chrono::Duration::seconds(60 * 60 * 24),
        _ => chrono::Duration::seconds(60 * 60 * 24 * 7)
    }
}
