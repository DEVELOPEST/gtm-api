use crate::domain::sync::model::{SyncClient, NewSyncClient};
use crate::errors::Error;
use reqwest;
use crate::domain::sync::resource::{AddRepositoryDto, AddRepoResponseDto, SyncClientDto};
use diesel::PgConnection;
use crate::domain::sync;
use crate::common;

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

pub fn add_sync_client(
    conn: &PgConnection,
    base_url: String,
    client_type: i32
) -> Result<SyncClientDto, Error> {
    let client  = NewSyncClient {
        base_url,
        api_key: common::random::random_string(32),
        sync_client_type: client_type
    };

    sync::db::create_sync_client(conn, &client)?;

    Ok(SyncClientDto {
        api_key: client.api_key,
        sync_client_type: client.sync_client_type
    })
}

pub fn delete_sync_client(conn: &PgConnection, api_key: &str) -> Result<usize, Error> {
    sync::db::delete_sync_client(conn, api_key)
}