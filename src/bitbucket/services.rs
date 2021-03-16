use crate::bitbucket::resource::{BitbucketUser, BitbucketEmail, BitbucketRepo};
use crate::common;

pub async fn fetch_bitbucket_user(token: &str) -> Result<BitbucketUser, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://api.bitbucket.org/2.0/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<BitbucketUser>()
        .await
}

pub async fn fetch_emails_from_bitbucket(token: &str) -> Result<Vec<BitbucketEmail>, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client.get("https://api.bitbucket.org/2.0/user/emails")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<common::json::Values<BitbucketEmail>>()
        .await?;
    Ok(resp.values)
}

pub async fn fetch_repos_from_bitbucket(token: &str) -> Result<Vec<BitbucketRepo>, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client.get("https://api.bitbucket.org/2.0/repositories?role=member")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<common::json::Values<BitbucketRepo>>()
        .await?;
    Ok(resp.values)
}