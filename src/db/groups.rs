use crate::models::group::{Group};
use crate::schema::groups;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};

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