#![feature(proc_macro_hygiene, decl_macro)]
#![feature(result_contains_err)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use(error)]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;
use rocket_contrib::json::JsonValue;
use rocket_cors;
use rocket_cors::Cors;
use rocket_oauth2::OAuth2;
use rocket_okapi::routes_with_openapi;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod config;
mod domain;
mod errors;
mod schema;
mod setup;
mod timeline;
mod security;
mod common;
mod vcs;

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

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "../openapi.json".to_owned(),
        ..Default::default()
    }
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::ignite()
        .mount("/services/gtm/api/",
            routes_with_openapi![
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
                security::routes::change_password,
                domain::sync::routes::post_sync_client,
                domain::sync::routes::delete_sync_client,
                domain::user::routes::get_user_id,
                domain::user::routes::get_user,
                domain::user::routes::get_users,
                domain::commit::routes::get_commit_hash,
                domain::repository::routes::post_repository,
                domain::repository::routes::put_repository,
                domain::repository::routes::delete_repository,
                domain::group::routes::post_group_parents,
                domain::group::routes::post_group_children,
                domain::group::routes::get_groups,
                domain::group::routes::get_group_stats,
                domain::group::routes::get_group_export,
                domain::group::routes::get_groups_with_access,
                domain::group::routes::get_groups_without_access,
                timeline::routes::get_timeline,
                timeline::routes::get_activity_timeline,
                timeline::routes::get_subdir_level_timeline,
                domain::role::routes::add_role_to_user,
                domain::role::routes::delete_role_from_user,
                domain::group_access::routes::post_group_accesses,
                domain::group_access::routes::delete_group_accesses,
                domain::group_access::routes::toggle_recursive_access,
                vcs::routes::get_accessible_repositories,
                vcs::routes::post_start_tracking_repository,
            ],
        )
        .mount("/services/gtm/api/swagger", make_swagger_ui(&get_docs()))
        .attach(domain::db::Conn::fairing())
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