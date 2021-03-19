use diesel::Queryable;
use serde::Serialize;

use crate::errors::{Error};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct AuthUser {
    pub user_id: i32,
    pub roles: Vec<String>,
}

impl AuthUser {
    pub fn require_role(&self, role: &String) -> Result<(), Error> {
        if !self.roles.contains(role) {
            return Err(Error::AuthorizationError("Unauthorized!"));
        }
        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJson {
    pub id: i32,
    pub username: String,
    pub roles: Vec<String>,
}

impl User {
    pub fn attach(&self, roles: Vec<String>) -> UserJson {
        UserJson {
            id: self.id,
            username: self.username.clone(),
            roles,
        }
    }
}