use crate::models::group_repository::{GroupRepository};
use crate::schema::group_repository_members;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable};

#[derive(Insertable)]
#[table_name = "group_repository_members"]
struct NewGroupRepository<> {
    repository: i32,
    group: i32,
}

pub fn create(
    conn: &PgConnection,
    repository: i32,
    group: i32,
) -> GroupRepository {
    let new_group_repository = &NewGroupRepository {
        repository,
        group,
    };

    diesel::insert_into(group_repository_members::table)
        .values(new_group_repository)
        .get_result::<GroupRepository>(conn)
        .expect("Error creating GroupRepository!")
}

pub fn find(
    conn: &PgConnection,
    repository: i32,
    group: i32,
) -> GroupRepository {
    group_repository_members::table
        .filter(group_repository_members::repository.eq(repository)
            .and(group_repository_members::group.eq(group)))
        .get_result::<GroupRepository>(conn)
        .expect("Cannot load GroupRepository")
}
