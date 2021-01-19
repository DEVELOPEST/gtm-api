use crate::db::{self, users::UserCreationError};
use crate::errors::{Errors, FieldValidator};
use rocket::{self, get, post};
use crate::helpers::jwt;

use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;
use crate::models::user::AuthUser;


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

#[get("/user")]
pub fn get_user(user: AuthUser, conn: db::Conn) -> Option<JsonValue> {
    db::users::find(&conn, user.user_id)
        .map(|user| json!({ "user": user.id }))
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn post_users(
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
