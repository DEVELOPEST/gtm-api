// use crate::auth::Auth;
use crate::config::AppState;
use crate::db::{self, users::UserCreationError};
use crate::errors::{Errors, FieldValidator};
use rocket::{self, get, post};

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;


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
pub fn get_user(conn: db::Conn, state: State<AppState>) -> Option<JsonValue> {
    db::users::find(&conn, 1).map(|user| json!({ "user": user.to_user_auth(&state.secret) }))
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn post_users(
    new_user: Json<NewUser>,
    conn: db::Conn,
    state: State<AppState>,
) -> Result<JsonValue, Errors> {

    let new_user = new_user.into_inner().user;


    let mut extractor = FieldValidator::validate(&new_user);
    let email = extractor.extract("email", new_user.email);
    let password = extractor.extract("password", new_user.password);

    extractor.check()?;

    db::users::create(&conn, &email, &password)
        .map(|user| json!({ "user": user.to_user_auth(&state.secret) }))
        .map_err(|error| {
            let field = match error {
                UserCreationError::DuplicatedEmail => "email",
                UserCreationError::DuplicatedUsername => "username",
            };
            Errors::new(&[(field, "has already been taken")])
        })
}