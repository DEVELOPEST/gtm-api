use diesel::sql_types::Integer;

#[derive(QueryableByName)]
pub struct RepositoryId {
    #[sql_type = "Integer"]
    pub id: i32,
}