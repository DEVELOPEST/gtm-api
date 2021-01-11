use serde::Deserialize;
use crate::errors::{Errors, FieldValidator};
use validator::Validate;
use crate::db;
use rocket_contrib::json::{JsonValue};
use rocket::request::Form;
use chrono::{NaiveDate, DateTime, Datelike};
use rocket::http::route::Source::Data;
use chrono::format::strftime;


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
    //auth: Auth,
    group_name: String,
    params: Form<Period>,
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let period = params.into_inner();

    let mut validator = FieldValidator::validate(&period);
    let start = validator.extract("start", period.start);
    let end = validator.extract("end", period.end);
    let interval = validator.extract("interval", period.interval);
    let timezone = validator.extract("timezone", period.timezone);
    validator.validate_timeline_period(start, end, &interval, &timezone);
    validator.check()?;


    let timeline = db::timelines::get_timeline(&conn, &group_name, start, end, &timezone, &interval);
    Ok(json!({ "timeline": timeline }))
}