use async_trait::async_trait;
use reqwest::Error;
use rocket_oauth2::TokenResponse;
use serde::Deserialize;

use crate::common;

#[async_trait]
pub trait LoginType {
    fn get_login_type(&self) -> i32;
    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error>;
    async fn fetch_username(&self) -> Result<String, reqwest::Error>;
    async fn fetch_emails(&self) -> Result<Vec<String>, reqwest::Error>;
}

pub struct GitHub;
pub struct GitLab;
pub struct Microsoft;

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

    async fn fetch_emails(&self) -> Result<Vec<String>, Error> {
        let emails_res = fetch_emails_from_github(&self.access_token()).await?;
        let emails = emails_res.iter()
            .filter(|email| email.verified)
            .map(|email| email.email.clone())
            .collect();
        Ok(emails)
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

    async fn fetch_emails(&self) -> Result<Vec<String>, Error> {
        let emails_res = fetch_emails_from_gitlab(&self.access_token()).await?;
        let emails = emails_res.iter().map(|email| email.email.clone()).collect();
        Ok(emails)
    }
}

#[async_trait]
impl LoginType for TokenResponse<Microsoft> {
    fn get_login_type(&self) -> i32 {
        return 3;
    }

    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error> {
        let user = fetch_microsoft_user(&self.access_token()).await?;
        Ok(user.get_identity_hash().to_string())
    }

    async fn fetch_username(&self) -> Result<String, Error> {
        let user = fetch_microsoft_user(&self.access_token()).await?;
        Ok(user.display_name.to_string())
    }

    async fn fetch_emails(&self) -> Result<Vec<String>, Error> {
        let emails_wrapper = fetch_emails_from_microsoft(&self.access_token()).await?;
        let emails = emails_wrapper.value.iter().map(|email| email.address.clone()).collect();
        Ok(emails)
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

#[derive(Deserialize)]
pub struct MicrosoftUser {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub id: String,
}

#[derive(Deserialize)]
pub struct MicrosoftEmail {
    pub address: String,
}

#[derive(Deserialize)]
pub struct GitlabEmail {
    pub email: String,
}

#[derive(Deserialize)]
pub struct GithubEmail {
    pub email: String,
    pub verified: bool,
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

impl IdentityUser for MicrosoftUser {
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

pub async fn fetch_microsoft_user(token: &str) -> Result<MicrosoftUser, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://graph.microsoft.com/v1.0/me")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<MicrosoftUser>()
        .await
}

pub async fn fetch_emails_from_microsoft(token: &str) -> Result<common::json::Value<Vec<MicrosoftEmail>>, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://graph.microsoft.com/beta/me/profile/emails")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "gtm-api")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<common::json::Value<Vec<MicrosoftEmail>>>()
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