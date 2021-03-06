use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::schema::login;
use crate::security::model::Login;

pub fn exists_oauth_login(conn: &PgConnection, user_id: i32, login_type: i32) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(login::table
        .filter(login::user.eq(user_id)
            .and(login::login_type.eq(login_type)))))
        .get_result(conn)
        .expect("Error loading repository")
}

#[derive(Insertable)]
#[table_name = "login"]
pub struct NewLogin {
    pub user: i32,
    pub login_type: i32,
    pub token: String,
    pub refresh_token: Option<String>,
    pub exp: Option<i64>,
}

pub fn create_oauth_login(
    conn: &PgConnection,
    user_id: i32,
    login_type: i32,
    token: &str,
    refresh_token: Option<String>,
    exp: Option<i64>) -> Option<Login> {
    let login = NewLogin {
        user: user_id,
        login_type,
        token: token.to_string(),
        refresh_token,
        exp,
    };

    diesel::insert_into(login::table)
        .values(login)
        .get_result::<Login>(conn)
        .ok()
}

pub fn update_oauth_login(
    conn: &PgConnection,
    user_id: i32,
    login_type: i32,
    token: &str,
    refresh_token: Option<&str>,
    exp: Option<i64>) -> Option<Login> {
    diesel::update(
        login::table
            .filter(login::user.eq(user_id)
                .and(login::login_type.eq(login_type))))
        .set(
            (login::token.eq(token),
             login::refresh_token.eq(refresh_token),
             login::exp.eq(exp)))
        .get_result::<Login>(conn)
        .ok()
}