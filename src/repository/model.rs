use chrono::{DateTime, Utc};

use crate::schema::repositories;

#[derive(Queryable, QueryableByName, Identifiable)]
#[table_name = "repositories"]
pub struct Repository {
    pub id: i32,
    pub group: i32,
    pub user: String,
    pub provider: String,
    pub repo: String,
    pub added_at: DateTime<Utc>,
    pub sync_client: Option<i32>,
}
