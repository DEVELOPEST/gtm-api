use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rocket::http::{ContentType, Header, Status};
use rocket::local::Client;
use serde_json::{json, Value};

pub fn random_string(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect::<String>()
}

pub fn bearer_header(token: &str) -> Header<'static> {
    Header::new("Authorization", format!("Bearer {}", token))
}

pub fn api_key_header(key: &str) -> Header<'static> {
    Header::new("API-Key", key.to_string())
}

pub fn setup() -> String {
    let client = Client::new(gtm_api::rocket()).unwrap();
    let mut response = client.post("/services/gtm/api/auth/register")
        .header(ContentType::JSON)
        .body(json!({
            "username": format!("test-user-{}", random_string(32)),
            "password": random_string(32)
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let jwt_val: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    jwt_val["jwt"].as_str().unwrap().to_string()
}

pub fn get_admin_jwt() -> String {
    let client = Client::new(gtm_api::rocket()).unwrap();
    let mut response = client.post("/services/gtm/api/auth/login")
        .header(ContentType::JSON)
        .body(json!({
            "username": "admin@admin",
            "password": "password",
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let jwt_val: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    jwt_val["jwt"].as_str().unwrap().to_string()
}

pub fn teardown(jwt: &str) {
    let client = Client::new(gtm_api::rocket()).unwrap();
    let response = client.delete("/services/gtm/api/auth/account")
        .header(bearer_header(jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

pub fn create_sync_client_api_key(jwt: &str, client_type: i32) -> String {
    let client = Client::new(gtm_api::rocket()).unwrap();
    let mut response = client.post("/services/gtm/api/sync/client")
        .header(bearer_header(jwt))
        .header(ContentType::JSON)
        .body(json!({
            "base_url": format!("http:/localhost/test-{}", random_string(10)),
            "client_type": client_type
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let api_key_val: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    api_key_val["api_key"].as_str().unwrap().to_string()
}

pub fn teardown_api_key(jwt: &str, api_key: &str) {
    let client = Client::new(gtm_api::rocket()).unwrap();
    let response = client.delete("/services/gtm/api/sync/client")
        .header(bearer_header(jwt))
        .header(api_key_header(api_key))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

