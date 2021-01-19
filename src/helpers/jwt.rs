use chrono::Utc;
use jsonwebtoken::{Header, Validation};
use jsonwebtoken::{DecodingKey, EncodingKey};
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::Conn;
use crate::routes::login::LoginDto;
use crate::models::user::User;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7;
// in seconds
static SECRET: [u8; 16] = [0u8; 16];

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
}

#[derive(Debug, Serialize)]
pub struct AuthError {
    message: String
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
    type Error = status::Custom<Json<AuthError>>;
    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<Self, status::Custom<Json<AuthError>>> {
        let conn = request.guard::<Conn>().unwrap();
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let auth_str = auth_header.to_string();
            if auth_str.starts_with("Bearer") {
                let token = auth_str[6..auth_str.len()].trim();
                if let Ok(token_data) = decode_token(token.to_string()) {
                    if verify_token(&token_data, &conn) {
                        return Outcome::Success(token_data.claims);
                    }
                }
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::Unauthorized,
                Json(AuthError {
                    message: String::from("Invalid token!"),
                }),
            ),
        ))
    }
}

pub fn generate_token(login: LoginDto) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = AuthToken {
        iat: now,
        exp: now + ONE_WEEK,
        user: login.email,
    };

    jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(&SECRET)).unwrap()
}

fn decode_token(token: String) -> Result<TokenData<AuthToken>> {
    jsonwebtoken::decode::<AuthToken>(&token, &DecodingKey::from_secret(&SECRET), &Validation::default())
}

fn verify_token(token_data: &TokenData<AuthToken>, conn: &Conn) -> bool {
    User::is_valid_login_session(&token_data.claims, conn)
}