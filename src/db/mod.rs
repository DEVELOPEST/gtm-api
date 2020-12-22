use rocket_contrib::databases::{database, diesel::PgConnection};

pub mod users;
pub mod commits;
pub mod repositories;
pub mod files;
pub mod timelines;

#[database("diesel_postgres_pool")]
pub struct Conn(PgConnection);

use diesel::prelude::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::query_builder::*;
use diesel::pg::Pg;
use diesel::sql_types::BigInt;
