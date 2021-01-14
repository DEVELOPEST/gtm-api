use crate::models::timeline::{Timeline, TimelineJson};
use crate::models::interval::{IntervalJson};
use crate::models::timeline_dwh::{TimelineDWH};
use crate::schema::timeline;
use crate::schema::commits;
use crate::schema::files;
use crate::schema::repositories;
use crate::schema::groups;
use crate::errors::{FieldValidator};
use crate::routes::timelines::NewTimelineData;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};
use crate::mappers::timeline::{map_timeline};



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

pub fn get_timeline(
    conn: &PgConnection,
    group_name: &str,
    start: i64,
    end: i64,
    timezone: &str,
    interval: &str,
) -> Vec<IntervalJson> {
    let day_timeline = timeline::table
        .inner_join(files::table)
        .inner_join(commits::table.on(files::commit.eq(commits::id)))
        .inner_join(repositories::table.on(repositories::id.eq(commits::repository_id)))
        // TODO(Tavo): Recursive join group_group_member
        // .inner_join(group_group_members::table.on(group_group_members::repository.eq(repositories::id)))
        .inner_join(groups::table.on(groups::id.eq(repositories::group)))
        .filter(groups::name.eq(group_name)
            .and(timeline::timestamp.ge(start)
                .and(timeline::timestamp.lt(end))))
        .order(timeline::timestamp.asc())
        .select((repositories::user, timeline::time, timeline::timestamp ))
        .load::<TimelineDWH>(conn)
        .expect("Cannot get day timeline");
    println!("{:?}", day_timeline);
    map_timeline(day_timeline, start, end, timezone, interval)
}
