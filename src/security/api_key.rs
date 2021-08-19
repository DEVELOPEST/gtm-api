
use rocket::{Request, request};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::security::AuthError;

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
