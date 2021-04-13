use diesel::sql_types::{BigInt, Text, Integer};

#[derive(QueryableByName, Debug)]
pub struct TimelineDWH {
    #[sql_type = "Text"]
    pub user: String,
    #[sql_type = "BigInt"]
    pub time: i64,
    #[sql_type = "BigInt"]
    pub timestamp: i64,
    #[sql_type = "BigInt"]
    pub lines_added: i64,
    #[sql_type = "BigInt"]
    pub lines_removed: i64,
}

#[derive(QueryableByName, Debug)]
pub struct PathlessFileEditDWH {
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

#[derive(QueryableByName, Debug)]
pub struct FileEditDWH {
    #[sql_type = "Text"]
    pub user: String,
    #[sql_type = "Text"]
    pub path: String,
    #[sql_type = "BigInt"]
    pub time: i64,
    #[sql_type = "BigInt"]
    pub lines_added: i64,
    #[sql_type = "BigInt"]
    pub lines_deleted: i64,
    #[sql_type = "BigInt"]
    pub timestamp: i64,
    #[sql_type = "Text"]
    pub commit_hash: String,
}

#[derive(QueryableByName, Debug)]
pub struct ComparisonDWH {
    #[sql_type = "Text"]
    pub user: String,
    #[sql_type = "Integer"]
    pub repo: i32,
    #[sql_type = "Text"]
    pub repo_name: String,
    #[sql_type = "Text"]
    pub commit_hash: String,
    #[sql_type = "Text"]
    pub branch: String,
    #[sql_type = "BigInt"]
    pub timestamp: i64,
    #[sql_type = "BigInt"]
    pub time: i64,
    #[sql_type = "BigInt"]
    pub lines_added: i64,
    #[sql_type = "BigInt"]
    pub lines_removed: i64,
}