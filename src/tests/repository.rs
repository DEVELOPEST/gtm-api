use crate::tests::common::{setup, teardown, create_sync_client_api_key, teardown_api_key};

#[test]
fn test_create_repository() {
    let jwt = setup();
    let api_key = create_sync_client_api_key(&jwt, 2);

    // TODO: Actual test

    teardown_api_key(&jwt, &api_key);
    teardown(&jwt);
}