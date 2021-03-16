use crate::github::resource::{GithubUser, GithubEmail, GithubRepo};

pub async fn fetch_github_user(token: &str) -> Result<GithubUser, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://api.github.com/user")
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?
        .json::<GithubUser>()
        .await
}

pub async fn fetch_emails_from_github(token: &str) -> Result<Vec<GithubEmail>, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://api.github.com/user/emails")
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?
        .json::<Vec<GithubEmail>>()
        .await
}

pub async fn fetch_repos_from_github(token: &str) -> Result<Vec<GithubRepo>, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://api.github.com/user/repos")
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?
        .json::<Vec<GithubRepo>>()
        .await
}