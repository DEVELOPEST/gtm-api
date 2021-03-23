use rocket_contrib::json::JsonValue;

use crate::user::model::AuthUser;
use crate::db::Conn;
use crate::user;
use crate::role::model::ADMIN;
use crate::errors::{Error};
use crate::role;

#[get("/user")]
pub fn get_user_id(user: AuthUser, conn: Conn) -> Result<JsonValue, Error> {
    user::db::find(&conn, user.user_id)
        .map(|user| json!({ "userId": user.id }))
}

#[get("/users")]
pub fn get_users(auth_user: AuthUser, conn: Conn) -> Result<JsonValue, Error> {
    auth_user.require_role(&ADMIN)?;
    Ok(json!(user::service::find_all(&conn)))
}

#[get("/users/<id>")]
pub fn get_user(auth_user: AuthUser, id: i32, conn: Conn) -> Result<JsonValue, Error> {
    auth_user.require_role(&ADMIN)?;
    let user = user::db::find(&conn, id)
        .unwrap()
        .attach(role::db::find_all_by_user(&conn, id)
            .into_iter()
            .map(|x| x.name)
            .collect());
    Ok(json!(user))
}
