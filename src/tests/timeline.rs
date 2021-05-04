use rocket::http::{ContentType, Status};
use rocket::local::Client;
use serde_json::{json, Value};

use crate::tests::common::{api_key_header, bearer_header, create_sync_client_api_key, get_admin_jwt, random_string, setup, teardown, teardown_api_key};
use validator::HasLen;

#[test]
fn test_timeline() {
    let jwt = setup();
    let admin_jwt = get_admin_jwt();
    let api_key = create_sync_client_api_key(&jwt, 2);

    let client = Client::new(gtm_api::rocket()).unwrap();

    let user = random_string(16);
    let provider = random_string(10);
    let repo = random_string(10);

    let timeline = vec![json!({
        "timestamp": 100,
        "time": 100,
    })];

    let files = vec![json!({
        "path": "/test/a/b/c",
        "status": "m",
        "time_total": 100,
        "added_lines": 50,
        "deleted_lines": 10,
        "timeline": &timeline,
    })];

    let commits = vec![json!({
        "author": "test-author <test@test.test>",
        "branch": "test-branch",
        "message": "test-message",
        "hash": random_string(16),
        "time": 1000,
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

    let mut response = client.get(
        format!("/services/gtm/api/{}-{}-{}/timeline?start={}&end={}&interval={}&timezone={}",
                provider, user, repo, 0, 60 * 60 * 24 * 7, "day", "Europe/Tallinn"))
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let data = body_json.as_array();

    assert!(data.is_some());
    let data = data.unwrap();

    assert_eq!(data.length(), 8);

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}

#[test]
fn test_activity_timeline() {
    let jwt = setup();
    let admin_jwt = get_admin_jwt();
    let api_key = create_sync_client_api_key(&jwt, 2);

    let client = Client::new(gtm_api::rocket()).unwrap();

    let user = random_string(16);
    let provider = random_string(10);
    let repo = random_string(10);

    let timeline = vec![json!({
        "timestamp": 100,
        "time": 100,
    })];

    let files = vec![json!({
        "path": "/test/a/b/c",
        "status": "m",
        "time_total": 100,
        "added_lines": 50,
        "deleted_lines": 10,
        "timeline": &timeline,
    })];

    let commits = vec![json!({
        "author": "test-author <test@test.test>",
        "branch": "test-branch",
        "message": "test-message",
        "hash": random_string(16),
        "time": 1000,
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

    let mut response = client.get(
        format!("/services/gtm/api/{}-{}-{}/activity?start={}&end={}&interval={}&timezone={}",
                provider, user, repo, 0, 60 * 60 * 24 * 7, "day", "Europe/Tallinn"))
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let data = body_json.as_array();

    assert!(data.is_some());
    let data = data.unwrap();

    assert_eq!(data.length(), 24);

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}

#[test]
fn test_subdirs_timeline() {
    let jwt = setup();
    let admin_jwt = get_admin_jwt();
    let api_key = create_sync_client_api_key(&jwt, 2);

    let client = Client::new(gtm_api::rocket()).unwrap();

    let user = random_string(16);
    let provider = random_string(10);
    let repo = random_string(10);

    let timeline = vec![json!({
        "timestamp": 100,
        "time": 100,
    })];

    let files = vec![json!({
        "path": "/test/a/b/c",
        "status": "m",
        "time_total": 100,
        "added_lines": 50,
        "deleted_lines": 10,
        "timeline": &timeline,
    })];

    let commits = vec![json!({
        "author": "test-author <test@test.test>",
        "branch": "test-branch",
        "message": "test-message",
        "hash": random_string(16),
        "time": 1000,
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

    let mut response = client.get(
        format!("/services/gtm/api/{}-{}-{}/subdirs-timeline?start={}&end={}&interval={}&timezone={}&depth={}",
                provider, user, repo, 0, 60 * 60 * 24 * 7, "day", "Europe/Tallinn", 2))
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let data = body_json["data"].as_array();
    let paths = body_json["paths"].as_array();

    assert!(data.is_some());
    assert!(paths.is_some());

    let data = data.unwrap();
    let paths = paths.unwrap();

    assert_eq!(data.length(), 8);
    assert_eq!(paths.length(), 1);

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}
