use crate::domain::group_access::resource::GroupAccessJson;

#[derive(Queryable, Debug, Clone)]
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
