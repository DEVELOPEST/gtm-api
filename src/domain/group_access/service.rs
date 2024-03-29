use diesel::PgConnection;
use rocket_contrib::json::JsonValue;

use crate::common::git;
use crate::common::git::RepoCredentials;
use crate::domain::db::Conn;
use crate::domain::group;
use crate::domain::group_access;
use crate::domain::group_access::db;
use crate::domain::group_access::model::GroupAccess;
use crate::domain::group_access::resource::GroupAccessJson;
use crate::domain::group_access::routes::{DeleteGroupAccess, NewGroupAccess, UserGroupAccess};
use crate::errors::{Error, FieldValidator};

pub fn add_group_accesses(
    conn: &PgConnection,
    group_accesses: Vec<NewGroupAccess>,
) -> Result<JsonValue, Error> {
    let mut extractor = FieldValidator::default();
    let new_group_accesses = group_accesses.into_iter()
        .filter_map(|group_access| {
            let user = extractor.extract("user", group_access.user);
            let group = extractor.extract("group", group_access.group);
            let access_level_recursive = extractor.extract("access_level_recursive", group_access.access_level_recursive);
            if group_access.user.is_none() ||
                group_access.group.is_none() ||
                group_access.access_level_recursive.is_none() {
                return None;
            }

            Some(db::NewGroupAccess {
                user,
                group,
                access_level_recursive,
            })
        }).collect();
    group_access::db::create(&conn, new_group_accesses)?;
    extractor.check()?;
    Ok(json!({}))
}

pub fn create_group_accesses_for_user(
    conn: &PgConnection,
    repos: Vec<RepoCredentials>,
    user_id: i32,
) -> Result<(), Error> {
    let groups = group::db::find_all(conn)?;
    let group_accesses: Vec<NewGroupAccess> = repos.into_iter()
        .filter_map(|r| Some(NewGroupAccess {
            user: Option::from(user_id),
            group: Option::from(groups.iter()
                .find(|&g| g.name == git::generate_group_name(&r.provider, &r.user, &r.repo))?.id),
            access_level_recursive: Option::from(false),
        }))
        .collect();
    add_group_accesses(conn, group_accesses)?;
    Ok(())
}

pub fn delete_group_accesses(
    conn: &Conn,
    group_accesses: Vec<DeleteGroupAccess>,
) -> Result<usize, Error> {
    let mut count = 0;
    for group_access in group_accesses {
        let mut extractor = FieldValidator::validate(&group_access);
        let user = extractor.extract("user", group_access.user);
        let group = extractor.extract("group", group_access.group);
        extractor.check()?;
        count += group_access::db::delete(conn, user, group).unwrap_or(0);
    }
    Ok(count)
}

pub fn find_by_user_and_group(conn: &Conn, user: i32, group: i32,) -> Result<GroupAccessJson, Error> {
   group_access::db::find_by_user_and_group(conn, user, group)
        .map(|x| x.attach())
}

pub fn toggle_access(
    conn: &Conn,
    group_access: UserGroupAccess
) -> Result<GroupAccess, Error> {
    let mut extractor = FieldValidator::validate(&group_access);
    let user = extractor.extract("user", group_access.user);
    let group = extractor.extract("group", group_access.group);
    extractor.check()?;
    let access = group_access::db::find_by_user_and_group(&conn, user, group)?;
    group_access::db::update(&conn, access)
}

pub fn get_group_access_count(conn: &PgConnection, user: i32, group: &str) -> Result<i64, Error>{
    group_access::db::fetch_group_access_count(conn, user, group)
}
