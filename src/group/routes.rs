use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

use crate::db::Conn;
use crate::errors::{FieldValidator, Error};
use crate::{group, security};
use crate::group::model::{GroupJson, GroupWithAccessJson};
use crate::group::service;
use crate::group_access;
use crate::role::model::ADMIN;
use crate::user::model::AuthUser;

#[derive(Deserialize, Validate)]
pub struct NewGroupParentsRelation {
    #[validate(length(min = 1))]
    parents: Option<Vec<String>>,
}

#[derive(Deserialize, Validate)]
pub struct NewGroupChildrenRelation {
    #[validate(length(min = 1))]
    children: Option<Vec<String>>,
}

#[derive(FromForm, Default, Validate, Deserialize)]
pub struct GroupStatsParams {
    start: Option<i64>,
    end: Option<i64>,
    depth: Option<i32>,
}

#[get("/groups")]
pub fn get_groups(auth_user: AuthUser, conn: Conn) -> Result<JsonValue, Error> {
    let groups: Vec<GroupJson> = if auth_user.roles.contains(&ADMIN) {
        group::db::find_all(&conn)?.into_iter().map(|x| x.attach()).collect()
    } else {
        group::service::get_groups_with_access(&conn, auth_user.user_id)?
            .into_iter().map(|x| x.attach()).collect()
    };
    Ok(json!(groups))
}

#[get("/groups/accessible/user/<user_id>")]
pub fn get_groups_with_access(auth_user: AuthUser, conn: Conn, user_id: i32) -> Result<JsonValue, Error> {
    auth_user.require_role(&ADMIN)?;
    let groups: Vec<GroupWithAccessJson> = group::service::get_groups_with_access(&conn, user_id)?
        .into_iter()
        .map(|x| {
            let group_id = x.id.clone();
            x.attach_with_access(
                group_access::service::find_by_user_and_group(&conn, user_id, group_id).ok()
            )
        })
        .collect();
    Ok(json!(groups))
}

#[get("/groups/not-accessible/user/<user_id>")]
pub fn get_groups_without_access(auth_user: AuthUser, conn: Conn, user_id: i32) -> Result<JsonValue, Error> {
    auth_user.require_role(&ADMIN)?;
    let groups: Vec<GroupJson> = group::service::get_groups_without_access(&conn, user_id)?
        .into_iter().map(|x| x.attach()).collect();
    Ok(json!(groups))
}

#[get("/groups/<group_name>/stats?<params..>")]
pub fn get_group_stats(
    auth_user: AuthUser,
    conn: Conn,
    group_name: String,
    params: Form<GroupStatsParams>,
) -> Result<JsonValue, Error> {
    if auth_user.require_role(&ADMIN).is_err() {
        security::service::check_group_access(&conn, auth_user.user_id, &group_name)?;
    }
    let period = params.into_inner();
    let start = period.start.unwrap_or(0);
    let end = period.end.unwrap_or(std::i64::MAX);
    let depth = period.depth.unwrap_or(1);
    let stats = service::get_group_stats(&conn, &group_name, start, end, depth)?;
    Ok(json!(stats))
}

#[get("/groups/<group_name>/export?<params..>")]
pub fn get_group_export(
    auth_user: AuthUser,
    conn: Conn,
    group_name: String,
    params: Form<GroupStatsParams>,
) -> Result<JsonValue, Error> {
    if auth_user.require_role(&ADMIN).is_err() {
        security::service::check_group_access(&conn, auth_user.user_id, &group_name)?;
    }
    let period = params.into_inner();
    let start = period.start.unwrap_or(0);
    let end = period.end.unwrap_or(std::i64::MAX);
    let depth = period.depth.unwrap_or(1);
    let data = service::export_group_data(&conn, &group_name, start, end, depth)?;
    Ok(json!(data))
}

#[post("/groups/<group_name>/parents", format = "json", data = "<parents>")]
pub fn post_group_parents(
    //auth: Auth,
    group_name: String,
    parents: Json<NewGroupParentsRelation>,
    conn: Conn,
) -> Result<JsonValue, Error> {
    let parents = parents.into_inner();
    let mut extractor = FieldValidator::validate(&parents);
    let parents_vec = extractor.extract("parents", parents.parents);
    extractor.check()?;

    service::add_group_relations(&conn, parents_vec, vec![group_name])?;
    // TODO return something useful
    Ok(json!({}))
}

#[post("/groups/<group_name>/children", format = "json", data = "<children>")]
pub fn post_group_children(
    //auth: Auth,
    group_name: String,
    children: Json<NewGroupChildrenRelation>,
    conn: Conn,
) -> Result<JsonValue, Error> {
    let children = children.into_inner();
    let mut extractor = FieldValidator::validate(&children);
    let children_vec = extractor.extract("children", children.children);
    extractor.check()?;

    service::add_group_relations(&conn, vec![group_name], children_vec)?;
    // TODO return something useful
    Ok(json!({}))
}
