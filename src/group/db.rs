use diesel;
use diesel::Insertable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::group::model::Group;
use crate::schema::groups;


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

pub fn find(conn: &PgConnection, name: &str) -> Option<Group> {
    groups::table
        .filter(groups::name.eq(name))
        .first::<Group>(conn)
        .ok()
}

pub fn find_all(conn: &PgConnection) -> Vec<Group> {
    groups::table
        .load::<Group>(conn)
        .expect("Unable to load groups")
}
