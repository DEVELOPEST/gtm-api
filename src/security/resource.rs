use serde::Serialize;
use schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
pub struct JwtResponse {
    pub(crate) jwt: String
}