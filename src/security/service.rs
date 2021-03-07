use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::PgConnection;
use rocket_oauth2::TokenResponse;

use crate::{security, user};
use crate::errors::Errors;
use crate::security::jwt::get_auth_user_from_token;
use crate::security::oauth::{IdentityUser, LoginType};
use crate::user::db::UserCreationError;
use crate::user::model::User;
use crate::user_role_member;

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

pub fn password_login(
    conn: &PgConnection,
    username: String,
    password: String,
) -> Result<String, Errors> {
    let user = user::db::find_by_username(&conn, &username);

    if user.is_none() {
        return Err(Errors::new(&[("username", "Cannot find user with username")], None));
    }

    let user = user.unwrap();
    match user.password.clone() {
        None => { return Err(Errors::new(&[("password", "Password authentication not enabled for user!")], None)); }
        Some(pass) => {
            if !scrypt_check(&password, &pass).unwrap() {
                return Err(Errors::new(&[("password", "Wrong password!")], None));
            }
        }
    }

    let jwt = security::jwt::generate_token_for_user(&conn, user);
    match jwt {
        None => { Err(Errors::new(&[("jwt", "Error generating jwt for user")], None)) }
        Some(token) => { Ok(token) }
    }
}

pub fn change_password(
    conn: &PgConnection,
    user_id: i32,
    old_password: String,
    new_password: String) -> Result<(), Errors> {
    let user = user::db::find(&conn, user_id).unwrap();

    if user.password.is_some() {
        if !scrypt_check(&old_password, &user.password.unwrap()).unwrap() {
            return Err(Errors::new(&[("password", "Wrong password!")], None));
        }
    }

    user::db::update_password(&conn, user.id, &crypt_password(&new_password).to_string());
    Ok(())
}

fn crypt_password(password: &str) -> String {
    scrypt_simple(password, &ScryptParams::new(10, 8, 1)).expect("hash error")
}

pub async fn oauth_register<T>(conn: &PgConnection, token: TokenResponse<T>, jwt: &str) -> Result<(), reqwest::Error>
    where TokenResponse<T>: LoginType
{
    if let Some(auth_user) = get_auth_user_from_token(conn, jwt) {
        let user = security::oauth::fetch_github_user(token.access_token()).await?;
        if security::db::exists_oauth_login(conn, auth_user.user_id, token.get_login_type()) {
            security::db::update_oauth_login(
                conn,
                auth_user.user_id,
                token.get_login_type(),
                user.get_identity_hash(),
                token.access_token(),
                token.refresh_token(),
                token.expires_in());
        } else {
            security::db::create_oauth_login(
                conn,
                auth_user.user_id,
                token.get_login_type(),
                user.get_identity_hash(),
                token.access_token(),
                token.refresh_token(),
                token.expires_in());
        }
    }
    Ok(())
}

pub async fn oauth_login<T>(conn: &PgConnection, token: TokenResponse<T>) -> Option<String>
    where TokenResponse<T>: LoginType
{
    let identity_hash = token.fetch_identity_hash().await.ok()?;
    if let Some(user) = security::db::find_user_for_oath_login(conn, &identity_hash, token.get_login_type()) {
        return security::jwt::generate_token_for_user(conn, user);
    }
    None
}