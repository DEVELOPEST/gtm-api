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
) {
    diesel::delete(group_accesses::table
        .filter(group_accesses::user.eq(user))
        .filter(group_accesses::group.eq(group)))
        .execute(conn);
}