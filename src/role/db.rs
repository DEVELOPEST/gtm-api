use crate::schema::roles;
use diesel::{PgConnection, QueryDsl, ExpressionMethods};
use crate::role::model::Role;
use crate::user_role_member;
use crate::diesel::RunQueryDsl;


pub fn find(conn: &PgConnection, id: i32) -> Option<Role> {
    roles::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_role: {}", err))
        .ok()
}

pub fn find_all_by_user(conn: &PgConnection, user_id: i32) -> Vec<Role> {
    let mut vec = Vec::new();
    let user_role_members = user_role_member::db::find_by_user(conn, user_id);

    for var in user_role_members {
        vec.push(find(conn, var.role).unwrap());
    }
    vec
}

pub fn exists(conn: &PgConnection, id: i32) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(roles::table
        .filter(roles::id.eq(id))))
        .get_result(conn)
        .expect("Error finding role")
}