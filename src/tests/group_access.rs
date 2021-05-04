use crate::tests::common::{setup, get_admin_jwt, create_sync_client_api_key, random_string, api_key_header, bearer_header, teardown_api_key, teardown};
use rocket::local::Client;
use rocket::http::{ContentType, Status};
use serde_json::{json, Value};

#[test]
fn test_group_access_timeline() {
    let jwt = setup();
    let admin_jwt = get_admin_jwt();
    let api_key = create_sync_client_api_key(&jwt, 2);

    let client = Client::new(gtm_api::rocket()).unwrap();

    let user = random_string(16);
    let provider = random_string(10);
    let repo = random_string(10);


    let response = client.post("/services/gtm/api/repositories")
        .header(api_key_header(&api_key))
        .header(ContentType::JSON)
        .body(json!({
            "repository": {
                "user": &user,
                "provider": &provider,
                "repo": &repo,
                "commits": Vec::<String>::new(),
            }
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let mut response = client.get("/services/gtm/api/user")
        .header(bearer_header(&jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let user_val: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let id = user_val["user_id"].as_i64();
    assert!(id.is_some());
    let id = id.unwrap();

    let mut response = client.get("/services/gtm/api/groups")
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let group: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let groups = group.as_array();
    assert!(groups.is_some());
    let groups = groups.unwrap();

    let group = groups.iter()
        .map(|g| (g["id"].as_i64().unwrap(), g["name"].as_str().unwrap().to_string()))
        .find(|(_, name)|  *name == format!("{}-{}-{}", provider, user, repo));

    assert!(group.is_some());
    let group = group.unwrap();

    let response = client.post("/services/gtm/api/group_accesses")
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .body(json!([
            {
                "user": id,
                "group": group.0,
                "access_level_recursive": true,
            }
        ]).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let mut response = client.get(
        format!("/services/gtm/api/{}-{}-{}/timeline?start={}&end={}&interval={}&timezone={}",
                provider, user, repo, 0, 60 * 60 * 24 * 7, "day", "Europe/Tallinn"))
        .header(bearer_header(&&jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let data = body_json.as_array();

    assert!(data.is_some());
    let data = data.unwrap();

    assert_eq!(data.len(), 8);

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}

#[test]
fn test_group_access_delete_no_access_to_timeline() {
    let jwt = setup();
    let admin_jwt = get_admin_jwt();
    let api_key = create_sync_client_api_key(&jwt, 2);

    let client = Client::new(gtm_api::rocket()).unwrap();

    let user = random_string(16);
    let provider = random_string(10);
    let repo = random_string(10);


    let response = client.post("/services/gtm/api/repositories")
        .header(api_key_header(&api_key))
        .header(ContentType::JSON)
        .body(json!({
            "repository": {
                "user": &user,
                "provider": &provider,
                "repo": &repo,
                "commits": Vec::<String>::new(),
            }
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let mut response = client.get("/services/gtm/api/user")
        .header(bearer_header(&jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let user_val: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let id = user_val["user_id"].as_i64();
    assert!(id.is_some());
    let id = id.unwrap();

    let mut response = client.get("/services/gtm/api/groups")
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let group: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let groups = group.as_array();
    assert!(groups.is_some());
    let groups = groups.unwrap();

    let group = groups.iter()
        .map(|g| (g["id"].as_i64().unwrap(), g["name"].as_str().unwrap().to_string()))
        .find(|(_, name)|  *name == format!("{}-{}-{}", provider, user, repo));

    assert!(group.is_some());
    let group = group.unwrap();

    let response = client.post("/services/gtm/api/group_accesses")
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .body(json!([
            {
                "user": id,
                "group": group.0,
                "access_level_recursive": true,
            }
        ]).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response = client.delete("/services/gtm/api/group_accesses")
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .body(json!([
            {
                "user": id,
                "group": group.0,
            }
        ]).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response = client.get(
        format!("/services/gtm/api/{}-{}-{}/timeline?start={}&end={}&interval={}&timezone={}",
                provider, user, repo, 0, 60 * 60 * 24 * 7, "day", "Europe/Tallinn"))
        .header(bearer_header(&&jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}

#[test]
fn test_group_access_delete() {
    let jwt = setup();
    let admin_jwt = get_admin_jwt();
    let api_key = create_sync_client_api_key(&jwt, 2);

    let client = Client::new(gtm_api::rocket()).unwrap();

    let user = random_string(16);
    let provider = random_string(10);
    let repo = random_string(10);


    let response = client.post("/services/gtm/api/repositories")
        .header(api_key_header(&api_key))
        .header(ContentType::JSON)
        .body(json!({
            "repository": {
                "user": &user,
                "provider": &provider,
                "repo": &repo,
                "commits": Vec::<String>::new(),
            }
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let mut response = client.get("/services/gtm/api/user")
        .header(bearer_header(&jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let user_val: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let id = user_val["user_id"].as_i64();
    assert!(id.is_some());
    let id = id.unwrap();

    let mut response = client.get("/services/gtm/api/groups")
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let group: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let groups = group.as_array();
    assert!(groups.is_some());
    let groups = groups.unwrap();

    let group = groups.iter()
        .map(|g| (g["id"].as_i64().unwrap(), g["name"].as_str().unwrap().to_string()))
        .find(|(_, name)|  *name == format!("{}-{}-{}", provider, user, repo));

    assert!(group.is_some());
    let group = group.unwrap();

    let response = client.post("/services/gtm/api/group_accesses")
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .body(json!([
            {
                "user": id,
                "group": group.0,
                "access_level_recursive": true,
            }
        ]).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let mut response = client.get(format!("/services/gtm/api/groups/accessible/user/{}", id))
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let data = body_json.as_array();

    assert!(data.is_some());

    let data = data.unwrap();

    assert_eq!(data.len(), 1);

    let response = client.delete("/services/gtm/api/group_accesses")
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .body(json!([
            {
                "user": id,
                "group": group.0,
            }
        ]).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let mut response = client.get(format!("/services/gtm/api/groups/accessible/user/{}", id))
        .header(bearer_header(&admin_jwt))
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let data = body_json.as_array();

    assert!(data.is_some());

    let data = data.unwrap();

    assert_eq!(data.len(), 0);

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}
