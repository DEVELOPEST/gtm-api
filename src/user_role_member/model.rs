#[derive(Queryable)]
pub struct UserRoleMember {
    pub user: i32,
    pub role: i32,
}