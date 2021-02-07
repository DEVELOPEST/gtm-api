use std::sync::RwLock;

use lazy_static::lazy_static;
use rocket::{Outcome, Request, request};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::response::status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::security::AuthError;
use rocket::fairing::AdHoc;

lazy_static! {
    // This is overridden in Rocket.toml
    pub static ref API_KEY: RwLock<String> = RwLock::new("".to_string());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKey {
    pub key: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = status::Custom<Json<AuthError>>;
    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<Self, status::Custom<Json<AuthError>>> {
        if let Some(auth_header) = request.headers().get_one("API-Key") {
            let api_key = auth_header.to_string();
            return Outcome::Success(ApiKey {
                key: api_key
            });
        }
        Outcome::Failure((
            Status::Unauthorized,
            status::Custom(
                Status::Unauthorized,
                Json(AuthError {
                    message: String::from("No API-key found!"),
                }),
            ),
        ))
    }
}

pub fn manage() -> AdHoc {
    AdHoc::on_attach("Manage api-key", |rocket| {
        // Rocket doesn't expose it's own secret_key, so we use our own here.
        let cfg = rocket.config();
        let extras = &cfg.extras;

        let api_key = extras.get("api-key");

        if api_key.is_some() {
            let api_key_table = api_key.unwrap().as_table().unwrap();
            let key = api_key_table.get("sync_api_key").unwrap().as_str().unwrap();
            let mut global_secret = API_KEY.write().unwrap();
            *global_secret = key.to_string();
        }

        Ok(rocket)
    })
}