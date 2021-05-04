use schemars::JsonSchema;
use serde::Serialize;

use crate::domain::user_role_member::model::UserRoleMember;

#[derive(Serialize, JsonSchema)]
pub struct UserRoleMemberJson {
    pub user: i32,
    pub role: i32,
}

impl From<UserRoleMember> for UserRoleMemberJson {
    fn from(user_role_member: UserRoleMember) -> Self {
        UserRoleMemberJson {
            user: user_role_member.user,
            role: user_role_member.role,
        }
    }
}