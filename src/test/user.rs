use serde::Serialize;
use serde_json;
use rocket::http::{Status, ContentType};
use rocket;

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
    use rocket::local::Client;

    let client = Client::new(gtm_api::rocket()).unwrap();
    let mut response = client.post("/services/gtm/api/auth/register")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&RegisterDto {
            username: "test-user",
            password: "PassIsStrong123"
        }).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    println!("{}", response.body_string().unwrap()) // TODO: Validate?
    //TODO: different db
}