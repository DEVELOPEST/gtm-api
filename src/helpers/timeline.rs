use crate::models::hour_data::HourData;

pub fn generate_hour_base_data(
    start: i64,
    end: i64
) -> Vec<HourData> {
    // TODO Validation is for start and end!
    let mut hour_data = Vec::new();
    let hour_in_seconds = 3600;
    for x in 0..24 {
        hour_data.push(HourData {
            start: start + hour_in_seconds * x,
            end: start + hour_in_seconds * (x + 1),
            hour: x as i32,
            time: 0,
            users: Vec::new()
        });
    }
    hour_data
}