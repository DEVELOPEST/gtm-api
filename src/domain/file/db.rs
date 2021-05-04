use diesel;
use diesel::{Insertable, PgConnection, RunQueryDsl, sql_query, sql_types};

use crate::common::sql;
use crate::errors::{Error, FieldValidator};
use crate::domain::file::model::{File};
use crate::domain::file::resource::{NewFileData, FileJson};
use crate::schema::files;
use crate::timeline;
use crate::timeline::dwh::{FileEditDWH, PathlessFileEditDWH};

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
    commit: i32,
) -> Result<Vec<FileJson>, Error> {
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
            .get_result::<File>(conn)?;

        let timeline_vec = timeline::db::create_all(
            &conn,
            var.timeline,
            file.id,
        )?.into_iter()
            .map(|f| f.attach())
            .collect();

        vec.push(file.attach(timeline_vec))
    }
    Ok(vec)
}

pub fn fetch_pathless_file_edits(conn: &PgConnection, group_name: &str, start: i64, end: i64) -> Result<Vec<PathlessFileEditDWH>, Error> {
    let edit_timeline: Vec<PathlessFileEditDWH> = sql_query(format!("
    {}
    SELECT coalesce(users.username, commits.email) AS user,
           timeline.time,
           files.lines_added,
           files.lines_deleted,
           timeline.timestamp
    FROM timeline
        INNER JOIN files ON timeline.file = files.id
        INNER JOIN commits ON files.commit = commits.id
        INNER JOIN repositories ON commits.repository_id = repositories.id
        LEFT JOIN emails ON commits.email = emails.email
        LEFT JOIN users ON emails.user = users.id
    WHERE repositories.group IN (
        SELECT  group_repos_query.child
        FROM    group_repos_query
        UNION (
            SELECT g.id
            FROM groups g
            WHERE g.name = $1))
        AND commits.timestamp >= $2
        AND commits.timestamp < $3", sql::GROUP_CHILDREN_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)?;

    Ok(edit_timeline)
}

pub fn fetch_file_edits(conn: &PgConnection, group_name: &str, start: i64, end: i64) -> Result<Vec<FileEditDWH>, Error> {
    let edit_timeline: Vec<FileEditDWH> = sql_query(format!("
    {}
    SELECT
        coalesce(users.username, commits.email) AS user,
        files.path,
        timeline.time,
        files.lines_added,
        files.lines_deleted,
        timeline.timestamp,
        commits.hash AS commit_hash
    FROM timeline
        INNER JOIN files ON timeline.file = files.id
        INNER JOIN commits ON files.commit = commits.id
        INNER JOIN repositories ON commits.repository_id = repositories.id
        LEFT JOIN emails ON commits.email = emails.email
        LEFT JOIN users ON emails.user = users.id
    WHERE repositories.group IN (
        SELECT  group_repos_query.child
        FROM    group_repos_query
        UNION (
            SELECT g.id
            FROM groups g
            WHERE g.name = $1))
        AND commits.timestamp >= $2
        AND commits.timestamp < $3", sql::GROUP_CHILDREN_QUERY))
        .bind::<sql_types::Text, _>(group_name)
        .bind::<sql_types::BigInt, _>(start)
        .bind::<sql_types::BigInt, _>(end)
        .load(conn)?;
    Ok(edit_timeline)
}