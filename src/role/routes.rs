use rocket_contrib::json::Json;
use rocket_okapi::{JsonSchema, openapi};
use serde::Deserialize;
use validator::Validate;

use crate::db::Conn;
use crate::errors::{Error, FieldValidator,};
use crate::role;
use crate::role::model::ADMIN;
use crate::role::resource::UserRoleMemberJson;
use crate::user;
use crate::user::model::AuthUser;
use crate::user_role_member;

#[derive(Deserialize, Validate, JsonSchema)]
pub struct UserRoleMemberDto {
    #[validate(range(min = 1))]
    pub user: Option<i32>,
    #[validate(range(min = 1, max = 3))]
    pub role: Option<i32>,
}

#[openapi]
#[post("/roles", format = "json", data = "<user_role_data>")]
pub fn add_role_to_user(
    auth_user: AuthUser,
    conn: Conn,
    user_role_data: Json<UserRoleMemberDto>,
) -> Result<Json<UserRoleMemberJson>, Error> {
    auth_user.require_role(&ADMIN)?;
    let user_role_data = user_role_data.into_inner();
    let mut extractor = FieldValidator::validate(&user_role_data);
    let user = extractor.extract("user", user_role_data.user);
    let role = extractor.extract("role", user_role_data.role);
    extractor.check()?;

    if !user::db::exists(&conn, user) {
        return Err(Error::Custom("User does not exist!"));
    }

    if !role::db::exists(&conn, role) {
        return Err(Error::Custom("Role does not exist!"));
    }

    let res = user_role_member::db::create(&conn, user, role)?;

    Ok(Json(UserRoleMemberJson::from(res)))
}

#[openapi]
#[delete("/roles", format = "json", data = "<user_role_data>")]
pub fn delete_role_from_user(
    auth_user: AuthUser,
    conn: Conn,
    user_role_data: Json<UserRoleMemberDto>,
) -> Result<Json<bool>, Error> {
    auth_user.require_role(&ADMIN)?;
    let user_role_data = user_role_data.into_inner();
    let mut extractor = FieldValidator::validate(&user_role_data);
    let user = extractor.extract("user", user_role_data.user);
    let role = extractor.extract("role", user_role_data.role);
    extractor.check()?;

    if !user::db::exists(&conn, user) {
        return Err(Error::Custom("Cannot find user!"));
    }

    if !role::db::exists(&conn, role) {
        return Err(Error::Custom("Cannot find role!"));
    }

    user_role_member::db::delete(&conn, user, role);

    // TODO: Return something useful
    Ok(Json(true))
}