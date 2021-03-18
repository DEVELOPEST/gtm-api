use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::PgConnection;
use rocket_oauth2::TokenResponse;

use crate::{common, security, user};
use crate::email;
use crate::errors::{Error};
use crate::group_access;
use crate::security::oauth::LoginType;
use crate::user::db::UserCreationError;
use crate::user::model::User;
use crate::user_role_member;

pub fn new_user(
    conn: &PgConnection,
    username: &str,
    password: Option<String>,
) -> Result<User, UserCreationError> {
    let hash = password.map(|pass| crypt_password(&pass));
    let user_result = user::db::create(&conn, &username, hash)?;
    user_role_member::db::create(conn, user_result.id.clone(), 1);
    Ok(user_result)
}

pub fn password_login(
    conn: &PgConnection,
    username: String,
    password: String,
) -> Result<String, Error> {
    let user = user::db::find_by_username(&conn, &username);

    if user.is_none() {
        return Err(Error::AuthorizationError("Invalid username!".to_string()));
    }

    let user = user.unwrap();
    match user.password.clone() {
        None => { return Err(Error::AuthorizationError("Password authentication not enabled!".to_string())); }
        Some(pass) => {
            if !scrypt_check(&password, &pass).unwrap() {
                return Err(Error::AuthorizationError("Invalid password!".to_string()));
            }
        }
    }

    let jwt = security::jwt::generate_token_for_user(&conn, user);
    match jwt {
        None => { Err(Error::Custom("Error generating jwt!".to_string())) }
        Some(token) => { Ok(token) }
    }
}

pub fn change_password(
    conn: &PgConnection,
    user_id: i32,
    old_password: String,
    new_password: String
) -> Result<(), Error> {
    let user = user::db::find(&conn, user_id).unwrap();

    if user.password.is_some() {
        if !scrypt_check(&old_password, &user.password.unwrap()).unwrap() {
            return Err(Error::AuthorizationError("Bad password!".to_string()));
        }
    }

    user::db::update_password(&conn, user.id, &crypt_password(&new_password).to_string());
    Ok(())
}

pub fn create_password(
    conn: &PgConnection,
    user_id: i32,
    new_password: String
) -> Result<(), Error> {
    let user = user::db::find(&conn, user_id).unwrap();

    user::db::update_password(&conn, user.id, &crypt_password(&new_password).to_string());
    Ok(())
}

fn crypt_password(password: &str) -> String {
    scrypt_simple(password, &ScryptParams::new(10, 8, 1)).expect("hash error")
}

pub async fn oauth_register<T>(conn: &PgConnection, token: &TokenResponse<T>, user_id: i32) -> Result<(), reqwest::Error>
    where TokenResponse<T>: LoginType
{
    let identity_hash = token.fetch_identity_hash().await
        .map_err(|e| {
            error!("Error fetching identity hash for user: {}", e);
            e
        })?;
    if security::db::exists_oauth_login(conn, user_id, token.get_login_type()) {
        security::db::update_oauth_login(
            conn,
            user_id,
            token.get_login_type(),
            &identity_hash,
            token.access_token(),
            token.refresh_token(),
            token.expires_in());
    } else {
        security::db::create_oauth_login(
            conn,
            user_id,
            token.get_login_type(),
            &identity_hash,
            token.access_token(),
            token.refresh_token(),
            token.expires_in());
    }
    let emails = token.fetch_emails().await.map_err(|e| {
        error!("Error fetching emails for user: {}", e);
        e
    })?;
    email::service::create_emails_for_user(conn, user_id, emails.iter().map(|x| &**x).collect());
    give_group_accesses(conn, token, user_id).await?;
    Ok(())
}

pub async fn oauth_login<T>(conn: &PgConnection, token: &TokenResponse<T>) -> Option<String>
    where TokenResponse<T>: LoginType
{
    let identity_hash = token.fetch_identity_hash().await.ok()?;
    if let Some(user) = security::db::find_user_for_oath_login(conn, &identity_hash, token.get_login_type()) {
        security::db::update_oauth_login(
            conn,
            user.id,
            token.get_login_type(),
            &identity_hash,
            token.access_token(),
            token.refresh_token(),
            token.expires_in());
        let emails = token.fetch_emails().await.ok()?;
        email::service::create_emails_for_user(conn, user.id, emails.iter().map(|x| &**x).collect());
        give_group_accesses(conn, token, user.id).await
            .map_err(|e| error!("Error giving group accesses: {}", e))
            .unwrap();
        return security::jwt::generate_token_for_user(conn, user);
    }
    None
}

pub async fn login_and_register<T>(conn: &PgConnection, token: TokenResponse<T>) -> String
    where TokenResponse<T>: LoginType
{
    let mut username = token.fetch_username().await
        .map_err(|e| error!("Error fetching username: {}", e))
        .unwrap_or("Anonymous User".to_string());
    if user::db::find_by_username(conn, &username).is_some() {
        username = format!("{}{}", username, common::random::random_string(5))
    }
    let user = new_user(&conn, &username, None)
        .map_err(|e| error!("Error creating user: {}", e))
        .unwrap();
    oauth_register(&conn, &token, user.id).await
        .map_err(|e| error!("Error registering oauth for user: {}", e))
        .unwrap();
    security::jwt::generate_token_for_user(&conn, user).unwrap()
}

async fn give_group_accesses<T>(
    conn: &PgConnection,
    token: &TokenResponse<T>,
    user_id: i32,
) -> Result<(), reqwest::Error>
    where TokenResponse<T>: LoginType
{
    let repos = token.fetch_accessible_repositories().await.map_err(|e| {
        error!("Error fetching accessible repositories for user: {}", e);
        e
    })?;
    Ok(group_access::service::create_group_accesses_for_user(conn, repos, user_id).unwrap())
}