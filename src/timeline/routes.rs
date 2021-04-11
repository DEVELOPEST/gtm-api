use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_okapi::{JsonSchema, openapi};
use serde::Deserialize;
use validator::{Validate, HasLen};

use crate::{security, timeline};
use crate::db::Conn;
use crate::errors::{Error, FieldValidator};
use crate::role::model::ADMIN;
use crate::timeline::resources::{ActivityJson, IntervalJson, SubdirLevelTimelineJsonWrapper, ComparisonJsonWrapper};
use crate::user::model::AuthUser;

#[derive(Deserialize, Validate, JsonSchema)]
pub struct NewTimelineData {
    pub timestamp: Option<i64>,
    pub time: Option<i64>,
}

#[derive(FromForm, Default, Validate, Deserialize, JsonSchema)]
pub struct Period {
    start: Option<i64>,
    end: Option<i64>,
    interval: Option<String>,
    timezone: Option<String>,
}

#[derive(FromForm, Default, Validate, Deserialize, JsonSchema)]
pub struct SubdirTimelineParams {
    start: Option<i64>,
    end: Option<i64>,
    interval: Option<String>,
    timezone: Option<String>,
    depth: Option<i32>,
    time_threshold: Option<f32>,
    lines_threshold: Option<i32>,
}

#[openapi]
#[get("/<group_name>/timeline?<params..>")]
pub fn get_timeline(
    auth_user: AuthUser,
    group_name: String,
    params: Form<Period>,
    conn: Conn,
) -> Result<Json<Vec<IntervalJson>>, Error> {
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
    Ok(Json(timeline))
}

#[openapi]
#[get("/<group_name>/activity?<params..>")]
pub fn get_activity_timeline(
    auth_user: AuthUser,
    group_name: String,
    params: Form<Period>,
    conn: Conn,
) -> Result<Json<Vec<ActivityJson>>, Error> {
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

    let timeline = timeline::service::get_activity_timeline(
        &conn,
        &group_name,
        start,
        end,
        &timezone,
        &interval,
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

    Ok(Json(timeline))
}

// TODO: Switch to vec in rocket 5.0
#[derive(FromForm, Default, Validate, Deserialize, JsonSchema)]
pub struct ComparisonParams {
    groups: Option<String>,
    repo: Option<String>,
    branch: Option<String>,
    user: Option<String>,
    start: Option<i64>,
    end: Option<i64>,
    interval: Option<String>,
    timezone: Option<String>,
}

#[openapi]
#[get("/comparison/timeline?<params..>")]
pub fn get_timeline_comparison(
    auth_user: AuthUser,
    params: Form<ComparisonParams>,
    conn: Conn,
) -> Result<Json<ComparisonJsonWrapper>, Error> {
    let params = params.into_inner();
    let mut validator = FieldValidator::validate(&params);
    let groups: Vec<String> = validator.extract("groups", params.groups)
        .split(",").map(|s| s.to_string()).collect();
    let repos = params.repo.unwrap_or("".to_string())
        .split(",").map(|r| r.parse::<i32>().unwrap_or(-1)).collect();
    let branches = params.branch.unwrap_or("".to_string())
        .split(",").filter(|s| s.len() > 0).map(|s| s.to_string()).collect();
    let users = params.user.unwrap_or("".to_string())
        .split(",").map(|u| u.parse::<i32>().unwrap_or(-1)).collect();
    let start = validator.extract("start", params.start);
    let end = validator.extract("end", params.end);
    let interval = validator.extract("interval", params.interval);
    let timezone = validator.extract("timezone", params.timezone);
    validator.validate_timeline_period(start, end, &interval);
    validator.check()?;

    // if auth_user.require_role(&ADMIN).is_err() {
    //     for group_name in &groups {
    //         security::service::check_group_access(&conn, auth_user.user_id, &group_name)?;
    //     }
    // }

    let timeline = timeline::service::get_timeline_comparison(
        &conn,
        &groups,
        &repos,
        &branches,
        &users,
        start,
        end,
        &timezone,
        &interval
    )?;
    Ok(Json(timeline))
}
