use rocket_contrib::databases::{database, diesel::PgConnection};

pub mod users;
pub mod commits;
pub mod repositories;
pub mod files;
pub mod timelines;
pub mod groups;
pub mod group_relations;


#[database("diesel_postgres_pool")]
pub struct Conn(PgConnection);
