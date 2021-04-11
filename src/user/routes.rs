use rocket_contrib::json::Json;

use crate::db::Conn;
use crate::errors::Error;
use crate::role;
use crate::role::model::ADMIN;
use crate::user;
use crate::user::model::{AuthUser, UserJson};
use crate::user::resource::UserIdResponse;

#[openapi]
#[get("/user")]
pub fn get_user_id(user: AuthUser, conn: Conn) -> Result<Json<UserIdResponse>, Error> {
    user::db::find(&conn, user.user_id)
        .map(|user| Json(UserIdResponse { user_id: user.id }))
}

#[openapi]
#[get("/users")]
pub fn get_users(auth_user: AuthUser, conn: Conn) -> Result<Json<Vec<UserJson>>, Error> {
    auth_user.require_role(&ADMIN)?;
    Ok(Json(user::service::find_all(&conn)))
}

#[openapi]
#[get("/users/<id>")]
pub fn get_user(auth_user: AuthUser, id: i32, conn: Conn) -> Result<Json<UserJson>, Error> {
    auth_user.require_role(&ADMIN)?;
    let user = user::db::find(&conn, id)
        .unwrap()
        .attach(role::db::find_all_by_user(&conn, id)
            .into_iter()
            .map(|x| x.name)
            .collect());
    Ok(Json(user))
}
