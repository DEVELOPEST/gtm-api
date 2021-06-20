use diesel::sql_types::BigInt;

#[derive(QueryableByName, Debug)]
pub struct GroupAccessCountDWH {
    #[sql_type = "BigInt"]
    pub sum: i64,
}