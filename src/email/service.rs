use diesel::PgConnection;

use crate::email::db;
use crate::errors::Error;

pub fn create_emails_for_user(conn: &PgConnection, user_id: i32, emails: Vec<&str>) -> Result<(), Error> {
    db::create_emails_for_user(conn, user_id, emails)?;
    Ok(())
}

pub fn delete_email_from_user(conn: &PgConnection, user_id: i32, email: &str) -> Result<(), Error> {
    db::delete_email_from_user(conn, user_id, email)?;
    Ok(())
}