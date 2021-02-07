use chrono::{DateTime, Utc};
use serde::Serialize;
use diesel::sql_types::{Integer, Text, Timestamptz};
use crate::config::DATE_FORMAT;
use crate::group_access::model::GroupAccessJson;


#[derive(QueryableByName, Queryable, Serialize, Debug, Clone)]
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

impl Group {
    pub fn attach_with_access(self, group_access: Option<GroupAccessJson>) -> GroupWithAccessJson {
        GroupWithAccessJson {
            id: self.id,
            name: self.name,
            added_at: self.added_at.format(DATE_FORMAT).to_string(),
            group_access
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupWithAccessJson {
    pub id: i32,
    pub name: String,
    pub added_at: String,
    pub group_access: Option<GroupAccessJson>
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}