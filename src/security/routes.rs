use crypto::scrypt::scrypt_check;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

use crate::db::Conn;
use crate::errors::{Errors, FieldValidator};
use crate::security;
use crate::user;
use crate::user::db::UserCreationError;
use crate::user::model::AuthUser;

#[derive(Deserialize, Validate)]
pub struct LoginDto {
    #[validate(length(min = 1))]
    pub email: Option<String>,
    #[validate(length(min = 1))]
    pub password: Option<String>,
}

#[post("/auth/login", format = "json", data = "<login_data>")]
pub fn login(conn: Conn, login_data: Json<LoginDto>) -> Result<JsonValue, Errors> {
    let login_data = login_data.into_inner();
    let mut extractor = FieldValidator::validate(&login_data);
    let email = extractor.extract("email", login_data.email);
    let password = extractor.extract("password", login_data.password);
    extractor.check()?;

    let user = user::db::find_by_email(&conn, &email);

    if user.is_none() {
        return Err(Errors::new(&[("email", "Cannot find user with email")], None));
    }

    let user = user.unwrap();
    if !scrypt_check(&password, &user.password).unwrap() {
        return Err(Errors::new(&[("password", "Wrong password!")], None));
    }

    Ok(json!({"jwt": security::jwt::generate_token_for_user(&conn, user)}))
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