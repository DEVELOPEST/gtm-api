#[derive(Queryable, Debug)]
pub struct TimelineDWH {
    pub user: String,
    pub time: i64,
    pub timestamp: i64,
}