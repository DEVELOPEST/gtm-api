
#[derive(Queryable, Debug)]
pub struct UserDWH {
    pub id: i32,
    pub email: String,
    pub password: Option<String>,
    pub role: String,
}