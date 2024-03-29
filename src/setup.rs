use rocket::fairing::AdHoc;

use crate::domain::db::Conn;

embed_migrations!();

pub fn migrate_database() -> AdHoc {
    AdHoc::on_attach("Database Migrations", |rocket| {
        let conn = Conn::get_one(&rocket).expect("database connection");
        match embedded_migrations::run(&*conn) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                error!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        }
    })
}
