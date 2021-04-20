use rocket::http::{Status, ContentType};

#[test]
fn test_context_loads() {
    use rocket::local::Client;

    let client = Client::new(gtm_api::rocket()).unwrap();
    let response = client.get("/services/gtm/api/swagger/swagger-ui-config.json").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}