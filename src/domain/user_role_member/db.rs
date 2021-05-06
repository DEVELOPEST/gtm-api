use crate::schema::user_role_members;
use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods, BoolExpressionMethods};
use crate::domain::user_role_member::model::UserRoleMember;
use crate::errors::Error;


#[derive(Insertable)]
#[table_name = "user_role_members"]
struct NewUserRoleMember {
    user: i32,
    role: i32,
}

pub fn create(
    conn: &PgConnection,
    user: i32,
    role: i32,
) -> Result<UserRoleMember, Error> {
    let new_group_relation = NewUserRoleMember {
        user,
        role,
    };

    let res = diesel::insert_into(user_role_members::table)
        .values(&new_group_relation)
        .get_result::<UserRoleMember>(conn)?;
    Ok(res)
}

pub fn delete(
    conn: &PgConnection,
    user_id: i32,
    role_id: i32,
) -> Option<usize> {
    diesel::delete(user_role_members::table
        .filter(user_role_members::user.eq(user_id))
        .filter(user_role_members::role.eq(role_id)))
        .execute(conn)
        .ok()
}

pub fn find(
    conn: &PgConnection,
    user: i32,
    role: i32,
) -> UserRoleMember {
    user_role_members::table
        .filter(user_role_members::user.eq(user)
            .and(user_role_members::role.eq(role)))
        .get_result::<UserRoleMember>(conn)
        .expect("Cannot load GroupRepository")
}

pub fn find_by_user(
    conn: &PgConnection,
    user: i32,
) -> Vec<UserRoleMember> {
    user_role_members::table
        .filter(user_role_members::user.eq(user))
        .load(conn)
        .expect("Cannot load UserRoleMember")
}