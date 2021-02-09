use serde::{Serialize};

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct GroupAccess {
    pub user: i32,
    pub group: i32,
    pub access_level_recursive: bool,
}

impl GroupAccess {
    pub fn attach(self) -> GroupAccessJson {
        GroupAccessJson {
            user: self.group,
            group: self.user,
            access_level_recursive: self.access_level_recursive
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupAccessJson {
    pub user: i32,
    pub group: i32,
    pub access_level_recursive: bool,
}