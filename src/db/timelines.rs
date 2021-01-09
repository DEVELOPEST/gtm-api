use crate::models::timeline::{Timeline, TimelineJson};
use crate::models::hour_data::{HourDataJson};
use crate::models::hour_data_dwh::{HourDataDWH};
use crate::schema::timeline;
use crate::schema::commits;
use crate::schema::files;
use crate::schema::repositories;
use crate::schema::groups;
use crate::schema::group_repository_members;
use crate::errors::{FieldValidator};
use crate::routes::timelines::NewTimelineData;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};
use crate::mappers::timeline::{map_day_data};



#[derive(Insertable)]
#[table_name = "timeline"]
struct NewTimeline<> {
    file: i32,
    timestamp: i64,
    time: i64,
}

pub fn create_all(
    conn: &PgConnection,
    files: Vec<NewTimelineData>,
    file: i32
) -> Vec<TimelineJson> {
    let mut vec = Vec::new();
    for var in files {
        let mut extractor = FieldValidator::validate(&var);
        let timestamp = extractor.extract("timestamp", var.timestamp);
        let time = extractor.extract("time", var.time);

        let new_timeline = &NewTimeline {
            file,
            timestamp,
            time,
        };

        let timeline_json = diesel::insert_into(timeline::table)
            .values(new_timeline)
            .get_result::<Timeline>(conn)
            .expect("Error creating timeline")
            .attach();

        vec.push(timeline_json)
    }
    vec
}

pub fn get_day(
    conn: &PgConnection,
    group_name: &str,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<HourDataJson> {
    let day_timeline = timeline::table
        .inner_join(files::table)
        .inner_join(commits::table.on(files::commit.eq(commits::id)))
        .inner_join(repositories::table.on(repositories::id.eq(commits::repository_id)))
        .inner_join(group_repository_members::table.on(group_repository_members::repository.eq(repositories::id)))
        .inner_join(groups::table.on(groups::id.eq(group_repository_members::group)))
        .filter(groups::name.eq(group_name)
            .and(timeline::timestamp.ge(start)
                .and(timeline::timestamp.lt(end))))
        .order(timeline::timestamp.asc())
        .select((repositories::user, timeline::time, timeline::timestamp ))
        .load::<HourDataDWH>(conn)
        .expect("Cannot get day timeline");
    map_day_data(day_timeline, start, end, timezone, interval)
}
