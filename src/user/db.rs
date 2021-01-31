use crypto::scrypt::{scrypt_simple, ScryptParams};
use diesel::{Insertable, sql_types};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use crate::user::model::User;
use crate::schema::users;
use crate::schema::user_role_members;
use crate::schema::roles;
use crate::user::dwh::UserDWH;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

pub enum UserCreationError {
    DuplicatedEmail,
    DuplicatedUsername,
}

impl From<Error> for UserCreationError {
    fn from(err: Error) -> UserCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => return UserCreationError::DuplicatedUsername,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}

pub fn create(
    conn: &PgConnection,
    email: &str,
    password: &str,
) -> Result<User, UserCreationError> {
    // see https://blog.filippo.io/the-scrypt-parameters
    let hash = &scrypt_simple(password, &ScryptParams::new(10, 8, 1)).expect("hash error");

    let new_user = &NewUser {
        email,
        password: hash,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(Into::into)
}

pub fn find(conn: &PgConnection, id: i32) -> Option<User> {
    users::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}

pub fn find_by_email(conn: &PgConnection, email: &str) -> Option<User> {
    users::table
        .filter(users::email.eq(email))
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
        .select((users::id, users::email, users::password, roles::name))
        .load::<UserDWH>(conn)
        .expect("Cannot load users");
    users
}
