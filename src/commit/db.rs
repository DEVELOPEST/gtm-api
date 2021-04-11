use diesel;
use diesel::Insertable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::commit::model::{Commit};
use crate::commit::routes::NewCommitData;
use crate::file;
use crate::schema::commits;
use crate::errors::Error;
use crate::commit::resource::CommitJson;

lazy_static! {
    static ref GIT_USER_NAME_EMAIL_REGEX: Regex = Regex::new("^(.*)\\s+<(.*)>$").unwrap();
}

#[derive(Insertable)]
#[table_name = "commits"]
struct NewCommit<'a> {
    repository_id: i32,
    email: &'a str,
    git_user_name: &'a str,
    branch: &'a str,
    message: &'a str,
    hash: &'a str,
    timestamp: i64,
}

pub fn find_last_by_repository_id(
    conn: &PgConnection,
    repository_id: i32,
) -> Result<Commit, Error> {
    commits::table
        .filter(commits::repository_id.eq(repository_id))
        .order(commits::timestamp.desc())
        .limit(1)
        .get_result::<Commit>(conn)
        .map_err(Error::DatabaseError)
}


pub fn find_all_by_repository_id(
    conn: &PgConnection,
    repository_id: i32
) -> Result<Vec<String>, Error> {
    commits::table
        .filter(commits::repository_id.eq(repository_id))
        .order(commits::timestamp.desc())
        .select(commits::hash)
        .load::<String>(conn)
        .map_err(Error::DatabaseError)
}

pub fn create_all(
    conn: &PgConnection,
    commits: Vec<NewCommitData>,
    repository_id: i32
) -> Result<Vec<CommitJson>, Error> {
    let mut vec = Vec::new();
    for var in commits {
        let author = var.author.unwrap_or_default();
        let author_matches = GIT_USER_NAME_EMAIL_REGEX.captures(&author).unwrap();
        let git_user_name = author_matches.get(1).unwrap().as_str().to_string();
        let email = author_matches.get(2).unwrap().as_str().to_string();
        let branch =var.branch.unwrap_or_default();
        let message = var.message.unwrap_or_default();
        let hash = var.hash.unwrap_or_default();
        let timestamp = var.time.unwrap_or_default();

        let new_commit = &NewCommit {
            repository_id,
            email: &email,
            git_user_name: &git_user_name,
            branch: &branch,
            message: &message,
            hash: &hash,
            timestamp,
        };

        let commit = diesel::insert_into(commits::table)
            .values(new_commit)
            .get_result::<Commit>(conn)?;

        let files_vec = file::db::create_all(
            &conn,
            var.files,
            commit.id
        )?;

        vec.push(commit.attach(files_vec))
    }
    Ok(vec)
}