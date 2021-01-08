use crate::helpers::timeline::generate_hour_base_data;
use crate::models::timeline::HourDataJson;

pub fn map_day_data(
    data: Vec<(String, String, i64, i64)>,
    start: i64,
    end: i64,
) -> Vec<HourDataJson> {
    let mut hour_data = generate_hour_base_data(start, end);
    for item in data {
        for i in 0..hour_data.len() {
            if hour_data[i].start <= item.3 && item.3 < hour_data[i].end {
                hour_data[i].time = hour_data[i].time + item.2;
                if !hour_data[i].users.contains(&item.1) {
                    hour_data[i].users.push(item.1.to_string());
                }
                break;
            }
        }
    }

    hour_data.into_iter().map(|x| x.attach()).collect()
}