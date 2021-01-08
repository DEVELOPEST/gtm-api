use crate::models::git_group::{GitGroup, GitGroupJson};
use crate::schema::git_groups;
use crate::db;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};

#[derive(Insertable)]
#[table_name = "git_groups"]
struct NewGitGroup<'a> {
    name: &'a str,
}

pub fn create(
    conn: &PgConnection,
    name: &str,
) -> GitGroup {
    let new_git_group = &NewGitGroup {
        name,
    };

    let git_group = diesel::insert_into(git_groups::table)
        .values(new_git_group)
        .get_result::<GitGroup>(conn)
        .expect("Error creating git group");

    git_group
}


pub fn exists(conn: &PgConnection, name: &str) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(git_groups::table
        .filter(git_groups::name.eq(name))))
        .get_result(conn)
        .expect("Error finding git group")
}

pub fn find(conn: &PgConnection, name: &str) -> GitGroup {
    git_groups::table
        .filter(git_groups::name.eq(name))
        .get_result::<GitGroup>(conn)
        .expect("Cannot load repository")
}