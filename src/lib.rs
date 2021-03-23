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

#[macro_use(error)] extern crate log;

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
mod group_access;
mod email;
mod github;
mod gitlab;
mod microsoft;
mod bitbucket;
mod vcs;
mod sync;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;
use rocket_oauth2::OAuth2;

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
                security::routes::github_callback,
                security::routes::github_login,
                security::routes::gitlab_callback,
                security::routes::gitlab_login,
                security::routes::gitlab_taltech_callback,
                security::routes::gitlab_taltech_login,
                security::routes::microsoft_callback,
                security::routes::microsoft_login,
                security::routes::bitbucket_callback,
                security::routes::bitbucket_login,
                security::routes::get_user_logins,
                security::routes::delete_user_login,
                security::routes::delete_account,
                security::routes::has_password,
                security::routes::create_password,
                user::routes::get_user_id,
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
                group::routes::get_groups_with_access,
                group::routes::get_groups_without_access,
                timeline::routes::get_timeline,
                timeline::routes::get_activity_timeline,
                timeline::routes::get_subdir_level_timeline,
                role::routes::add_role_to_user,
                role::routes::delete_role_from_user,
                group_access::routes::post_group_accesses,
                group_access::routes::delete_group_accesses,
                group_access::routes::toggle_recursive_access,
                vcs::routes::get_accessible_repositories,
                vcs::routes::post_start_tracking_repository,
            ],
        )
        .attach(db::Conn::fairing())
        .attach(setup::migrate_database())
        .attach(cors_fairing())
        .attach(security::config::manage())
        .attach(OAuth2::<security::oauth::GitHub>::fairing("github"))
        .attach(OAuth2::<security::oauth::GitLab>::fairing("gitlab"))
        .attach(OAuth2::<security::oauth::GitLabTalTech>::fairing("gitlab-taltech"))
        .attach(OAuth2::<security::oauth::Microsoft>::fairing("microsoft"))
        .attach(OAuth2::<security::oauth::Bitbucket>::fairing("bitbucket"))
        .register(catchers![not_found])
}