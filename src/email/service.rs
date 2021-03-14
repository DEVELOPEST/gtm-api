use diesel::PgConnection;

use crate::email::db;

pub fn create_emails_for_user(conn: &PgConnection, user_id: i32, emails: Vec<&str>) {
    db::create_emails_for_user(conn, user_id, emails)
        .map_err(|e| error!("Error creating email: {}", e))
        .unwrap();
    // TODO: Some better error system
}

pub fn delete_email_from_user(conn: &PgConnection, user_id: i32, email: &str) {
    db::delete_email_from_user(conn, user_id, email)
        .map_err(|e| error!("Error deleting email: {}", e))
        .unwrap();
    // TODO: Some better error system
}