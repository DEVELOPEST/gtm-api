use crate::gitlab::resource::{GitlabUser, GitlabEmail, GitlabRepo};
use reqwest::RequestBuilder;

pub const GITLAB_COM_DOMAIN: &str = "gitlab.com";
pub const GITLAB_TALTECH_DOMAIN: &str = "gitlab.cs.ttu.ee";

pub async fn fetch_gitlab_user(token: &str, domain: &str) -> Result<GitlabUser, reqwest::Error> {
    create_get_request(domain, "/user", token).send().await?
        .json::<GitlabUser>().await
}

pub async fn fetch_emails_from_gitlab(token: &str, domain: &str) -> Result<Vec<GitlabEmail>, reqwest::Error> {
    create_get_request(domain, "/user/emails", token).send().await?
        .json::<Vec<GitlabEmail>>().await
}

pub async fn fetch_repos_from_gitlab(token: &str, domain: &str) -> Result<Vec<GitlabRepo>, reqwest::Error> {
    create_get_request(domain, "/projects?membership=true&min_access_level=30&statistics=true", token).send().await?
        .json::<Vec<GitlabRepo>>().await
}

fn create_get_request(domain: &str, endpoint: &str, token: &str) -> RequestBuilder {
    let client = reqwest::Client::new();
    client.get(&format!("https://{}/api/v4{}", domain, endpoint))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/json")
}