use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct AddRepoResponseDto {
    pub success: bool,
    pub sync_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AddRepositoryDto {
    pub url: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SyncClientDto {
    pub api_key: String,
    pub sync_client_type: i32,
}