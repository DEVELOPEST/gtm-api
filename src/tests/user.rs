use std::thread::sleep;
use std::time::Duration;

use rocket;
use rocket::http::{ContentType, Status};
use serde::Serialize;
use serde_json::{json, Value};

use crate::tests::common::{bearer_header, setup, teardown};
use rocket::local::Client;

#[test]
fn test_authorization_required_user() {
    use rocket::local::Client;

    let client = Client::new(gtm_api::rocket()).unwrap();
    let response = client.get("/services/gtm/api/user").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_authorization_required_users() {
    use rocket::local::Client;

    let client = Client::new(gtm_api::rocket()).unwrap();
    let response = client.get("/services/gtm/api/users").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[derive(Serialize)]
pub struct RegisterDto<'a> {
    username: &'a str,
    password: &'a str,
}

#[test]
fn test_authorization_register() {
    let jwt = setup();
    // This is all :P
    teardown(&jwt);
}

#[test]
fn test_login() {
    let client = Client::new(gtm_api::rocket()).unwrap();
    let response = client.post("/services/gtm/api/auth/register")
        .header(ContentType::JSON)
        .body(json!({
            "username": "some-user-name",
            "password": "StrongPass123"
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let mut response = client.post("/services/gtm/api/auth/login")
        .header(ContentType::JSON)
        .body(json!({
            "username": "some-user-name",
            "password": "StrongPass123"
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let jwt_val: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let jwt = jwt_val["jwt"].as_str().unwrap().to_string();

    teardown(&jwt)
}

#[test]
fn test_get_user_id() {
    let jwt = setup();

    let client = Client::new(gtm_api::rocket()).unwrap();
    let mut response = client.get("/services/gtm/api/user")
        .header(bearer_header(&jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let user: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let id = user["user_id"].as_i64();

    assert!(id.is_some());
    assert!(id.unwrap() > 0);

    teardown(&jwt);
}