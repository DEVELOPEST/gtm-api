use crate::config::DATE_FORMAT;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable)]
pub struct GitGroup {
    pub id: i32,
    pub name: String,
    pub added_at: DateTime<Utc>,
}

impl GitGroup {
    pub fn attach(self) -> GitGroupJson {
        GitGroupJson {
            id: self.id,
            name: self.name,
            added_at: self.added_at.format(DATE_FORMAT).to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitGroupJson {
    pub id: i32,
    pub name: String,
    pub added_at: String,
}
