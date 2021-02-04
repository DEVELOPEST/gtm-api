use rocket::request::Form;
use rocket_contrib::json::JsonValue;
use serde::Deserialize;
use validator::Validate;

use crate::errors::{Errors, FieldValidator};
use crate::timeline;
use crate::db::Conn;
use crate::user::model::AuthUser;

#[derive(Deserialize, Validate)]
pub struct NewTimelineData {
    pub timestamp: Option<i64>,
    pub time: Option<i64>,
}

#[derive(FromForm, Default, Validate, Deserialize)]
pub struct Period {
    start: Option<i64>,
    end: Option<i64>,
    interval: Option<String>,
    timezone: Option<String>,
}

#[get("/<group_name>/timeline?<params..>")]
pub fn get_timeline(
    auth_user: AuthUser,
    group_name: String,
    params: Form<Period>,
    conn: Conn,
) -> Result<JsonValue, Errors> {
    let period = params.into_inner();

    let mut validator = FieldValidator::validate(&period);
    let start = validator.extract("start", period.start);
    let end = validator.extract("end", period.end);
    let interval = validator.extract("interval", period.interval);
    let timezone = validator.extract("timezone", period.timezone);
    validator.validate_timeline_period(start, end, &interval);
    validator.check()?;

    let timeline = timeline::service::get_timeline(&conn, &group_name, start, end, &timezone, &interval);
    Ok(json!({ "timeline": timeline }))
}

#[get("/<group_name>/activity?<params..>")]
pub fn get_activity_timeline(
    auth_user: AuthUser,
    group_name: String,
    params: Form<Period>,
    conn: Conn,
) -> Result<JsonValue, Errors> {
    let period = params.into_inner();

    let mut validator = FieldValidator::validate(&period);
    let start = validator.extract("start", period.start);
    let end = validator.extract("end", period.end);
    let interval = validator.extract("interval", period.interval);
    let timezone = validator.extract("timezone", period.timezone);
    validator.validate_timeline_period(start, end, &interval);
    validator.check()?;

    let timeline = timeline::service::get_activity_timeline(&conn, &group_name, start, end, &timezone, &interval);
    Ok(json!({ "activityTimeline": timeline }))
}
