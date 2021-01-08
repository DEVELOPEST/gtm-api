#[derive(Queryable)]
pub struct HourDataDWH {
    pub user: String,
    pub time: i64,
    pub timestamp: i64,
}