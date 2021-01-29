use diesel::Queryable;
use serde::{Serialize};
use crate::errors::Errors;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthUser {
    pub user_id: i32,
    pub roles: Vec<String>,
}

impl AuthUser {
    pub fn has_role(&self, role: &String) -> Result<(), Errors> {
        if !self.roles.contains(role) {
            return Err(Errors::new(&[("Authorization", "Not authorized!")]));
        }
        Ok(())
    }
}
