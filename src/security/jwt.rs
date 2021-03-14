use chrono::Utc;
use diesel::PgConnection;
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
use crate::role;
use crate::security::AuthError;
use crate::security::config;
use crate::user::model::{AuthUser, User};

const TOKEN_DURATION: i64 = 24 * 60 * 60; // seconds

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    pub user: i32,
    pub username: String,
    pub roles: Vec<String>,
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
                if let Some(auth_user) = get_auth_user_from_token(&conn, token) {
                    return Outcome::Success(auth_user);
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
        username: user.username,
        roles: role::db::find_all_by_user(conn, user.id)
            .into_iter().map(|x| x.attach()).collect(),
    };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_base64_secret(&*config::JWT_SECRET.read().unwrap()).unwrap(),
    ).ok()
}

pub fn get_auth_user_from_token(conn: &PgConnection, token: &str) -> Option<AuthUser> {
    if let Ok(token_data) = decode_token(token.to_string()) {
        if verify_token(&token_data, &conn) {
            return Some(token_data.claims.to_auth_user());
        }
    }
    None
}

fn decode_token(token: String) -> Result<TokenData<AuthToken>> {
    jsonwebtoken::decode::<AuthToken>(
        &token,
        &DecodingKey::from_base64_secret(&*config::JWT_SECRET.read().unwrap()).unwrap(),
        &Validation::default(),
    )
}

fn verify_token(token_data: &TokenData<AuthToken>, _conn: &PgConnection) -> bool {
    Utc::now().timestamp_nanos() / 1_000_000_000 < token_data.claims.exp
}
