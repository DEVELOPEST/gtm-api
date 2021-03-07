use rocket::http::{Cookie, Cookies, SameSite, Status};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::json::{Json, JsonValue};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::Deserialize;
use validator::Validate;

use crate::db::Conn;
use crate::errors::{Errors, FieldValidator};
use crate::security;
use crate::security::oauth::{GitHub, GitLab, LoginType};
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

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
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

    let created_user = security::service::new_user(&conn, &email, &password)
        .map_err(|error| {
            let field = match error {
                UserCreationError::DuplicatedEmail => "email",
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

#[derive(FromForm, Default, Validate, Deserialize)]
pub struct OAuthRegisterParams {
    token: Option<String>,
}

#[get("/oauth/register/github?<params..>")]
pub fn github_register(oauth2: OAuth2<GitHub>, mut cookies: Cookies<'_>, params: Form<OAuthRegisterParams>) -> Redirect {
    oauth_register(oauth2, cookies, &["user:read"], params.into_inner())
}

#[get("/oauth/login/github")]
pub fn github_login(oauth2: OAuth2<GitHub>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &["user:read"]).unwrap()
}

#[get("/oauth/github/callback")]
pub fn github_callback(conn: Conn, token: TokenResponse<GitHub>, mut cookies: Cookies<'_>) -> Redirect {
    oauth_callback(conn, token, cookies)
}

#[get("/oauth/register/gitlab?<params..>")]
pub fn gitlab_register(oauth2: OAuth2<GitLab>, mut cookies: Cookies<'_>, params: Form<OAuthRegisterParams>) -> Redirect {
    oauth_register(oauth2, cookies, &["read_user"], params.into_inner())
}

#[get("/oauth/login/gitlab")]
pub fn gitlab_login(oauth2: OAuth2<GitLab>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &["read_user"]).unwrap()
}

#[get("/oauth/gitlab/callback")]
pub fn gitlab_callback(conn: Conn, token: TokenResponse<GitLab>, mut cookies: Cookies<'_>) -> Redirect {
    oauth_callback(conn, token, cookies)
}

fn oauth_register<T>(oauth2: OAuth2<T>, mut cookies: Cookies<'_>, scopes: &[&str], params: OAuthRegisterParams) -> Redirect
    where T: 'static
{
    if params.token.is_none() {
        return Redirect::to(security::config::REGISTER_REDIRECT.read().unwrap().clone());
    }
    cookies.add_private(Cookie::build(security::config::JWT_COOKIE.clone(), params.token.unwrap())
        .same_site(SameSite::Lax)
        .finish());

    oauth2.get_redirect(&mut cookies, scopes).unwrap()
}

fn oauth_callback<T>(conn: Conn, token: TokenResponse<T>, mut cookies: Cookies<'_>) -> Redirect
    where TokenResponse<T>: LoginType
{
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    if let Some(client_token) = cookies.get_private(&security::config::JWT_COOKIE) {
        if let Err(_) = rt.block_on(security::service::oauth_register(&conn, token, client_token.value())) {
            error!("OAuth register error");
        }
        return Redirect::to(security::config::REGISTER_REDIRECT.read().unwrap().clone());
    }

    let token = rt.block_on(security::service::oauth_login(&conn, token));
    Redirect::to(format!("{}?token={}", security::config::LOGIN_REDIRECT.read().unwrap().clone(), token.unwrap_or("".to_string())))
}