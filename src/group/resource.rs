use serde::Serialize;
use schemars::JsonSchema;
use crate::group_access::resource::GroupAccessJson;

#[derive(Debug, Serialize, JsonSchema)]
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

#[derive(Debug, Serialize, JsonSchema)]
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

#[derive(Debug, Serialize, JsonSchema)]
pub struct GroupExportDataJson {
    pub user_name: String,
    pub user: String,
    pub provider: String,
    pub repository: String,
    pub path: String,
    pub is_app: bool,
    pub files_count: i64,
    pub timestamp: i64,
    pub message: String,
    pub total_time: i64,
    pub lines_added: i64,
    pub lines_removed: i64,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct GroupStatsJson {
    pub users: Vec<GroupUserStatsJson>,
    pub files: Vec<GroupFileStatsJson>,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupJson {
    pub id: i32,
    pub name: String,
    pub added_at: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupWithAccessJson {
    pub id: i32,
    pub name: String,
    pub added_at: String,
    pub group_access: Option<GroupAccessJson>
}