use std::fmt::{Display, Formatter};
use std::fmt;

use diesel::Insertable;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;

use crate::errors::Error;
use crate::schema::roles;
use crate::schema::user_role_members;
use crate::schema::users;
use crate::domain::user::dwh::UserDWH;
use crate::domain::user::model::User;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: Option<String>,
}

pub enum UserCreationError {
    DuplicatedUsername,
}

impl From<diesel::result::Error> for UserCreationError {
    fn from(err: diesel::result::Error) -> UserCreationError {
        if let diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => return UserCreationError::DuplicatedUsername,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}

impl Display for UserCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UserCreationError::DuplicatedUsername => write!(f, "Duplicate Username")
        }
    }
}

pub fn create(
    conn: &PgConnection,
    username: &str,
    password: Option<String>,
) -> Result<User, UserCreationError> {
    // see https://blog.filippo.io/the-scrypt-parameters
    let new_user = &NewUser {
        username,
        password,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(Into::into)
}

pub fn update_password(
    conn: &PgConnection,
    user_id: i32,
    password: &str, ) -> Result<User, Error> {
    let user = diesel::update(
        users::table
            .filter(users::id.eq(user_id)))
        .set(users::password.eq(password))
        .get_result::<User>(conn)?;
    Ok(user)
}

pub fn find(conn: &PgConnection, id: i32) -> Result<User, Error> {
    users::table
        .find(id)
        .get_result(conn)
        .map_err(Error::DatabaseError)
}

pub fn find_by_username(conn: &PgConnection, username: &str) -> Option<User> {
    users::table
        .filter(users::username.eq(username))
        .first::<User>(conn).ok()
}

pub fn exists(conn: &PgConnection, id: i32) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(users::table
        .filter(users::id.eq(id))))
        .get_result(conn)
        .expect("Error finding  user")
}

pub fn find_all(conn: &PgConnection) -> Vec<UserDWH> {
    let users: Vec<UserDWH> = users::table
        .inner_join(user_role_members::table)
        .inner_join(roles::table.on(roles::id.eq(user_role_members::role)))
        .select((users::id, users::username, users::password, roles::name))
        .load::<UserDWH>(conn)
        .expect("Cannot load users");
    users
}
