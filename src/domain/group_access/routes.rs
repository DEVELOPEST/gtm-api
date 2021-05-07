use serde::Deserialize;
use validator::Validate;
use rocket_contrib::json::{Json};
use rocket_okapi::{JsonSchema, openapi};

use crate::domain::group_access;
use crate::errors::{Error};
use crate::domain::db::Conn;
use crate::domain::user::model::AuthUser;
use crate::domain::role::model::ADMIN;
use crate::domain::group_access::resource::GroupAccessJson;

#[derive(Deserialize, Validate, JsonSchema)]
pub struct NewGroupAccess {
    pub user: Option<i32>,
    pub group: Option<i32>,
    pub access_level_recursive: Option<bool>,
}

#[derive(Deserialize, Validate, JsonSchema)]
pub struct DeleteGroupAccess {
    pub user: Option<i32>,
    pub group: Option<i32>,
}

#[derive(Deserialize, Validate, JsonSchema)]
pub struct UserGroupAccess {
    pub user: Option<i32>,
    pub group: Option<i32>,
}

#[openapi]
#[post("/group_accesses", format = "json", data = "<group_accesses>")]
pub fn post_group_accesses(
    auth_user: AuthUser,
    group_accesses: Json<Vec<NewGroupAccess>>,
    conn: Conn,
) -> Result<Json<bool>, Error> {
    auth_user.require_role(&ADMIN)?;
    group_access::service::add_group_accesses(&conn, group_accesses.into_inner())?;
    // TODO: Return something useful?
    Ok(Json(true))
}

#[openapi]
#[delete("/group_accesses", format = "json", data = "<group_accesses>")]
pub fn delete_group_accesses(
    auth_user: AuthUser,
    group_accesses: Json<Vec<DeleteGroupAccess>>,
    conn: Conn,
) -> Result<Json<usize>, Error> {
    auth_user.require_role(&ADMIN)?;
    let res = group_access::service::delete_group_accesses(&conn, group_accesses.into_inner())?;
    Ok(Json(res))
}

#[openapi]
#[put("/group_accesses/toggle", format = "json", data = "<group_access>")]
pub fn toggle_recursive_access(
    auth_user: AuthUser,
    group_access: Json<UserGroupAccess>,
    conn: Conn,
) -> Result<Json<GroupAccessJson>, Error> {
    auth_user.require_role(&ADMIN)?;
    let access = group_access::service::toggle_access(&conn, group_access.into_inner())?;
    Ok(Json(access.attach()))
}