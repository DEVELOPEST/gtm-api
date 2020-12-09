use crate::models::repository::{Repository, RepositoryJson};
use crate::schema::repositories;
use crate::routes::commits::NewCommitData;
use crate::db;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};


#[derive(Insertable)]
#[table_name = "repositories"]
struct NewRepository<'a> {
    url: &'a str,
    sync_url: &'a str,
    access_token: &'a str,
}

pub fn create(
    conn: &PgConnection,
    url: &str,
    sync_url: &str,
    access_token: &str,
    commits: Vec<NewCommitData>,
) -> RepositoryJson {
    let new_repository = &NewRepository {
        url,
        sync_url,
        access_token,
    };

    let repo = diesel::insert_into(repositories::table)
        .values(new_repository)
        .get_result::<Repository>(conn)
        .expect("Error creating repository");

    let commits_vec = db::commits::create_all(
        &conn,
        commits,
        repo.id
    );
    repo.attach(commits_vec)
}