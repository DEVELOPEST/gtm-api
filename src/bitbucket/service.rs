use std::time::Duration;

use reqwest::RequestBuilder;

use crate::bitbucket::resource::{BitbucketEmail, BitbucketRepo, BitbucketUser};
use crate::common;

pub const BITBUCKET_ORG_DOMAIN: &str = "bitbucket.org";

pub async fn fetch_bitbucket_user(token: &str) -> Result<BitbucketUser, reqwest::Error> {
    create_get_request(BITBUCKET_ORG_DOMAIN, "/user", token)
        .send()
        .await?
        .json::<BitbucketUser>()
        .await
}

pub async fn fetch_emails_from_bitbucket(token: &str) -> Result<Vec<BitbucketEmail>, reqwest::Error> {
    let resp = create_get_request(BITBUCKET_ORG_DOMAIN, "/user/emails", token)
        .send()
        .await?
        .json::<common::json::Values<BitbucketEmail>>()
        .await?;
    Ok(resp.values)
}

pub async fn fetch_repos_from_bitbucket(token: &str, repo_name: Option<&str>) -> Result<Vec<BitbucketRepo>, reqwest::Error> {
    let name_query = repo_name.map(|n| format!("&q=name~{}", n));
    let resp = create_get_request(
        BITBUCKET_ORG_DOMAIN,
        &format!("/repositories?role=member{}", name_query.unwrap_or("".to_string())),
        token)
        .send()
        .await?
        .json::<common::json::Values<BitbucketRepo>>()
        .await?;
    Ok(resp.values)
}

fn create_get_request(domain: &str, endpoint: &str, token: &str) -> RequestBuilder {
    let client = reqwest::Client::new();
    client.get(&format!("https://{}/api/2.0{}", domain, endpoint))
        .bearer_auth(token)
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(10))
}