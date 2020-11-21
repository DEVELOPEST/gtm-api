// use crate::auth::Auth;
use chrono::{Duration, Utc};
use serde::Serialize;
use diesel::{Queryable};

type Url = String;

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
    pub fn to_user_auth(&self, secret: &[u8]) -> UserAuth {
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

}