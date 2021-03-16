use diesel::{Insertable, Queryable};

use crate::schema::logins;
use crate::schema::login_types;

#[derive(Insertable, Queryable)]
#[table_name = "logins"]
pub struct Login {
    pub id: i32,
    pub user: i32,
    pub login_type: i32,
    pub identity_hash: String,
    pub token: String,
    pub refresh_token: Option<String>,
    pub exp: Option<i64>,
}

#[derive(Insertable, Queryable)]
#[table_name = "login_types"]
pub struct LoginType {
    pub id: i32,
    pub name: String,
}