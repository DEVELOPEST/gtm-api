use serde::Deserialize;
use validator::Validate;
use crate::db;
use rocket_contrib::json::{JsonValue};
use rocket::request::Form;

#[derive(Deserialize, Validate)]
pub struct NewTimelineData {
    pub timestamp: Option<i64>,
    pub time: Option<i64>,
}

#[derive(FromForm, Default)]
pub struct Period {
    start: Option<i64>,
    end: Option<i64>,
}

#[get("/<group_name>/timeline?<params..>")]
pub fn get_day_timeline(
    //auth: Auth,
    group_name: String,
    params: Form<Period>,
    conn: db::Conn,
) -> JsonValue {
    // TODO input validation
    let start = params.start.unwrap_or(-1);
    let end = params.end.unwrap_or(-1);
    let day_timeline = db::timelines::get_day(&conn, &group_name, start, end);
    json!({ "timeline": day_timeline })
}