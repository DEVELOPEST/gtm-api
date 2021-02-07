use diesel::PgConnection;
use crate::user;
use crate::user::db::UserCreationError;
use crate::user_role_member;
use crate::user::model::User;
use crypto::scrypt::{scrypt_simple, ScryptParams, scrypt_check};
use crate::errors::Errors;

pub fn new_user(
    conn: &PgConnection,
    email: &str,
    password: &str,
) -> Result<User, UserCreationError> {
    let hash = crypt_password(password);
    let user_result = user::db::create(&conn, &email, &hash)?;
    user_role_member::db::create(conn, user_result.id.clone(), 1);
    Ok(user_result)
}

pub fn change_password(
    conn: &PgConnection,
    user_id: i32,
    old_password: String,
    new_password: String) -> Result<(), Errors> {
    let user = user::db::find(&conn, user_id).unwrap();

    if !scrypt_check(&old_password, &user.password).unwrap() {
        return Err(Errors::new(&[("password", "Wrong password!")], None));
    }
    user::db::update_password(&conn, user.id, &crypt_password(&new_password).to_string());
    Ok(())
}

fn crypt_password(password: &str) -> String {
    scrypt_simple(password, &ScryptParams::new(10, 8, 1)).expect("hash error")
}