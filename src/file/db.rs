use diesel;
use diesel::{Insertable, sql_query, sql_types, RunQueryDsl, PgConnection};
use crate::file::routes::NewFileData;
use crate::file::model::{FileJson, File};
use crate::errors::FieldValidator;
use crate::timeline::dwh::FileEditDWH;
use crate::schema::files;
use crate::timeline;
use crate::common::sql;

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

        let timeline_vec = timeline::db::create_all(
            &conn,
            var.timeline,
            file.id
        );

        vec.push(file.attach(timeline_vec))
    }
    vec
}

pub fn fetch_file_edits(conn: &PgConnection, group_name: &str, start: i64, end: i64) -> Vec<FileEditDWH> {
    let edit_timeline: Vec<FileEditDWH> = sql_query(format!("
    {}
    SELECT repositories.user, files.time, files.lines_added, files.lines_deleted, commits.timestamp
    FROM files
    INNER JOIN commits ON files.commit = commits.id
    INNER JOIN repositories ON commits.repository_id = repositories.id
    WHERE repositories.group IN (
        SELECT  group_repos_query.child
        FROM    group_repos_query
        UNION (
            SELECT g.id
            FROM groups g
            WHERE g.name = $1))
        AND commits.timestamp >= $2
        AND commits.timestamp < $3", sql::GROUP_REPOS_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)
        .expect("Error loading timeline for group");
    edit_timeline
}