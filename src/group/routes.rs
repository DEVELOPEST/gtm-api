use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

use crate::db::Conn;
use crate::errors::{Errors, FieldValidator};
use crate::group;
use crate::user::model::AuthUser;
use crate::group::service;
use crate::group::model::GroupJson;

// use crate::auth::Auth;


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
}

#[get("/groups")]
pub fn get_groups(auth_user: AuthUser, conn: Conn) -> JsonValue {
    let groups = group::db::find_all(&conn);
    json!({"groups": groups})
}

#[get("/test")]
pub fn get_groups2( conn: Conn) -> JsonValue {
    let groups: Vec<GroupJson> = group::db::fetch_group_children(&conn, 4).into_iter().map(|x| x.attach()).collect();
    json!({"groups": groups})
}

#[get("/groups/<group_name>/stats?<params..>")]
pub fn get_group_stats(conn: Conn, group_name: String, params: Form<GroupStatsParams>) -> Result<JsonValue, Errors> {
    let period = params.into_inner();
    let start = period.start.unwrap_or(0);
    let end = period.end.unwrap_or(std::i64::MAX);
    Ok(json!(service::get_group_repos(&conn, &group_name, start, end)?))
}

#[post("/groups/<group_name>/parents", format = "json", data = "<parents>")]
pub fn post_group_parents(
    //auth: Auth,
    group_name: String,
    parents: Json<NewGroupParentsRelation>,
    conn: Conn,
) -> Result<JsonValue, Errors> {
    let parents = parents.into_inner();
    let mut extractor = FieldValidator::validate(&parents);
    let parents_vec = extractor.extract("parents", parents.parents);
    extractor.check()?;

    service::add_group_relations(&conn, parents_vec, vec![group_name]);
    // TODO return something useful
    Ok(json!({}))
}

#[post("/groups/<group_name>/children", format = "json", data = "<children>")]
pub fn post_group_children(
    //auth: Auth,
    group_name: String,
    children: Json<NewGroupChildrenRelation>,
    conn: Conn,
) -> Result<JsonValue, Errors> {
    let children = children.into_inner();
    let mut extractor = FieldValidator::validate(&children);
    let children_vec = extractor.extract("children", children.children);
    extractor.check()?;

    service::add_group_relations(&conn, vec![group_name], children_vec);
    // TODO return something useful
    Ok(json!({}))
}
