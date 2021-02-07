use crate::db::Conn;
use crate::{group_access};
use crate::group_access::routes::{NewGroupAccess, DeleteGroupAccess, UserGroupAccess};
use crate::errors::{FieldValidator, Errors};
use rocket_contrib::json::JsonValue;
use crate::group_access::model::{GroupAccessJson};


pub fn add_group_accesses(
    conn: &Conn,
    group_accesses: Vec<NewGroupAccess>
) -> Result<JsonValue, Errors> {
    for group_access in group_accesses {
        let mut extractor = FieldValidator::validate(&group_access);
        let user = extractor.extract("user", group_access.user);
        let group = extractor.extract("group", group_access.group);
        let access_level_recursive = extractor.extract("access_level_recursive", group_access.access_level_recursive);
        extractor.check()?;
        group_access::db::create(&conn, user, group, access_level_recursive);
    }
    Ok(json!({}))
}

pub fn delete_group_accesses(
    conn: &Conn,
    group_accesses: Vec<DeleteGroupAccess>
) -> Result<JsonValue, Errors> {
    for group_access in group_accesses {
        let mut extractor = FieldValidator::validate(&group_access);
        let user = extractor.extract("user", group_access.user);
        let group = extractor.extract("group", group_access.group);
        extractor.check()?;
        group_access::db::delete(&conn, user, group);
    }
    Ok(json!({}))
}

pub fn find_by_user_and_group(conn: &Conn, user: i32, group: i32,) -> Option<GroupAccessJson> {
    let access = group_access::db::find_by_user_and_group(conn, user, group);
    match access {
        Some(x) => Some(x.attach()),
        None    => None,
    }
}

pub fn toggle_access(
    conn: &Conn,
    group_access: UserGroupAccess
) -> Result<JsonValue, Errors> {
    let mut extractor = FieldValidator::validate(&group_access);
    let user = extractor.extract("user", group_access.user);
    let group = extractor.extract("group", group_access.group);
    extractor.check()?;
    let access = group_access::db::find_by_user_and_group(&conn, user, group).unwrap();
    group_access::db::update(&conn, access);
    Ok(json!({}))
}
