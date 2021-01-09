use crate::helpers::timeline::generate_hour_base_data;
use crate::models::hour_data::{HourDataJson};
use crate::models::hour_data_dwh::{HourDataDWH};

pub fn map_day_data(
    data: Vec<HourDataDWH>,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<HourDataJson> {
    let mut hour_data = generate_hour_base_data(start, end, timezone, interval);
    for item in data {
        for i in 0..hour_data.len() {
            if hour_data[i].start <= item.timestamp && item.timestamp < hour_data[i].end {
                hour_data[i].time = hour_data[i].time + item.time;
                if !hour_data[i].users.contains(&item.user) {
                    hour_data[i].users.push(item.user.to_string());
                }
                break;
            }
        }
    }
    
    hour_data.into_iter().map(|x| x.attach()).collect()
}