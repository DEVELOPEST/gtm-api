use rocket_contrib::json::{JsonValue, Json};
use crate::db::{self, users::UserCreationError};
use serde::Deserialize;
use validator::Validate;

use crate::db::Conn;
use crate::errors::{Errors, FieldValidator};
use crate::helpers::jwt;
use crypto::scrypt::{scrypt_check};

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

    let user = db::users::find_by_email(&conn, &email);

    if user.is_none() {
        return Err(Errors::new(&[("email", "Cannot find user with email")]));
    }

    let user = user.unwrap();
    if !scrypt_check(&password, &user.password).unwrap() {
        return Err(Errors::new(&[("password", "Wrong password!")]));
    }

    Ok(json!({"jwt": jwt::generate_token_for_user(user)}))
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
    conn: db::Conn,
) -> Result<JsonValue, Errors> {

    let new_user = new_user.into_inner().user;

    let mut extractor = FieldValidator::validate(&new_user);
    let email = extractor.extract("email", new_user.email);
    let password = extractor.extract("password", new_user.password);

    extractor.check()?;

    let created_user = db::users::create(&conn, &email, &password)
        .map_err(|error| {
            let field = match error {
                UserCreationError::DuplicatedEmail => "email",
                UserCreationError::DuplicatedUsername => "username",
            };
            Errors::new(&[(field, "has already been taken")])
        });

    Ok(json!(jwt::generate_token_for_user(created_user?)))
}