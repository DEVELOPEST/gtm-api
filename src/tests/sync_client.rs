use rocket::http::{ContentType, Status};
use rocket::local::Client;
use serde_json::{json};

use crate::tests::common::{bearer_header, create_sync_client_api_key, setup, teardown, teardown_api_key};
use crate::tests::common::random_string;

#[test]
fn test_create_private_sync_client() {
    let jwt = setup();
    let api_key = create_sync_client_api_key(&jwt, 2);
    assert!(api_key.len() > 16);

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}

#[test]
fn test_create_public_sync_client() {
    let jwt = setup();
    let api_key = create_sync_client_api_key(&jwt, 1);
    assert!(api_key.len() > 16);

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}

#[test]
fn test_create_illegal_sync_client() {
    let jwt = setup();
    let client = Client::new(gtm_api::rocket()).unwrap();
    let response = client.post("/services/gtm/api/sync/client")
        .header(bearer_header(&jwt))
        .header(ContentType::JSON)
        .body(json!({
            "base_url": format!("http:/localhost/test-{}", random_string(10)),
            "client_type": 999
        }).to_string())
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    teardown(&jwt);
}