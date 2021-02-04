use std::sync::RwLock;

use chrono::Utc;
use jsonwebtoken::{Header, Validation};
use jsonwebtoken::{DecodingKey, EncodingKey};
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use lazy_static::lazy_static;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use crate::user::model::{AuthUser, User};
use crate::db::Conn;
use crate::role;
use diesel::PgConnection;


const TOKEN_DURATION: i64 = 60 * 5;
// in seconds
lazy_static! {
    static ref SECRET: RwLock<String> = RwLock::new("zRXL2u7hw84MTir+ZMjIGg==".to_string());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    pub user: i32,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthError {
    message: String
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
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
                        return Outcome::Success(token_data.claims.to_auth_user());
                    }
                }
            }
        }

        Outcome::Failure((
            Status::Unauthorized,
            status::Custom(
                Status::Unauthorized,
                Json(AuthError {
                    message: String::from("Invalid token!"),
                }),
            ),
        ))
    }
}

impl AuthToken {
    pub fn to_auth_user(&self) -> AuthUser {
        AuthUser {
            user_id: self.user,
            roles: self.roles.clone(),
        }
    }
}

pub fn generate_token_for_user(conn: &PgConnection, user: User) -> Option<String> {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = AuthToken {
        iat: now,
        exp: now + TOKEN_DURATION,
        user: user.id,
        roles: role::db::find_all_by_user(conn, user.id)
            .into_iter().map(|x| x.attach()).collect(),
    };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_base64_secret(&*SECRET.read().unwrap()).unwrap(),
    ).ok()
}

fn decode_token(token: String) -> Result<TokenData<AuthToken>> {
    jsonwebtoken::decode::<AuthToken>(
        &token,
        &DecodingKey::from_base64_secret(&*SECRET.read().unwrap()).unwrap(),
        &Validation::default(),
    )
}

fn verify_token(token_data: &TokenData<AuthToken>, _conn: &Conn) -> bool {
    Utc::now().timestamp_nanos() / 1_000_000_000 < token_data.claims.exp
}

pub fn manage() -> AdHoc {
    AdHoc::on_attach("Manage config", |rocket| {
        // Rocket doesn't expose it's own secret_key, so we use our own here.
        let cfg = rocket.config();
        let extras = &cfg.extras;

        let secret_value = extras.get("jwt");

        if secret_value.is_some() {
            let secret_table = secret_value.unwrap().as_table().unwrap();
            let secret = secret_table.get("secret").unwrap().as_str().unwrap();
            let mut global_secret = SECRET.write().unwrap();
            *global_secret = secret.to_string();
        }

        Ok(rocket)
    })
}