use std::sync::RwLock;

use lazy_static::lazy_static;
use rocket::fairing::AdHoc;

lazy_static! {
    // This is overridden in Rocket.toml
    pub static ref JWT_SECRET: RwLock<String> = RwLock::new("zRXL2u7hw84MTir+ZMjIGg==".to_string());
    pub static ref API_KEY: RwLock<String> = RwLock::new("".to_string());

    pub static ref LOGIN_REDIRECT: RwLock<String> = RwLock::new("/".to_string());
    pub static ref REGISTER_REDIRECT: RwLock<String> = RwLock::new("/".to_string());

    pub static ref JWT_COOKIE: String = "user_jwt".to_string();
}

pub fn manage() -> AdHoc {
    AdHoc::on_attach("Security config", |rocket| {
        // Rocket doesn't expose it's own secret_key, so we use our own here.
        let cfg = rocket.config();
        let extras = &cfg.extras;

        let secret_value = extras.get("jwt");
        if secret_value.is_some() {
            let secret_table = secret_value.unwrap().as_table().unwrap();
            let secret = secret_table.get("secret").unwrap().as_str().unwrap();
            let mut global_secret = JWT_SECRET.write().unwrap();
            *global_secret = secret.to_string();
        }

        let api_key = extras.get("api-key");
        if api_key.is_some() {
            let api_key_table = api_key.unwrap().as_table().unwrap();
            let key = api_key_table.get("sync_api_key").unwrap().as_str().unwrap();
            let mut global_secret = API_KEY.write().unwrap();
            *global_secret = key.to_string();
        }

        let oauth = extras.get("oauth");
        if oauth.is_some() {
            let oauth_table = oauth.unwrap().as_table().unwrap();

            let login_redirect = oauth_table.get("login_redirect").unwrap().as_str().unwrap();
            let mut global_login_redirect_val = LOGIN_REDIRECT.write().unwrap();
            *global_login_redirect_val = login_redirect.to_string();

            let register_redirect = oauth_table.get("register_redirect").unwrap().as_str().unwrap();
            let mut global_register_redirect_val = REGISTER_REDIRECT.write().unwrap();
            *global_register_redirect_val = register_redirect.to_string();
        }

        Ok(rocket)
    })
}