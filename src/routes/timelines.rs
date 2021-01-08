use serde::Deserialize;
use validator::Validate;
use crate::routes::files::NewFileData;
use crate::db;
use rocket_contrib::json::{JsonValue};
use rocket::request::Form;
use crate::db::timelines::{Period};


#[derive(Deserialize, Validate)]
pub struct NewTimelineData {
    pub timestamp: Option<i64>,
    pub time: Option<i64>,
}

#[get("/<repo>/timeline?<params..>")]
pub fn get_day_timeline(
    //auth: Auth,
    repo: String,
    params: Form<Period>,
    conn: db::Conn,
) -> JsonValue {
    let day_timeline = db::timelines::get_day(&conn, &repo, &params);
    json!({ "timeline": day_timeline })
}