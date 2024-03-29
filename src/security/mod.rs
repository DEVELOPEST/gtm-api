use serde::Serialize;

pub mod api_key;
pub mod jwt;
pub mod routes;
pub mod service;
pub mod db;
pub mod config;
pub mod model;
pub mod oauth;
pub mod constants;
pub mod resource;

#[derive(Debug, Serialize)]
pub struct AuthError {
    message: String
}