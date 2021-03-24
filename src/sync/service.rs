use crate::sync::model::SyncClient;
use crate::errors::Error;
use reqwest;
use crate::sync::resource::{AddRepositoryDto, AddRepoResponseDto};

pub async fn track_repository(sync_client: &SyncClient, clone_url: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let add_repo_dto = AddRepositoryDto { url: clone_url.to_string() };
    let resp = client.post(&format!("{}/repositories", sync_client.base_url))
        .json(&add_repo_dto)
        .send()
        .await?
        .json::<AddRepoResponseDto>()
        .await?;

    if resp.success && resp.sync_url.is_some() {
        return Ok(resp.sync_url.unwrap());
    }
    Err(Error::Custom("Something went wrong adding repository to sync!"))
}