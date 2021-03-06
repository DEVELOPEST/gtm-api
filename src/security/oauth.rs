use serde::Deserialize;

#[derive(Deserialize)]
pub struct GithubUser {
    // pub login: String,
    // pub id: i64,
    pub node_id: String,
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