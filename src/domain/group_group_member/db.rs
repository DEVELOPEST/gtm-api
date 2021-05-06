use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};
use crate::domain::group_group_member::model::GroupRelation;
use crate::schema::group_group_members;

#[derive(Insertable)]
#[table_name = "group_group_members"]
struct NewGroupRelation {
    parent: i32,
    child: i32,
}

pub fn create(
    conn: &PgConnection,
    parent: i32,
    child: i32,
) -> GroupRelation {
    let new_group_relation = NewGroupRelation {
        parent,
        child,
    };

    diesel::insert_into(group_group_members::table)
        .values(&new_group_relation)
        .get_result::<GroupRelation>(conn)
        .expect("Error creating GroupRelation!")
}

pub fn find(
    conn: &PgConnection,
    parent: i32,
    child: i32,
) -> GroupRelation {
    group_group_members::table
        .filter(group_group_members::parent.eq(parent)
            .and(group_group_members::child.eq(child)))
        .get_result::<GroupRelation>(conn)
        .expect("Cannot load GroupRepository")
}

pub fn exists(conn: &PgConnection, parent: &i32, child: &i32) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(group_group_members::table
        .filter(group_group_members::parent.eq(parent)
            .and(group_group_members::child.eq(child)))))
        .get_result(conn)
        .expect("Error finding group_group_member")
}

pub fn find_all(
    conn: &PgConnection,
) -> Vec<GroupRelation> {
    group_group_members::table
        .load::<GroupRelation>(conn)
        .unwrap()
}
