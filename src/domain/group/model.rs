use chrono::{DateTime, Utc};
use diesel::sql_types::{Integer, Text, Timestamptz};

use crate::config::DATE_FORMAT;
use crate::domain::group::resource::{GroupJson, GroupWithAccessJson};
use crate::domain::group_access::resource::GroupAccessJson;

#[derive(QueryableByName, Queryable, Debug, Clone)]
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

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}