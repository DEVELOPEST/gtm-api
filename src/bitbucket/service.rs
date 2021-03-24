use crate::bitbucket::resource::{BitbucketUser, BitbucketEmail, BitbucketRepo};
use crate::common;
use std::time::Duration;

pub async fn fetch_bitbucket_user(token: &str) -> Result<BitbucketUser, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://bitbucket.org/api/2.0/user")
        .bearer_auth(token)
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(10))
        .send()
        .await?
        .json::<BitbucketUser>()
        .await
}

pub async fn fetch_emails_from_bitbucket(token: &str) -> Result<Vec<BitbucketEmail>, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client.get("https://bitbucket.org/api/2.0/user/emails")
        .bearer_auth(token)
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(10))
        .send()
        .await?
        .json::<common::json::Values<BitbucketEmail>>()
        .await?;
    Ok(resp.values)
}

pub async fn fetch_repos_from_bitbucket(token: &str) -> Result<Vec<BitbucketRepo>, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client.get("https://bitbucket.org/api/2.0/repositories?role=member")
        .bearer_auth(token)
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(10))
        .send()
        .await?
        .json::<common::json::Values<BitbucketRepo>>()
        .await?;
    Ok(resp.values)
}