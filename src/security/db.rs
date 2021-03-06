use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, JoinOnDsl};

use crate::schema::{logins, users};
use crate::security::model::Login;
use crate::user::model::User;

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
) -> Option<User> {
    users::table
        .inner_join(logins::table.on(users::id.eq(logins::user)))
        .filter(logins::identity_hash.eq(identity_hash))
        .select((users::id, users::username, users::password))
        .first::<User>(conn)
        .ok()
}