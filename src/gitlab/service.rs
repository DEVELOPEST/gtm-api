use crate::gitlab::resource::{GitlabUser, GitlabEmail, GitlabRepo};

pub async fn fetch_gitlab_user(token: &str) -> Result<GitlabUser, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://gitlab.com/api/v4/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<GitlabUser>()
        .await
}

pub async fn fetch_emails_from_gitlab(token: &str) -> Result<Vec<GitlabEmail>, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://gitlab.com/api/v4/user/emails")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<Vec<GitlabEmail>>()
        .await
}

pub async fn fetch_repos_from_gitlab(token: &str) -> Result<Vec<GitlabRepo>, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://gitlab.com/api/v4/projects?membership=true&min_access_level=30")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<Vec<GitlabRepo>>()
        .await
}