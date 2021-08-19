use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_okapi::{openapi, JsonSchema};
use serde::Deserialize;
use validator::Validate;

use crate::domain::db::Conn;
use crate::domain::role::model::ADMIN;
use crate::domain::timeline;
use crate::domain::timeline::resources::{
    ActivityJson, ComparisonJsonWrapper, IntervalJson, SubdirLevelTimelineJsonWrapper,
};
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
    } as f64)
        .sqrt();
    if cumulative {
        time_threshold_multiplier *= 2.0;
    }
    let time_threshold = params
        .time_threshold
        .unwrap_or(((end - start) as f64).sqrt() * time_threshold_multiplier / 5000.0);
    let line_threshold = params
        .lines_threshold
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

// TODO: Switch to vec in rocket 0.5.0
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
    let groups: Vec<String> = validator
        .extract("groups", params.groups)
        .split(",")
        .map(|s| s.to_string())
        .collect();
    let repos = params
        .repo
        .unwrap_or("".to_string())
        .split(",")
        .map(|r| r.parse::<i32>().unwrap_or(-1))
        .collect();
    let branches = params
        .branch
        .unwrap_or("".to_string())
        .split(",")
        .filter(|s| s.len() > 0)
        .map(|s| s.to_string())
        .collect();
    let users = params
        .user
        .unwrap_or("".to_string())
        .split(",")
        .map(|u| u.to_string())
        .collect();
    let start = validator.extract("start", params.start);
    let end = validator.extract("end", params.end);
    let interval = validator.extract("interval", params.interval);
    let timezone = validator.extract("timezone", params.timezone);
    validator.validate_timeline_period(start, end, &interval);
    validator.check()?;

    if auth_user.require_role(&ADMIN).is_err() {
        for group_name in &groups {
            security::service::check_group_access(&conn, auth_user.user_id, &group_name)?;
        }
    }

    let timeline = timeline::service::get_timeline_comparison(
        &conn, &groups, &repos, &branches, &users, start, end, &timezone, &interval,
    )?;
    Ok(Json(timeline))
}
