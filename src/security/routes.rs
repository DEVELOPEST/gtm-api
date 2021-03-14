use rocket::http::{Cookies, Status};
use rocket::response::Redirect;
use rocket_contrib::json::{Json, JsonValue};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::Deserialize;
use validator::Validate;

use crate::db::Conn;
use crate::errors::{Errors, FieldValidator};
use crate::security;
use crate::security::oauth::{GitHub, GitLab, LoginType, Microsoft, GitLabTalTech, Bitbucket};
use crate::security::service;
use crate::user;
use crate::user::db::UserCreationError;
use crate::user::model::AuthUser;

#[derive(Deserialize, Validate)]
pub struct LoginDto {
    #[validate(length(min = 1))]
    pub username: Option<String>,
    #[validate(length(min = 1))]
    pub password: Option<String>,
}

#[post("/auth/login", format = "json", data = "<login_data>")]
pub fn login(conn: Conn, login_data: Json<LoginDto>) -> Result<JsonValue, Errors> {
    let login_data = login_data.into_inner();
    let mut extractor = FieldValidator::validate(&login_data);
    let username = extractor.extract("username", login_data.username);
    let password = extractor.extract("password", login_data.password);
    extractor.check()?;

    let token = service::password_login(&conn, username, password)?;
    Ok(json!({"jwt": token}))
}

#[get("/auth/login", format = "json")]
pub fn get_user_logins(
    auth_user: AuthUser,
    conn: Conn,
) -> Result<JsonValue, Errors> {

    let logins = security::db::find_all_logins_by_user(&conn, auth_user.user_id);
    Ok(json!(logins))
}

#[delete("/auth/login", format = "json", data = "<login_type>")]
pub fn delete_user_login(
    auth_user: AuthUser,
    conn: Conn,
    login_type: Json<Type>,
) -> Result<JsonValue, Errors> {

    let login_type = login_type.into_inner();

    let mut extractor = FieldValidator::validate(&login_type);
    let login_type_string = extractor.extract("login_type", login_type.login_type);

    security::db::delete_login_by_user_and_type(&conn, auth_user.user_id, &login_type_string)
        .map_err(|err| error!("Error deleting login: {}", err))
        .unwrap();
    Ok(json!({}))
}

#[delete("/auth/account")]
pub fn delete_account(
    auth_user: AuthUser,
    conn: Conn,
) -> Result<JsonValue, Errors> {
    security::db::delete_account(&conn, auth_user.user_id)
        .map_err(|err| error!("Error deleting account: {}", err))
        .unwrap();
    Ok(json!({}))
}

#[get("/auth/password")]
pub fn has_password(
    auth_user: AuthUser,
    conn: Conn,
) -> Result<JsonValue, Errors> {
    let has_password = security::db::exists_password(&conn, auth_user.user_id);
    println!("{}", has_password);
    Ok(json!(has_password))
}

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}

#[derive(Deserialize, Validate)]
pub struct Type {
    login_type: Option<String>,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(length(min = 1))]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
}

#[post("/auth/register", format = "json", data = "<new_user>")]
pub fn register(
    new_user: Json<NewUser>,
    conn: Conn,
) -> Result<JsonValue, Errors> {

    let new_user = new_user.into_inner().user;

    let mut extractor = FieldValidator::validate(&new_user);
    let email = extractor.extract("email", new_user.email);
    let password = extractor.extract("password", new_user.password);

    extractor.check()?;

    let created_user = security::service::new_user(&conn, &email, Option::from(password))
        .map_err(|error| {
            let field = match error {
                UserCreationError::DuplicatedUsername => "username",
            };
            Errors::new(&[(field, "has already been taken")], Option::from(Status::Conflict))
        });

    Ok(json!(security::jwt::generate_token_for_user(&conn, created_user?)))
}

#[get("/auth/token", format = "json")]
pub fn renew_token(
    auth_user: AuthUser,
    conn: Conn,
) -> Result<JsonValue, Errors> {

    let user = user::db::find(&conn, auth_user.user_id).unwrap();
    Ok(json!(security::jwt::generate_token_for_user(&conn, user)))
}

