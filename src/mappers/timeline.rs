use chrono_tz::Tz;

use crate::helpers::timeline::generate_intervals;
use crate::models::interval::IntervalJson;
use crate::models::timeline_dwh::TimelineDWH;

pub fn map_timeline(
    data: Vec<TimelineDWH>,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<IntervalJson> {
    let tz: Tz = timezone.parse().unwrap();
    let mut intervals = generate_intervals(start, end, &tz, interval);
    for item in data {
        for i in 0..intervals.len() {
            if intervals[i].start.timestamp() <= item.timestamp && item.timestamp < intervals[i].end.timestamp() {
                intervals[i].time = intervals[i].time + item.time;
                if !intervals[i].users.contains(&item.user) {
                    intervals[i].users.push(item.user.to_string());
                }
                break;
            }
        }
    }
    
    intervals.into_iter().map(|x| x.attach()).collect()
}