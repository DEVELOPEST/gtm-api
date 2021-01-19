use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

// use crate::auth::Auth;
use crate::db;
use crate::errors::{Errors, FieldValidator};

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

#[post("/groups/<group_name>/parents", format = "json", data = "<parents>")]
pub fn post_group_parents(
    //auth: Auth,
    group_name: String,
    parents: Json<NewGroupParentsRelation>,
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let parents = parents.into_inner();
    let mut extractor = FieldValidator::validate(&parents);
    let parents_vec = extractor.extract("parents", parents.parents);
    extractor.check()?;

    let relation_child = db::groups::find(&conn, &group_name);
    for parent in &parents_vec {
        if !db::groups::exists(&conn, &parent) {
            db::groups::create(&conn, &parent);
        }
        let relation_parent = db::groups::find(&conn, &parent);
        if !db::group_relations::exists(&conn, &relation_parent.id, &relation_child.id) {
            db::group_relations::create(&conn, relation_parent.id, relation_child.id);
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
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let children = children.into_inner();
    let mut extractor = FieldValidator::validate(&children);
    let children_vec = extractor.extract("children", children.children);
    extractor.check()?;

    let relation_parent = db::groups::find(&conn, &group_name);
    // TODO(Tavo): See if this can be moved to separate function
    for child in &children_vec {
        if !db::groups::exists(&conn, &child) {
            db::groups::create(&conn, &child);
        }
        let relation_child = db::groups::find(&conn, &child);
        if !db::group_relations::exists(&conn, &relation_parent.id, &relation_child.id) {
            db::group_relations::create(&conn, relation_parent.id, relation_child.id);
        }
    }

    // TODO return something useful
    Ok(json!({}))
}