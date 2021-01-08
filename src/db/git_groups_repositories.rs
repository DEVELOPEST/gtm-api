use crate::models::git_group_repository::{GitGroupRepository};
use crate::schema::git_group_repository_members;
use crate::db;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};

#[derive(Insertable)]
#[table_name = "git_group_repository_members"]
struct NewGitGroupRepository<> {
    repository: i32,
    git_group: i32,
}

pub fn create(
    conn: &PgConnection,
    repository: i32,
    git_group: i32,
) -> GitGroupRepository {
    let new_git_group_repository = &NewGitGroupRepository {
        repository,
        git_group,
    };

    diesel::insert_into(git_group_repository_members::table)
        .values(new_git_group_repository)
        .get_result::<GitGroupRepository>(conn)
        .expect("Error creating GitGroupRepository!")
}

pub fn find(
    conn: &PgConnection,
    repository: i32,
    git_group: i32,
) -> GitGroupRepository {
    git_group_repository_members::table
        .filter(git_group_repository_members::repository.eq(repository)
            .and(git_group_repository_members::git_group.eq(git_group)))
        .get_result::<GitGroupRepository>(conn)
        .expect("Cannot load GitGroupRepository")
}
