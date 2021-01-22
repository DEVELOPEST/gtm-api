use crate::models::commit::{Commit, CommitJson};
use crate::schema::commits;
use crate::errors::{FieldValidator};
use crate::routes::commits::NewCommitData;
use crate::db;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};

#[derive(Insertable)]
#[table_name = "commits"]
struct NewCommit<'a> {
    repository_id: i32,
    email: &'a str,
    branch: &'a str,
    message: &'a str,
    hash: &'a str,
    timestamp: i64,
}

pub fn find_last_by_repository_id(
    conn: &PgConnection,
    repository_id: i32
) -> Commit {
    commits::table
        .filter(commits::repository_id.eq(repository_id))
        .order(commits::timestamp.desc())
        .limit(1)
        .get_result::<Commit>(conn)
        .expect("Cannot load commit")
}


pub fn find_all_by_repository_id(
    conn: &PgConnection,
    repository_id: i32
) -> Vec<String> {
    commits::table
        .filter(commits::repository_id.eq(repository_id))
        .order(commits::timestamp.desc())
        .select(commits::hash)
        .load::<String>(conn)
        .expect("Cannot load commit")
}

pub fn create_all(
    conn: &PgConnection,
    commits: Vec<NewCommitData>,
    repository_id: i32
) -> Vec<CommitJson> {
    let mut vec = Vec::new();
    for var in commits {
        let mut extractor = FieldValidator::validate(&var);
        let email = &extractor.extract("author", var.author);
        let branch = &extractor.extract("branch", var.branch);
        let message = &extractor.extract("message", var.message);
        let hash = &extractor.extract("hash", var.hash);
        let timestamp = extractor.extract("timestamp", var.time);

        let new_commit = &NewCommit {
            repository_id,
            email,
            branch,
            message,
            hash,
            timestamp,
        };

        let commit = diesel::insert_into(commits::table)
            .values(new_commit)
            .get_result::<Commit>(conn)
            .expect("Error creating commit");

        let files_vec = db::files::create_all(
            &conn,
            var.files,
            commit.id
        );

        vec.push(commit.attach(files_vec))
    }

    vec
}