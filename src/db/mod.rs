use rocket_contrib::databases::{database, diesel::PgConnection};

#[database("diesel_postgres_pool")]
pub struct Conn(PgConnection);
