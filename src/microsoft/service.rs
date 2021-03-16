use crate::common;
use crate::microsoft::resource::{MicrosoftEmail, MicrosoftUser};

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

pub async fn fetch_emails_from_microsoft(
    token: &str
) -> Result<common::json::Value<Vec<MicrosoftEmail>>, reqwest::Error> {
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

// pub async fn fetch_repos_from_microsoft(
//     token: &str
// ) -> Result<Vec<MicrosoftEmail>, reqwest::Error> {
//     let client = reqwest::Client::new();
//     client.get("https://graph.microsoft.com/beta/me/profile/emails")
//         .header("Authorization", format!("Bearer {}", token))
//         .header("User-Agent", "gtm-api")
//         .header("Accept", "application/json")
//         .send()
//         .await?
//         .json::<common::json::Value<Vec<MicrosoftEmail>>>()
//         .await
// }