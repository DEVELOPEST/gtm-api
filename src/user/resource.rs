use serde::Serialize;
use schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
pub struct UserIdResponse {
    pub user_id: i32,
}