use chrono::{DateTime, Utc, TimeZone};
use chrono_tz::Tz;

use crate::models::hour_data::HourData;
use std::time::{UNIX_EPOCH, Duration};

pub fn generate_hour_base_data<Tz: TimeZone>(
    start: i64,
    end: i64,
    timezone: &Tz,
    interval: &str,
) -> Vec<HourData<Tz>> {
    // TODO Validation for timezone and range
    let d_start = UNIX_EPOCH + Duration::from_secs(start as u64);
    let d_end = UNIX_EPOCH + Duration::from_secs(end as u64);
    let start_date = DateTime::<Utc>::from(d_start);
    let end_date = DateTime::<Utc>::from(d_end);

    let start_tz = start_date.with_timezone(timezone);
    let end_tz = end_date.with_timezone(timezone);

    let step = step_from_interval(interval);
    let time_diff = end_date - start_date;
    let steps = time_diff.num_seconds() / step;

    let mut hour_data = Vec::new();

    for x in 0..steps {
        hour_data.push(HourData {
            start: start_tz.clone() + chrono::Duration::seconds(step * x),
            end: end_tz.clone() + chrono::Duration::seconds(step * (x + 1) - 1),
            hour: x as i32,
            time: 0,
            users: Vec::new(),
        });
    }
    hour_data
}

fn step_from_interval(interval: &str) -> i64 {
    return match interval {
        "hour" => 60 * 60,
        "day" => 60 * 60 * 24,
        "week" => 60 * 60 * 24 * 7,
        "year" => 60 * 60 * 24 * 365,
        _ => 60 * 60
    }
}