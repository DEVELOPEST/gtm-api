use async_trait::async_trait;
use rocket_oauth2::TokenResponse;

use serde::Deserialize;
use reqwest::Error;

#[async_trait]
pub trait LoginType {
    fn get_login_type(&self) -> i32;
    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error>;
    async fn fetch_username(&self) -> Result<String, reqwest::Error>;
}

pub struct GitHub;
pub struct GitLab;

#[async_trait]
impl LoginType for TokenResponse<GitHub> {
    fn get_login_type(&self) -> i32 {
        return 1;
    }

    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error> {
        let user = fetch_github_user(&self.access_token()).await?;
        Ok(user.get_identity_hash().to_string())
    }

    async fn fetch_username(&self) -> Result<String, Error> {
        let user = fetch_github_user(&self.access_token()).await?;
        Ok(user.login)
    }
}

#[async_trait]
impl LoginType for TokenResponse<GitLab> {
    fn get_login_type(&self) -> i32 {
        return 2;
    }

    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error> {
        let user = fetch_gitlab_user(&self.access_token()).await?;
        Ok(user.get_identity_hash().to_string())
    }

    async fn fetch_username(&self) -> Result<String, Error> {
        let user = fetch_gitlab_user(&self.access_token()).await?;
        Ok(user.username)
    }
}

pub trait IdentityUser {
    fn get_identity_hash(&self) -> String;
}

#[derive(Deserialize)]
pub struct GithubUser {
    pub login: String,
    // pub id: i64,
    pub node_id: String,
}

#[derive(Deserialize)]
pub struct GitlabUser {
    pub id: i64,
    pub username: String
}

impl IdentityUser for GithubUser {
    fn get_identity_hash(&self) -> String {
        return self.node_id.clone();
    }
}

impl IdentityUser for GitlabUser {
    fn get_identity_hash(&self) -> String {
        return self.id.to_string();
    }
}

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