use rocket::http::{ContentType, Status};
use rocket::local::Client;
use serde_json::{json, Value};

use crate::tests::common::{api_key_header, create_sync_client_api_key, random_string, setup, teardown, teardown_api_key};

#[test]
fn test_get_last_commit_hash() {
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
        "hash": "abcdefghijklmnopqrstuvw",
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

    let mut response = client.get(format!("/services/gtm/api/commits/{}/{}/{}/hash", provider, user, repo))
        .header(api_key_header(&api_key))
        .header(ContentType::JSON)
        .dispatch();

    let body_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let hash = body_json["hash"].as_str().unwrap().to_string();
    let timestamp = body_json["timestamp"].as_i64().unwrap();

    assert_eq!(hash, "abcdefghijklmnopqrstuvw");
    assert_eq!(timestamp, 123456789);

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}