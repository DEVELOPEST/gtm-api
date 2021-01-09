use chrono::{DateTime};

use crate::models::hour_data::HourData;

pub fn generate_hour_base_data(
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<HourData> {
    // TODO Validation for timezone and range
    let start_str = format!("{} {}", start, timezone);
    let end_str = format!("{} {}", end, timezone);
    let start_date = DateTime::parse_from_str(&start_str, "%s %Z")
        .unwrap();
    let end_date = DateTime::parse_from_str(&end_str, "%s %Z")
        .unwrap();
    let step = step_from_interval(interval);
    let time_diff = end_date - start_date;
    let steps = time_diff.num_seconds() / step;

    let mut hour_data = Vec::new();

    for x in 0..steps {
        hour_data.push(HourData {
            start: start + step * x,
            end: start + step * (x + 1),
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