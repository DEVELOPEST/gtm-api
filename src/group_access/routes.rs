use serde::Deserialize;
use validator::Validate;
use rocket_contrib::json::{Json, JsonValue};

use crate::group_access;
use crate::errors::{Error};
use crate::db::Conn;
use crate::user::model::AuthUser;
use crate::role::model::ADMIN;

#[derive(Deserialize, Validate)]
pub struct NewGroupAccess {
    pub user: Option<i32>,
    pub group: Option<i32>,
    pub access_level_recursive: Option<bool>,
}

#[derive(Deserialize, Validate)]
pub struct DeleteGroupAccess {
    pub user: Option<i32>,
    pub group: Option<i32>,
}

#[derive(Deserialize, Validate)]
pub struct UserGroupAccess {
    pub user: Option<i32>,
    pub group: Option<i32>,
}

#[post("/group_accesses", format = "json", data = "<group_accesses>")]
pub fn post_group_accesses(
    auth_user: AuthUser,
    group_accesses: Json<Vec<NewGroupAccess>>,
    conn: Conn,
) -> Result<JsonValue, Error> {
    auth_user.require_role(&ADMIN)?;
    group_access::service::add_group_accesses(&conn, group_accesses.into_inner())?;
    Ok(json!({}))
}

#[delete("/group_accesses", format = "json", data = "<group_accesses>")]
pub fn delete_group_accesses(
    auth_user: AuthUser,
    group_accesses: Json<Vec<DeleteGroupAccess>>,
    conn: Conn,
) -> Result<JsonValue, Error> {
    auth_user.require_role(&ADMIN)?;
    group_access::service::delete_group_accesses(&conn, group_accesses.into_inner())?;
    Ok(json!({}))
}

#[put("/group_accesses/toggle", format = "json", data = "<group_access>")]
pub fn toggle_recursive_access(
    auth_user: AuthUser,
    group_access: Json<UserGroupAccess>,
    conn: Conn,
) -> Result<JsonValue, Error> {
    auth_user.require_role(&ADMIN)?;
    group_access::service::toggle_access(&conn, group_access.into_inner())?;
    Ok(json!({}))
}