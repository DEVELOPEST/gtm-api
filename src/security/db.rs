use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use diesel::result::Error;

use crate::schema::{login_types, logins, users};
use crate::security::model::Login;
use crate::security::model::LoginType;
use crate::domain::user;
use crate::domain::user::model::User;

pub fn exists_oauth_login(conn: &PgConnection, user_id: i32, login_type: i32) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(logins::table
        .filter(logins::user.eq(user_id)
            .and(logins::login_type.eq(login_type)))))
        .get_result(conn)
        .expect("Error loading repository")
}

#[derive(Insertable)]
#[table_name = "logins"]
pub struct NewLogin<'a> {
    pub user: i32,
    pub login_type: i32,
    pub identity_hash: &'a str,
    pub token: &'a str,
    pub refresh_token: Option<&'a str>,
    pub exp: Option<i64>,
}

pub fn create_oauth_login(
    conn: &PgConnection,
    user_id: i32,
    login_type: i32,
    identity_hash: &str,
    token: &str,
    refresh_token: Option<&str>,
    exp: Option<i64>) -> Option<Login> {
    let login = NewLogin {
        user: user_id,
        login_type,
        identity_hash,
        token,
        refresh_token,
        exp,
    };

    diesel::insert_into(logins::table)
        .values(login)
        .get_result::<Login>(conn)
        .ok()
}

pub fn update_oauth_login(
    conn: &PgConnection,
    user_id: i32,
    login_type: i32,
    identity_hash: &str,
    token: &str,
    refresh_token: Option<&str>,
    exp: Option<i64>) -> Option<Login> {
    diesel::update(
        logins::table
            .filter(logins::user.eq(user_id)
                .and(logins::login_type.eq(login_type))))
        .set(
            (logins::identity_hash.eq(identity_hash),
             logins::token.eq(token),
             logins::refresh_token.eq(refresh_token),
             logins::exp.eq(exp)))
        .get_result::<Login>(conn)
        .ok()
}

pub fn find_user_for_oath_login(
    conn: &PgConnection,
    identity_hash: &str,
    login_type: i32,
) -> Option<User> {
    users::table
        .inner_join(logins::table.on(users::id.eq(logins::user)))
        .filter(logins::identity_hash.eq(identity_hash)
            .and(logins::login_type.eq(login_type)))
        .select((users::id, users::username, users::password))
        .first::<User>(conn)
        .ok()
}

pub fn find_all_login_names_by_user(
    conn: &PgConnection,
    user_id: i32,
) -> Vec<String> {
    logins::table
        .inner_join(users::table.on(logins::user.eq(users::id)))
        .inner_join(login_types::table.on(logins::login_type.eq(login_types::id)))
        .filter(users::id.eq(user_id))
        .select(login_types::name)
        .load::<String>(conn)
        .expect("Cannot load logins")
}

pub fn find_all_logins_by_user(
    conn: &PgConnection,
    user_id: i32,
) -> Result<Vec<Login>, Error> {
    Ok(logins::table
        .inner_join(users::table.on(logins::user.eq(users::id)))
        .filter(users::id.eq(user_id))
        .select(logins::all_columns)
        .load::<Login>(conn)?)
}

pub fn delete_login_by_user_and_type(
    conn: &PgConnection,
    user_id: i32,
    login_type_string: &str,
) -> Result<usize, Error> {
    let login_type: Option<LoginType> = login_types::table
        .filter(login_types::name.eq(login_type_string))
        .select((login_types::id, login_types::name))
        .first::<LoginType>(conn)
        .ok();

    diesel::delete(
        logins::table
            .filter(logins::user.eq(user_id)
                .and(logins::login_type.eq(login_type.unwrap().id))))
        .execute(conn)
}

pub fn delete_account(
    conn: &PgConnection,
    user_id: i32,
) -> Result<usize, Error> {
    diesel::delete(users::table.filter(users::id.eq(user_id))).execute(conn)
}

pub fn exists_password(conn: &PgConnection, user_id: i32) -> bool {
    user::db::find(conn, user_id).unwrap().password.is_some()
}