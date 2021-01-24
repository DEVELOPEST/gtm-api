use rocket_contrib::databases::{database, diesel::PgConnection};

pub mod service;
pub mod routes;
pub mod model;
pub mod mapper;
pub mod helper;
pub mod db;
pub mod dwh;
pub mod resources;

#[database("diesel_postgres_pool")]
pub struct Conn(PgConnection);
