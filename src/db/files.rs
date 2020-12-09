use crate::models::file::{File, FileJson};
use crate::schema::files;
use crate::errors::{FieldValidator};
use crate::routes::files::NewFileData;
use crate::db;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};


#[derive(Insertable)]
#[table_name = "files"]
struct NewFile<'a> {
    commit: i32,
    path: &'a str,
    status: &'a str,
    time: i64,
    lines_added: i64,
    lines_deleted: i64,
}

pub fn create_all(
    conn: &PgConnection,
    files: Vec<NewFileData>,
    commit: i32
) -> Vec<FileJson> {
    let mut vec = Vec::new();
    for var in files {
        let mut extractor = FieldValidator::validate(&var);
        let path = &extractor.extract("path", var.path);
        let status = &extractor.extract("status", var.status);
        let time = extractor.extract("time_total", var.time_total);
        let lines_added = extractor.extract("added_lines", var.added_lines);
        let lines_deleted = extractor.extract("deleted_lines", var.deleted_lines);

        let new_file = &NewFile {
            commit,
            path,
            status,
            time,
            lines_added,
            lines_deleted,
        };

        let file = diesel::insert_into(files::table)
            .values(new_file)
            .get_result::<File>(conn)
            .expect("Error creating file");

        let timeline_vec = db::timelines::create_all(
            &conn,
            var.timeline,
            file.id
        );

        vec.push(file.attach(timeline_vec))
    }
    vec
}