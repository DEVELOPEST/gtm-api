use diesel::sql_types::{BigInt, Text};
use std::collections::HashSet;

#[derive(QueryableByName, Debug)]
pub struct GroupUserStats {
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "BigInt"]
    pub total_time: i64,
    #[sql_type = "BigInt"]
    pub lines_added: i64,
    #[sql_type = "BigInt"]
    pub lines_removed: i64,
    #[sql_type = "BigInt"]
    pub commits: i64,
}

#[derive(QueryableByName, Debug)]
pub struct GroupFileStats {
    #[sql_type = "Text"]
    pub path: String,
    #[sql_type = "BigInt"]
    pub total_time: i64,
    #[sql_type = "BigInt"]
    pub lines_added: i64,
    #[sql_type = "BigInt"]
    pub lines_removed: i64,
    #[sql_type = "BigInt"]
    pub commits: i64,
    #[sql_type = "Text"]
    pub user: String,
}

#[derive(Debug, Clone)]
pub struct GroupFileStatsWrapper {
    pub path: String,
    pub total_time: i64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub commits: i64,
    pub users: HashSet<String>,
}