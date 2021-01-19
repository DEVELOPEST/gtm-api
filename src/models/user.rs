use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum UserRole {
    ADMIN,
    REGULAR,
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
}

#[derive(Serialize)]
pub struct AuthUser {
    pub user_id: i32,
    pub role: UserRole,
}
