use diesel::{Insertable, Queryable};

use crate::schema::login;

#[derive(Insertable, Queryable)]
#[table_name = "login"]
pub struct Login {
    pub id: i32,
    pub user: i32,
    pub login_type: i32,
    pub identity_hash: String,
    pub token: String,
    pub refresh_token: Option<String>,
    pub exp: Option<i64>,
}