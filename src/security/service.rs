use diesel::PgConnection;
use crate::user;
use crate::user::db::UserCreationError;
use crate::user_role_member;
use crate::user::model::User;

pub fn new_user(
    conn: &PgConnection,
    email: &str,
    password: &str,
) -> Result<User, UserCreationError> {
    let user_result = user::db::create(&conn, &email, &password)?;
    user_role_member::db::create(conn, user_result.id.clone(), 1);
    Ok(user_result)
}