use diesel::{ExpressionMethods, PgConnection, QueryDsl};

use crate::diesel::RunQueryDsl;
use crate::errors::Error;
use crate::schema;
use crate::sync::model::{SyncClient, NewSyncClient};

pub fn find_all_sync_clients_by_type(
    conn: &PgConnection,
    type_id: i32,
) -> Result<Vec<SyncClient>, Error> {
    Ok(schema::sync_clients::table
        .filter(schema::sync_clients::sync_client_type.eq(type_id))
        .load::<SyncClient>(conn)?)
}

pub fn find_by_api_key(
    conn: &PgConnection,
    api_key: &str,
) -> Result<SyncClient, Error> {
    Ok(schema::sync_clients::table
        .filter(schema::sync_clients::api_key.eq(api_key))
        .first::<SyncClient>(conn)?)
}

pub fn create_sync_client(conn: &PgConnection, client: &NewSyncClient) -> Result<usize, Error> {
    Ok(diesel::insert_into(schema::sync_clients::table)
        .values(client)
        .execute(conn)?)
}

pub fn delete_sync_client(conn: &PgConnection, api_key: &str) -> Result<usize, Error> {
    Ok(diesel::delete(schema::sync_clients::table
        .filter(schema::sync_clients::api_key.eq(api_key)))
        .execute(conn)?)
}