use serde::Serialize;
use schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupAccessJson {
    pub user: i32,
    pub group: i32,
    pub access_level_recursive: bool,
}