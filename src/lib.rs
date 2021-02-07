#![feature(proc_macro_hygiene, decl_macro)]
#![feature(result_contains_err)]

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
mod schema;
mod setup;
mod repository;
mod user;
mod timeline;
mod security;
mod group_group_member;
mod group;
mod file;
mod common;
mod commit;
mod role;
mod user_role_member;

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
                security::routes::login,
                security::routes::register,
                security::routes::renew_token,
                security::routes::change_password,
                user::routes::get_user,
                user::routes::get_users,
                commit::routes::get_commit_hash,
                repository::routes::post_repository,
                repository::routes::put_repository,
                group::routes::post_group_parents,
                group::routes::post_group_children,
                group::routes::get_groups,
                group::routes::get_group_stats,
                timeline::routes::get_timeline,
                timeline::routes::get_activity_timeline,
                timeline::routes::get_subdir_level_timeline,
                role::routes::add_role_to_user,
            ],
        )
        .attach(db::Conn::fairing())
        .attach(setup::migrate_database())
        .attach(cors_fairing())
        .attach(security::jwt::manage())
        .attach(security::api_key::manage())
        .register(catchers![not_found])
}