use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PATH_FROM_URL_REGEX: Regex =
        Regex::new(r#"(git@|https://)([a-zA-Z0-9.]+)[:/]([a-zA-Z0-9-_/.]+)/([a-zA-Z0-9-._]+)\.git"#).unwrap();
}

pub struct RepoCredentials {
    pub provider: String,
    pub user: String,
    pub repo: String,
}

pub fn generate_credentials_from_clone_url(url: &String) -> Option<RepoCredentials> {
    let caps = PATH_FROM_URL_REGEX.captures(url)?;
    return Some(RepoCredentials {
        provider: caps.get(2)?.as_str().to_string(),
        user: caps.get(3)?.as_str().to_string(),
        repo: caps.get(4)?.as_str().to_string(),
    });
}

pub trait GitRepo {
    fn get_repo_credentials(&self) -> Option<RepoCredentials>;
}