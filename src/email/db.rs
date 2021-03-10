use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use diesel::result::Error;

use crate::schema;
use crate::schema::emails;

#[derive(Insertable)]
#[table_name = "emails"]
struct NewEmail<'a> {
    user: i32,
    email: &'a str,
}

pub fn create_emails_for_user(conn: &PgConnection, user_id: i32, emails: Vec<&str>) -> Result<usize, Error> {
    let values: Vec<NewEmail> = emails.iter()
        .map(|email| NewEmail { user: user_id, email })
        .collect();

    diesel::insert_into(schema::emails::table)
        .values(&values)
        .on_conflict(schema::emails::email)
        .do_nothing()
        .execute(conn)
}

pub fn delete_email_from_user(conn: &PgConnection, user_id: i32, email: &str) -> Result<usize, Error> {
    diesel::delete(
        schema::emails::table.filter(
            schema::emails::user.eq(user_id)
                .and(schema::emails::email.eq(email))))
        .execute(conn)
}