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