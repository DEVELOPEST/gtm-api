use rocket::fairing::AdHoc;

use crate::db;


embed_migrations!();

pub fn migrate_database() -> AdHoc {
    AdHoc::on_attach("Database Migrations", |rocket| {
        let conn = db::Conn::get_one(&rocket).expect("database connection");
        match embedded_migrations::run(&*conn) {
            Ok(()) => Ok(rocket),
            Err(_) => {
                // error!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        }
    })
}