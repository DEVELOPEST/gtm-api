use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;
use crate::db::Conn;
use crate::errors::{Errors, FieldValidator};
use crate::group;
use crate::group::service;

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

#[get("/groups")]
pub fn get_groups(conn: Conn) -> JsonValue {
    let groups = group::db::find_all(&conn);
    json!({"groups": groups})
}

#[get("/groups/<group_name>/stats")]
pub fn get_group_stats(conn: Conn, group_name: String) -> Result<JsonValue, Errors> {
    // TODO: Start and end
    Ok(json!(service::get_group_repos(&conn, &group_name, 0, 9223372036854775807)))
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
