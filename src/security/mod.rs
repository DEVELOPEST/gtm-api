use serde::Serialize;

pub mod api_key;
pub mod jwt;
pub mod routes;
pub mod service;

#[derive(Debug, Serialize)]
pub struct AuthError {
    message: String
}