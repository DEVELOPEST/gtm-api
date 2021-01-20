use rocket_contrib::json::{JsonValue, Json};
use serde::Deserialize;
use validator::Validate;

use crate::db;
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

#[post("/user/login", format = "json", data = "<login_data>")]
pub fn post_login(conn: Conn, login_data: Json<LoginDto>) -> Result<JsonValue, Errors> {
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

    Ok(json!(jwt::generate_token_for_user(user)))
}