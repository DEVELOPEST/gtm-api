use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GroupUserStatsJson {
    pub name: String,
    pub total_time: f64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub lines_per_hour: i32,
    pub commits: i64,
    pub commits_per_hour: f64,
    pub lines_per_commit: f64,
}

#[derive(Debug, Serialize)]
pub struct GroupFileStatsJson {
    pub path: String,
    pub total_time: f64,
    pub time_per_user: f64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub total_commits: i64,
    pub commits_per_user: f64,
    pub commits_per_hour: f64,
    pub users: i64,
    pub lines_per_hour: i32,
}

#[derive(Debug, Serialize)]
pub struct GroupStatsJson {
    pub users: Vec<GroupUserStatsJson>,
    pub files: Vec<GroupFileStatsJson>,
}