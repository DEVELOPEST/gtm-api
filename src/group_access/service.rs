use crate::db::Conn;
use crate::{group_access};
use crate::group_access::routes::{NewGroupAccess, DeleteGroupAccess};
use crate::errors::{FieldValidator, Errors};
use rocket_contrib::json::JsonValue;


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