#[derive(Deserialize, Validate)]
pub struct PasswordChange {
    #[validate(length(min = 8))]
    old_password: Option<String>,
    #[validate(length(min = 8))]
    new_password: Option<String>,
}
//user::db::update_password(&conn, user.id, &crypt_password(&new_password).to_string());
//     Ok(())
#[put("/auth/password", format = "json", data = "<change_password>")]
pub fn change_password(
    auth_user: AuthUser,
    change_password: Json<PasswordChange>,
    conn: Conn,
) -> Result<(), Errors> {
    let change_password = change_password.into_inner();
    let mut extractor = FieldValidator::validate(&change_password);
    let old_password = extractor.extract("old_password", change_password.old_password);
    let new_password = extractor.extract("new_password", change_password.new_password);
    extractor.check()?;

    security::service::change_password(&conn, auth_user.user_id, old_password, new_password)
}

#[derive(Deserialize, Validate)]
pub struct PasswordCreate {
    #[validate(length(min = 8))]
    new_password: Option<String>,
}

#[put("/auth/password-create", format = "json", data = "<create_password>")]
pub fn create_password(
    auth_user: AuthUser,
    create_password: Json<PasswordCreate>,
    conn: Conn,
) -> Result<(), Errors> {
    let create_password = create_password.into_inner();
    let mut extractor = FieldValidator::validate(&create_password);
    let new_password = extractor.extract("new_password", create_password.new_password);
    extractor.check()?;

    security::service::create_password(&conn, auth_user.user_id, new_password)
}

#[get("/oauth/login/github")]
pub fn github_login(oauth2: OAuth2<GitHub>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &["user:email"]).unwrap()
}

#[get("/oauth/github/callback")]
pub fn github_callback(conn: Conn, token: TokenResponse<GitHub>, cookies: Cookies<'_>) -> Redirect {
    oauth_callback(conn, token, cookies)
}

#[get("/oauth/login/gitlab")]
pub fn gitlab_login(oauth2: OAuth2<GitLab>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &["read_user"]).unwrap()
}

#[get("/oauth/gitlab/callback")]
pub fn gitlab_callback(conn: Conn, token: TokenResponse<GitLab>, cookies: Cookies<'_>) -> Redirect {
    oauth_callback(conn, token, cookies)
}

#[get("/oauth/login/gitlab-taltech")]
pub fn gitlab_taltech_login(oauth2: OAuth2<GitLabTalTech>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &["api"]).unwrap()
}

#[get("/oauth/gitlab-taltech/callback")]
pub fn gitlab_taltech_callback(conn: Conn, token: TokenResponse<GitLabTalTech>, cookies: Cookies<'_>) -> Redirect {
    oauth_callback(conn, token, cookies)
}

#[get("/oauth/login/microsoft")]
pub fn microsoft_login(oauth2: OAuth2<Microsoft>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &["User.ReadBasic.All"]).unwrap()
}

#[get("/oauth/microsoft/callback")]
pub fn microsoft_callback(conn: Conn, token: TokenResponse<Microsoft>, cookies: Cookies<'_>) -> Redirect {
    oauth_callback(conn, token, cookies)
}

#[get("/oauth/login/bitbucket")]
pub fn bitbucket_login(oauth2: OAuth2<Bitbucket>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &["account repository"]).unwrap()
}

#[get("/oauth/bitbucket/callback")]
pub fn bitbucket_callback(conn: Conn, token: TokenResponse<Bitbucket>, cookies: Cookies<'_>) -> Redirect {
    oauth_callback(conn, token, cookies)
}

fn oauth_callback<T>(conn: Conn, token: TokenResponse<T>, cookies: Cookies<'_>) -> Redirect
    where TokenResponse<T>: LoginType
{
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    if let Some(client_token) = cookies.get(&security::config::JWT_COOKIE) {
        if let Some(auth_user) = security::jwt::get_auth_user_from_token(&conn, client_token.value()) {
            if let Err(_) = rt.block_on(security::service::oauth_register(&conn, &token, auth_user.user_id)) {
                error!("OAuth register error");
            }
        }
        return Redirect::to(security::config::REGISTER_REDIRECT.read().unwrap().clone());
    }

    let jwt = rt.block_on(security::service::oauth_login(&conn, &token));
    let jwt_token = match jwt {
        None => rt.block_on(security::service::login_and_register(&conn, token)),
        Some(token) => token
    };
    Redirect::to( format!("{}?token={}", security::config::LOGIN_REDIRECT.read().unwrap().clone(), jwt_token))
}
