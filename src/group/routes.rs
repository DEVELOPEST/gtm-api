use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;
use crate::db::Conn;
use crate::errors::{Errors, FieldValidator};
use crate::group_group_member;
use crate::group;

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

    let mut relation_child = group::db::find(&conn, &group_name);
    if relation_child.is_none() {
        relation_child = Some(group::db::create(&conn, &group_name));
    }
    let relation_child = relation_child.unwrap();

    for parent in &parents_vec {
        let relation_parent = if !group::db::exists(&conn, &parent) {
            group::db::create(&conn, &parent)
        } else {
            group::db::find(&conn, &parent).unwrap()
        };
        if !group_group_member::db::exists(&conn, &relation_parent.id, &relation_child.id) {
            group_group_member::db::create(&conn, relation_parent.id, relation_child.id);
        }
    }

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

    let mut relation_parent = group::db::find(&conn, &group_name);
    if relation_parent.is_none() {
        relation_parent = Some(group::db::create(&conn, &group_name));
    }
    let relation_parent = relation_parent.unwrap();

    for child in &children_vec {
        let relation_child = if !group::db::exists(&conn, &child) {
            group::db::create(&conn, &child)
        } else {
            group::db::find(&conn, &child).unwrap()
        };
        if !group_group_member::db::exists(&conn, &relation_parent.id, &relation_child.id) {
            group_group_member::db::create(&conn, relation_parent.id, relation_child.id);
        }
    }

    // TODO return something useful
    Ok(json!({}))
}
