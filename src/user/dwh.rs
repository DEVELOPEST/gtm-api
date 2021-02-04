
#[derive(Queryable, Debug)]
pub struct UserDWH {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub role: String,
}