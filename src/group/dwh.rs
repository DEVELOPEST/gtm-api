use diesel::sql_types::{BigInt, Text};
use serde::Serialize;

#[derive(QueryableByName, Debug, Serialize)]
pub struct GroupRepoStats {
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