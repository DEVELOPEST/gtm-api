use rocket::request::Form;
use rocket_contrib::json::JsonValue;
use serde::Deserialize;
use validator::Validate;

use crate::{security, timeline};
use crate::db::Conn;
use crate::errors::{Error, FieldValidator};
use crate::role::model::ADMIN;
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

#[derive(FromForm, Default, Validate, Deserialize)]
pub struct SubdirTimelineParams {
    start: Option<i64>,
    end: Option<i64>,
    interval: Option<String>,
    timezone: Option<String>,
    depth: Option<i32>,
    time_threshold: Option<f32>,
    lines_threshold: Option<i32>,
}

#[get("/<group_name>/timeline?<params..>")]
pub fn get_timeline(
    auth_user: AuthUser,
    group_name: String,
    params: Form<Period>,
    conn: Conn,
) -> Result<JsonValue, Error> {
    if auth_user.require_role(&ADMIN).is_err() {
        security::service::check_group_access(&conn, auth_user.user_id, &group_name)?;
    }
    let period = params.into_inner();
    let mut validator = FieldValidator::validate(&period);
    let start = validator.extract("start", period.start);
    let end = validator.extract("end", period.end);
    let interval = validator.extract("interval", period.interval);
    let timezone = validator.extract("timezone", period.timezone);
    validator.validate_timeline_period(start, end, &interval);
    validator.check()?;

    let timeline = timeline::service::get_timeline(&conn, &group_name, start, end, &timezone, &interval);
    Ok(json!(timeline))
}

#[get("/<group_name>/activity?<params..>")]
pub fn get_activity_timeline(
    auth_user: AuthUser,
    group_name: String,
    params: Form<Period>,
    conn: Conn,
) -> Result<JsonValue, Error> {
    if auth_user.require_role(&ADMIN).is_err() {
        security::service::check_group_access(&conn, auth_user.user_id, &group_name)?;
    }
    let period = params.into_inner();
    let mut validator = FieldValidator::validate(&period);
    let start = validator.extract("start", period.start);
    let end = validator.extract("end", period.end);
    let interval = validator.extract("interval", period.interval);
    let timezone = validator.extract("timezone", period.timezone);
    validator.validate_timeline_period(start, end, &interval);
    validator.check()?;

    let timeline = timeline::service::get_activity_timeline(&conn, &group_name, start, end, &timezone, &interval)?;
    Ok(json!(timeline))
}

#[get("/<group_name>/subdirs-timeline?<params..>")]
pub fn get_subdir_level_timeline(
    auth_user: AuthUser,
    conn: Conn,
    group_name: String,
    params: Form<SubdirTimelineParams>,
) -> Result<JsonValue, Error> {
    if auth_user.require_role(&ADMIN).is_err() {
        security::service::check_group_access(&conn, auth_user.user_id, &group_name)?;
    }
    let timeline_params = params.into_inner();

    let mut validator = FieldValidator::validate(&timeline_params);
    let start = validator.extract("start", timeline_params.start);
    let end = validator.extract("end", timeline_params.end);
    let interval = validator.extract("interval", timeline_params.interval);
    let timezone = validator.extract("timezone", timeline_params.timezone);
    let depth = validator.extract("depth", timeline_params.depth);
    let time_threshold = timeline_params.time_threshold.unwrap_or(0.2);
    let line_threshold = timeline_params.lines_threshold.unwrap_or(10);
    //TODO: validate depth
    validator.validate_timeline_period(start, end, &interval);
    validator.check()?;

    let timeline = timeline::service::get_subdir_level_timeline(
        &conn,
        &group_name,
        depth,
        start,
        end,
        &timezone,
        &interval,
        time_threshold,
        line_threshold,
    )?;

    Ok(json!(timeline))
}