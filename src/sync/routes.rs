use rocket_contrib::json::Json;
use rocket_okapi::{JsonSchema, openapi};
use serde::Deserialize;
use validator::Validate;

use crate::db::Conn;
use crate::errors::{Error, FieldValidator};
use crate::sync;
use crate::sync::resource::SyncClientDto;
use crate::user::model::AuthUser;
use crate::security::api_key::ApiKey;

#[derive(Deserialize, Validate, JsonSchema)]
pub struct SyncClient {
    #[validate(length(min = 8))]
    base_url: Option<String>,
    client_type: Option<i32>,
}

#[openapi]
#[post("/sync/client", format = "json", data = "<sync_client>")]
pub fn post_sync_client(
    _auth_user: AuthUser,
    conn: Conn,
    sync_client: Json<SyncClient>,
) -> Result<Json<SyncClientDto>, Error> {
    let sync_client = sync_client.into_inner();
    let mut extractor = FieldValidator::validate(&sync_client);
    let base_url = extractor.extract("base_url", sync_client.base_url);
    let client_type = extractor.extract("client_type", sync_client.client_type);
    extractor.check()?;

    let res = sync::service::add_sync_client(&conn, base_url, client_type)?;
    Ok(Json(res))
}

#[openapi]
#[delete("/sync/client")]
pub fn delete_sync_client(
    _auth_user: AuthUser,
    api_key: ApiKey,
    conn: Conn,
) -> Result<Json<usize>, Error> {

    let res = sync::service::delete_sync_client(&conn, &api_key.key)?;
    Ok(Json(res))
}
