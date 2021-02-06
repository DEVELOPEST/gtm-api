use chrono::{DateTime, Utc};
use serde::Serialize;
use diesel::sql_types::{Integer, Text, Timestamptz};
use crate::config::DATE_FORMAT;


#[derive(QueryableByName, Queryable, Serialize, Debug)]
pub struct Group {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Timestamptz"]
    pub added_at: DateTime<Utc>,
}

impl Group {
    pub fn attach(self) -> GroupJson {
        GroupJson {
            id: self.id,
            name: self.name,
            added_at: self.added_at.format(DATE_FORMAT).to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupJson {
    pub id: i32,
    pub name: String,
    pub added_at: String,
}
