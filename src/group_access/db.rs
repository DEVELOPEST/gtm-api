use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};

use crate::group_access::model::GroupAccess;
use crate::schema::group_accesses;

#[derive(Insertable)]
#[table_name = "group_accesses"]
struct NewGroupAccess<> {
    user: i32,
    group: i32,
    access_level_recursive: bool,
}

pub fn create(
    conn: &PgConnection,
    user: i32,
    group: i32,
    access_level_recursive: bool,
) -> GroupAccess {
    let new_group_access = &NewGroupAccess {
        user,
        group,
        access_level_recursive,
    };

    let group_access = diesel::insert_into(group_accesses::table)
        .values(new_group_access)
        .get_result::<GroupAccess>(conn)
        .expect("Error creating group access");

    group_access
}

pub fn delete(
    conn: &PgConnection,
    user: i32,
    group: i32,
) -> Option<usize> {
    diesel::delete(group_accesses::table
        .filter(group_accesses::user.eq(user))
        .filter(group_accesses::group.eq(group)))
        .execute(conn)
        .ok()
}

pub fn find_by_user(
    conn: &PgConnection,
    user: i32
) -> Vec<GroupAccess> {
    group_accesses::table
        .filter(group_accesses::user.eq(user))
        .load::<GroupAccess>(conn)
        .expect("Cannot load accesses by user")
}

pub fn find_by_user_and_group(
    conn: &PgConnection,
    user: i32,
    group: i32,
) -> Option<GroupAccess> {
    group_accesses::table
        .filter(group_accesses::user.eq(user)
            .and(group_accesses::group.eq(group)))
        .first::<GroupAccess>(conn)
        .ok()
}

pub fn update(
    conn: &PgConnection,
    access: GroupAccess
) -> Option<GroupAccess> {
    diesel::update(
        group_accesses::table.filter(group_accesses::user.eq(access.user)
            .and(group_accesses::group.eq(access.group))))
        .set(group_accesses::access_level_recursive.eq(!access.access_level_recursive.clone()))
        .get_result::<GroupAccess>(conn)
        .ok()
}