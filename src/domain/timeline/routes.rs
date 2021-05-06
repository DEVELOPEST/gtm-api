use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_okapi::{JsonSchema, openapi};
use serde::Deserialize;
use validator::Validate;

use crate::domain::db::Conn;
use crate::domain::role::model::ADMIN;
use crate::domain::timeline;
use crate::domain::timeline::resources::{ActivityJson, IntervalJson, SubdirLevelTimelineJsonWrapper};
use crate::domain::user::model::AuthUser;
use crate::errors::{Error, FieldValidator};
use crate::security;

#[derive(Deserialize, Validate, JsonSchema)]
pub struct NewTimelineData {
    pub timestamp: Option<i64>,
    pub time: Option<i64>,
}

#[derive(FromForm, Default, Validate, Deserialize, JsonSchema)]
pub struct TimelineParams {
    start: Option<i64>,
    end: Option<i64>,
    interval: Option<String>,
    timezone: Option<String>,
    cumulative: Option<bool>,
}

#[derive(FromForm, Default, Validate, Deserialize, JsonSchema)]
pub struct SubdirTimelineParams {
    start: Option<i64>,
    end: Option<i64>,
    interval: Option<String>,
    timezone: Option<String>,
    depth: Option<i32>,
    time_threshold: Option<f64>,
    lines_threshold: Option<i64>,
    cumulative: Option<bool>,
}

#[openapi]
#[get("/<group_name>/timeline?<params..>")]
pub fn get_timeline(
    auth_user: AuthUser,
    group_name: String,
    params: Form<TimelineParams>,
    conn: Conn,
) -> Result<Json<Vec<IntervalJson>>, Error> {
    if auth_user.require_role(&ADMIN).is_err() {
        security::service::check_group_access(&conn, auth_user.user_id, &group_name)?;
    }
    let params = params.into_inner();
    let mut validator = FieldValidator::validate(&params);
    let start = validator.extract("start", params.start);
    let end = validator.extract("end", params.end);
    let interval = validator.extract("interval", params.interval);
    let timezone = validator.extract("timezone", params.timezone);
    let cumulative = params.cumulative.unwrap_or(false);
    validator.validate_timeline_period(start, end, &interval);
    validator.check()?;

    let timeline = timeline::service::get_timeline(
        &conn,
        &group_name,
        start,
        end,
        &timezone,
        &interval,
        cumulative,
    );
    Ok(Json(timeline))
}

#[openapi]
#[get("/<group_name>/activity?<params..>")]
pub fn get_activity_timeline(
    auth_user: AuthUser,
    group_name: String,
    params: Form<TimelineParams>,
    conn: Conn,
) -> Result<Json<Vec<ActivityJson>>, Error> {
    if auth_user.require_role(&ADMIN).is_err() {
        security::service::check_group_access(&conn, auth_user.user_id, &group_name)?;
    }
    let params = params.into_inner();
    let mut validator = FieldValidator::validate(&params);
    let start = validator.extract("start", params.start);
    let end = validator.extract("end", params.end);
    let interval = validator.extract("interval", params.interval);
    let timezone = validator.extract("timezone", params.timezone);
    let cumulative = params.cumulative.unwrap_or(false);
    validator.validate_timeline_period(start, end, &interval);
    validator.check()?;

    let timeline = timeline::service::get_activity_timeline(
        &conn,
        &group_name,
        start,
        end,
        &timezone,
        &interval,
        cumulative,
    )?;
    Ok(Json(timeline))
}

#[openapi]
#[get("/<group_name>/subdirs-timeline?<params..>")]
pub fn get_subdir_level_timeline(
    auth_user: AuthUser,
    conn: Conn,
    group_name: String,
    params: Form<SubdirTimelineParams>,
) -> Result<Json<SubdirLevelTimelineJsonWrapper>, Error> {
    if auth_user.require_role(&ADMIN).is_err() {
        security::service::check_group_access(&conn, auth_user.user_id, &group_name)?;
    }
    let params = params.into_inner();

    let mut validator = FieldValidator::validate(&params);
    let start = validator.extract("start", params.start);
    let end = validator.extract("end", params.end);
    let interval = validator.extract("interval", params.interval);
    let timezone = validator.extract("timezone", params.timezone);
    let depth = validator.extract("depth", params.depth);
    let cumulative = params.cumulative.unwrap_or(false);

    let mut time_threshold_multiplier = (match &*interval {
        "year" => 365.0,
        "month" => 30.0,
        "week" => 7.0,
        "day" => 1.0,
        _ => 1.0,
    } as f64).sqrt();
    if cumulative {
        time_threshold_multiplier *= 2.0;
    }
    let time_threshold = params.time_threshold
        .unwrap_or(((end - start) as f64).sqrt() * time_threshold_multiplier / 5000.0);
    let line_threshold = params.lines_threshold
        .unwrap_or((((end - start) as f64).sqrt() * time_threshold_multiplier / 51.0) as i64);
    // TODO: validate depth?
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
        cumulative,
    )?;

    Ok(Json(timeline))
}
