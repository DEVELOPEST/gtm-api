use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct GroupAccess {
    pub user: i32,
    pub group: i32,
    pub access_level_recursive: bool,
}