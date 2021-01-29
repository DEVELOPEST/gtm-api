use chrono::{Datelike, DateTime, TimeZone};
use crate::timeline::resources::{Interval, Activity};


pub trait DateTimeExt<Tz: TimeZone> {
    fn next_month(&self) -> DateTime<Tz>;
}

impl<Tz: TimeZone> DateTimeExt<Tz> for DateTime<Tz> {
    fn next_month(&self) -> DateTime<Tz> {
        self.with_month(self.month() + 1)
            .unwrap_or(self.with_year(self.year() + 1).unwrap().with_month(1).unwrap())
    }
}

pub fn generate_intervals<Tz: TimeZone>(
    start_tz: DateTime<Tz>,
    end_tz: DateTime<Tz>,
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
    date_time_tz.next_month()
}

fn get_interval_duration(interval: &str) -> chrono::Duration {
    return match interval {
        "HOUR" => chrono::Duration::hours(1),
        "DAY" => chrono::Duration::days(1),
        _ => chrono::Duration::weeks(1)
    }
}

pub fn generate_activity_interval(interval: &str) -> Vec<Activity> {
    let mut res: Vec<Activity> = vec![];
    let interval= &*interval.to_lowercase();
    match interval {
        "day" => {
            for i in 0..24 {
                res.push(Activity {
                    id: i,
                    label: format!("{}", i),
                    time: 0,
                    lines_added: 0,
                    lines_removed: 0,
                    users: vec![],
                })
            }
        },
        "week" => {
            let days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
            for i in 0..days.len() {
                res.push(Activity {
                    id: (i + 1) as i32,
                    label: days[i].to_string(),
                    time: 0,
                    lines_added: 0,
                    lines_removed: 0,
                    users: vec![],
                })
            }
        },
        "month" => {
            for i in 0..31 {
                res.push(Activity {
                    id: i,
                    label: format!("{}", i),
                    time: 0,
                    lines_added: 0,
                    lines_removed: 0,
                    users: vec![],
                })
            }
        },
        _ => {}
    }
    res
}
