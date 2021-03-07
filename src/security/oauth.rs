use async_trait::async_trait;
use rocket_oauth2::TokenResponse;
use serde::Deserialize;

#[async_trait]
pub trait LoginType {
    fn get_login_type(&self) -> i32;
    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error>;
}

pub struct GitHub;

#[async_trait]
impl LoginType for TokenResponse<GitHub> {
    fn get_login_type(&self) -> i32 {
        return 1;
    }

    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error> {
        let user = fetch_github_user(&self.access_token()).await?;
        Ok(user.get_identity_hash().to_string())
    }
}

pub trait IdentityUser {
    fn get_identity_hash(&self) -> &str;
}

#[derive(Deserialize)]
pub struct GithubUser {
    // pub login: String,
    // pub id: i64,
    pub node_id: String,
}

impl IdentityUser for GithubUser {
    fn get_identity_hash(&self) -> &str {
        return &self.node_id;
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