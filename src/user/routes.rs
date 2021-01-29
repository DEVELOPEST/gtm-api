use rocket_contrib::json::JsonValue;

use crate::user::model::AuthUser;
use crate::db::Conn;
use crate::user;

#[get("/user")]
pub fn get_user(user: AuthUser, conn: Conn) -> Option<JsonValue> {
    user::db::find(&conn, user.user_id)
        .map(|user| json!({ "user": user.id }))
}