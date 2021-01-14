use crate::models::group::{Group};
use crate::schema::{groups};
use crate::schema::group_group_members;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable, sql_query, sql_types};
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "groups"]
struct NewGroup<'a> {
    name: &'a str,
}

pub fn create(
    conn: &PgConnection,
    name: &str,
) -> Group {
    let new_group = &NewGroup {
        name,
    };

    let group = diesel::insert_into(groups::table)
        .values(new_group)
        .get_result::<Group>(conn)
        .expect("Error creating  group");

    group
}


pub fn exists(conn: &PgConnection, name: &str) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(groups::table
        .filter(groups::name.eq(name))))
        .get_result(conn)
        .expect("Error finding  group")
}

pub fn find(conn: &PgConnection, name: &str) -> Group {
    groups::table
        .filter(groups::name.eq(name))
        .get_result::<Group>(conn)
        .expect("Cannot load repository")
}

#[derive(QueryableByName, Serialize)]
#[table_name = "group_group_members"]
pub struct Test {
    pub parent: i32
}

pub fn find_all_repositories(conn: &PgConnection, name: &str) -> Vec<Test> {
    sql_query("
    WITH RECURSIVE q AS
        (
        SELECT  group_group_members.*, 0 AS depth
        FROM    group_group_members
        WHERE   group_group_members.parent = (
            SELECT groups.id
            FROM groups
            WHERE groups.name = $1)
        UNION ALL
        SELECT  m.*, q.depth + 1
        FROM    group_group_members m
        JOIN    q
        ON      m.parent = q.child
        WHERE   q.depth < 100
        )
    SELECT  *
    FROM    q")
        .bind::<sql_types::Text, _>(name)
        .load(conn)
        .expect("Error finding repositories for group")

}