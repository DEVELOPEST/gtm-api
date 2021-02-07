use rocket_contrib::json::JsonValue;

use crate::user::model::AuthUser;
use crate::db::Conn;
use crate::user;
use crate::role::model::ADMIN;
use crate::errors::Errors;

#[get("/user")]
pub fn get_user(user: AuthUser, conn: Conn) -> Option<JsonValue> {
    user::db::find(&conn, user.user_id)
        .map(|user| json!({ "user": user.id }))
}

#[get("/users")]
pub fn get_users(auth_user: AuthUser, conn: Conn) -> Result<JsonValue, Errors> {
    auth_user.has_role(&ADMIN)?;
    Ok(json!({ "users": user::service::find_all(&conn)}))
}
