use diesel::{Queryable, Insertable};
use crate::schema::sync_clients;

#[derive(Queryable, Insertable)]
#[table_name = "sync_clients"]
pub struct SyncClient {
    pub id: i32,
    pub base_url: String,
    pub api_key: String,
    pub sync_client_type: i32,
}

#[derive(Insertable)]
#[table_name = "sync_clients"]
pub struct NewSyncClient {
    pub base_url: String,
    pub api_key: String,
    pub sync_client_type: i32,
}