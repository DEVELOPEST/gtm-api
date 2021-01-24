use diesel::Queryable;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ADMIN: String = "ADMIN".to_string();
    pub static ref LECTURER: String = "LECTURER".to_string();
    pub static ref USER: String = "USER".to_string();
}

#[derive(Queryable, Serialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

impl Role {
    pub fn attach(self) -> String {
        self.name
    }
}
