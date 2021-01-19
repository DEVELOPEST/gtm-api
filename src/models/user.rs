// use crate::auth::Auth;
use serde::Serialize;
use diesel::{Queryable};
use crate::helpers::jwt::AuthToken;
use crate::db::Conn;


#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    email: &'a str,
    token: String,
}

impl User {
    pub fn to_user_auth(&self, _secret: &[u8]) -> UserAuth {
        // let exp = Utc::now() + Duration::days(60); // TODO: move to config
        // let token = Auth {
        //     id: self.id,
        //     username: self.username.clone(),
        //     exp: exp.timestamp(),
        // }.token(secret);


        let token = "Token".to_string();

        UserAuth {
            email: &self.email,
            token,
        }
    }

    pub fn is_valid_login_session(token: &AuthToken, conn: &Conn) -> bool {
        return true;
    }

}