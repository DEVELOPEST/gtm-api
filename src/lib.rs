#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_cors;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;

mod config;
mod db;
mod errors;
mod models;
mod routes;
mod schema;
mod mappers;
mod helpers;
mod setup;
mod services;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::ignite()
        .mount(
            "/services/gtm/api/",
            routes![
                routes::auth::login,
                routes::auth::register,
                routes::users::get_user,
                routes::commits::get_commit_hash,
                routes::repositories::post_repository,
                routes::repositories::put_repository,
                routes::groups::post_group_parents,
                routes::groups::post_group_children,
                routes::groups::get_groups,
                routes::timelines::get_timeline,
                routes::timelines::get_activity_timeline,
            ],
        )
        .attach(db::Conn::fairing())
        .attach(setup::migrate_database())
        .attach(cors_fairing())
        .attach(helpers::jwt::manage())
        .register(catchers![not_found])
}