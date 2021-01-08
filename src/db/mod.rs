use rocket_contrib::databases::{database, diesel::PgConnection};

pub mod users;
pub mod commits;
pub mod repositories;
pub mod files;
pub mod timelines;
pub mod git_groups;
pub mod git_groups_repositories;

#[database("diesel_postgres_pool")]
pub struct Conn(PgConnection);
