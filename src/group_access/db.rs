use diesel::{Insertable, sql_query, sql_types};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::common::sql::GROUP_PARENTS_QUERY;
use crate::errors::Error;
use crate::group_access::dwh::GroupAccessCountDWH;
use crate::group_access::model::GroupAccess;
use crate::schema::group_accesses;

#[derive(Insertable)]
#[table_name = "group_accesses"]
pub struct NewGroupAccess<> {
    pub user: i32,
    pub group: i32,
    pub access_level_recursive: bool,
}

pub fn create(
    conn: &PgConnection,
    new_group_accesses: Vec<NewGroupAccess>,
) -> Result<usize, Error> {
    if new_group_accesses.len() > 0 {
        return Ok(diesel::insert_into(group_accesses::table)
            .values(new_group_accesses)
            .on_conflict_do_nothing()
            .execute(conn)?);
    }
    Ok(0)
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
) -> Result<GroupAccess, Error> {
    group_accesses::table
        .filter(group_accesses::user.eq(user)
            .and(group_accesses::group.eq(group)))
        .first::<GroupAccess>(conn)
        .map_err(Error::DatabaseError)
}

pub fn fetch_group_access_count(
    conn: &PgConnection,
    user: i32,
    group_name: &str,
) -> Result<i64, Error> {
    let count = sql_query(format!("
        {}
        SELECT sum(count)::bigint FROM (
            SELECT count(group_parents_query.parent) AS count
            FROM group_parents_query
            WHERE group_parents_query.parent IN (
                SELECT group_accesses.group
                FROM group_accesses
                WHERE group_accesses.access_level_recursive IS TRUE
                AND group_accesses.user = $2
            )
            UNION
            (
                SELECT 1 AS count
                FROM groups
                WHERE groups.name = $1
                AND groups.id IN (
                    SELECT group_accesses.group
                    FROM group_accesses
                    WHERE group_accesses.user = $2
                )
            )
        ) AS accesses_union", GROUP_PARENTS_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::Int4, _>(user)
        .get_result::<GroupAccessCountDWH>(conn)?;
    Ok(count.sum)
}

pub fn update(
    conn: &PgConnection,
    access: GroupAccess,
) -> Result<GroupAccess, Error> {
    let access = diesel::update(
        group_accesses::table.filter(group_accesses::user.eq(access.user)
            .and(group_accesses::group.eq(access.group))))
        .set(group_accesses::access_level_recursive.eq(!access.access_level_recursive.clone()))
        .get_result::<GroupAccess>(conn)?;
    Ok(access)
}