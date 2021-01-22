use rocket_contrib::json::JsonValue;

use crate::db;
use crate::models::user::AuthUser;

#[get("/user")]
pub fn get_user(user: AuthUser, conn: db::Conn) -> Option<JsonValue> {
    db::users::find(&conn, user.user_id)
        .map(|user| json!({ "user": user.id }))
}