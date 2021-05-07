use serde::Deserialize;

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