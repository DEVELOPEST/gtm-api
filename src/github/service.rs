use reqwest::RequestBuilder;

use crate::github::resource::{GithubEmail, GithubRepo, GithubUser};

pub const GITHUB_COM_DOMAIN: &str = "api.github.com";

pub async fn fetch_github_user(token: &str) -> Result<GithubUser, reqwest::Error> {
        create_get_request(GITHUB_COM_DOMAIN, "/user", token)
        .send()
        .await?
        .json::<GithubUser>()
        .await
}

pub async fn fetch_emails_from_github(token: &str) -> Result<Vec<GithubEmail>, reqwest::Error> {
        create_get_request(GITHUB_COM_DOMAIN, "/user/emails", token)
        .send()
        .await?
        .json::<Vec<GithubEmail>>()
        .await
}

pub async fn fetch_repos_from_github(token: &str, repo_name: Option<&str>) -> Result<Vec<GithubRepo>, reqwest::Error> {
    Ok(create_get_request(GITHUB_COM_DOMAIN, "/user/repos?sort=updated&per_page=100", token)
        .send()
        .await?
        .json::<Vec<GithubRepo>>()
        .await?
        .into_iter() // TODO: Upgrade to github search api if possible
        .filter(|r| repo_name.is_none() || r.full_name.contains(repo_name.unwrap()))
        .collect())
}

fn create_get_request(domain: &str, endpoint: &str, token: &str) -> RequestBuilder {
    let client = reqwest::Client::new();
    client.get(&format!("https://{}{}", domain, endpoint))
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/vnd.github.v3+json")
}