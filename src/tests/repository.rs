use rocket::http::{ContentType, Status};
use rocket::local::Client;
use serde_json::{json, Value};

use crate::tests::common::{api_key_header, create_sync_client_api_key, random_string, setup, teardown, teardown_api_key, bearer_header, get_admin_jwt};

#[test]
fn test_create_repository() {
    let jwt = setup();
    let api_key = create_sync_client_api_key(&jwt, 2);

    let client = Client::new(gtm_api::rocket()).unwrap();

    let user = random_string(16);
    let provider = random_string(10);
    let repo = random_string(10);

    let timeline = vec![json!({
        "timestamp": 123456789,
        "time": 123,
    })];

    let files = vec![json!({
        "path": "/test/a/b/c",
        "status": "m",
        "time_total": 123,
        "added_lines": 123,
        "deleted_lines": 12,
        "timeline": &timeline,
    })];

    let commits = vec![json!({
        "author": "test-author <test@test.test>",
        "branch": "test-branch",
        "message": "test-message",
        "hash": random_string(16),
        "time": 123456789,
        "files": &files
    })];

    let response = client.post("/services/gtm/api/repositories")
        .header(api_key_header(&api_key))
        .header(ContentType::JSON)
        .body(json!({
            "repository": {
                "user": &user,
                "provider": &provider,
                "repo": &repo,
                "commits": &commits,
            }
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}


#[test]
fn test_update_repository() {
    let jwt = setup();
    let api_key = create_sync_client_api_key(&jwt, 2);

    let client = Client::new(gtm_api::rocket()).unwrap();

    let user = random_string(16);
    let provider = random_string(10);
    let repo = random_string(10);

    let timeline = vec![json!({
        "timestamp": 123456789,
        "time": 123,
    })];

    let files = vec![json!({
        "path": "/test/a/b/c",
        "status": "m",
        "time_total": 123,
        "added_lines": 123,
        "deleted_lines": 12,
        "timeline": &timeline,
    })];

    let commits = vec![json!({
        "author": "test-author <test@test.test>",
        "branch": "test-branch",
        "message": "test-message",
        "hash": random_string(16),
        "time": 123456789,
        "files": &files
    })];

    let response = client.post("/services/gtm/api/repositories")
        .header(api_key_header(&api_key))
        .header(ContentType::JSON)
        .body(json!({
            "repository": {
                "user": &user,
                "provider": &provider,
                "repo": &repo,
                "commits": &commits,
            }
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response = client.put("/services/gtm/api/repositories")
        .header(api_key_header(&api_key))
        .header(ContentType::JSON)
        .body(json!({
            "repository": {
                "user": &user,
                "provider": &provider,
                "repo": &repo,
                "commits": &commits,
            }
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}

#[test]
fn test_delete_repository() {
    let jwt = setup();
    let admin_jwt = get_admin_jwt();
    let api_key = create_sync_client_api_key(&jwt, 2);

    let client = Client::new(gtm_api::rocket()).unwrap();

    let user = random_string(16);
    let provider = random_string(10);
    let repo = random_string(10);

    let timeline = vec![json!({
        "timestamp": 123456789,
        "time": 123,
    })];

    let files = vec![json!({
        "path": "/test/a/b/c",
        "status": "m",
        "time_total": 123,
        "added_lines": 123,
        "deleted_lines": 12,
        "timeline": &timeline,
    })];

    let commits = vec![json!({
        "author": "test-author <test@test.test>",
        "branch": "test-branch",
        "message": "test-message",
        "hash": random_string(16),
        "time": 123456789,
        "files": &files
    })];

    let mut response = client.post("/services/gtm/api/repositories")
        .header(api_key_header(&api_key))
        .header(ContentType::JSON)
        .body(json!({
            "repository": {
                "user": &user,
                "provider": &provider,
                "repo": &repo,
                "commits": &commits,
            }
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let id = body_json["id"].as_i64().unwrap();

    let response = client.delete(format!("/services/gtm/api/repositories/{}", id))
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}

#[test]
fn test_delete_repository_no_access() {
    let jwt = setup();
    let api_key = create_sync_client_api_key(&jwt, 2);

    let client = Client::new(gtm_api::rocket()).unwrap();

    let user = random_string(16);
    let provider = random_string(10);
    let repo = random_string(10);

    let timeline = vec![json!({
        "timestamp": 123456789,
        "time": 123,
    })];

    let files = vec![json!({
        "path": "/test/a/b/c",
        "status": "m",
        "time_total": 123,
        "added_lines": 123,
        "deleted_lines": 12,
        "timeline": &timeline,
    })];

    let commits = vec![json!({
        "author": "test-author <test@test.test>",
        "branch": "test-branch",
        "message": "test-message",
        "hash": random_string(16),
        "time": 123456789,
        "files": &files
    })];

    let mut response = client.post("/services/gtm/api/repositories")
        .header(api_key_header(&api_key))
        .header(ContentType::JSON)
        .body(json!({
            "repository": {
                "user": &user,
                "provider": &provider,
                "repo": &repo,
                "commits": &commits,
            }
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let id = body_json["id"].as_i64().unwrap();

    let response = client.delete(format!("/services/gtm/api/repositories/{}", id))
        .header(bearer_header(&jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}