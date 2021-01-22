use diesel::sql_types::{BigInt, Text};

#[derive(QueryableByName, Debug)]
pub struct TimelineDWH {
    #[sql_type = "Text"]
    pub user: String,
    #[sql_type = "BigInt"]
    pub time: i64,
    #[sql_type = "BigInt"]
    pub timestamp: i64,
}

#[derive(QueryableByName, Debug)]
pub struct FileEditDWH {
    #[sql_type = "Text"]
    pub user: String,
    #[sql_type = "BigInt"]
    pub time: i64,
    #[sql_type = "BigInt"]
    pub lines_added: i64,
    #[sql_type = "BigInt"]
    pub lines_deleted: i64,
    #[sql_type = "BigInt"]
    pub timestamp: i64,
}