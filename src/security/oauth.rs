use async_trait::async_trait;
use reqwest::Error;
use rocket_oauth2::TokenResponse;

use crate::bitbucket::resource::BitbucketUser;
use crate::common::git::{GitRepo, RepoCredentials};
use crate::github::resource::GithubUser;
use crate::github::service::{fetch_emails_from_github, fetch_github_user, fetch_repos_from_github};
use crate::gitlab::resource::GitlabUser;
use crate::gitlab::service::{fetch_emails_from_gitlab, fetch_gitlab_user, fetch_repos_from_gitlab, GITLAB_COM_DOMAIN, GITLAB_TALTECH_DOMAIN};
use crate::microsoft::resource::MicrosoftUser;
use crate::microsoft::service::{fetch_emails_from_microsoft, fetch_microsoft_user};
use crate::bitbucket::service::{fetch_bitbucket_user, fetch_emails_from_bitbucket, fetch_repos_from_bitbucket};
use crate::security::constants::{GITHUB_LOGIN_TYPE, GITLAB_LOGIN_TYPE, TALTECH_LOGIN_TYPE, MICROSOFT_LOGIN_TYPE, BITBUCKET_LOGIN_TYPE};

#[async_trait]
pub trait LoginType {
    fn get_login_type(&self) -> i32;
    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error>;
    async fn fetch_username(&self) -> Result<String, reqwest::Error>;
    async fn fetch_emails(&self) -> Result<Vec<String>, reqwest::Error>;
    async fn fetch_accessible_repositories(&self) -> Result<Vec<RepoCredentials>, reqwest::Error>;
}

pub struct GitHub;
pub struct GitLab;
pub struct GitLabTalTech;

pub struct Microsoft;

pub struct Bitbucket;

#[async_trait]
impl LoginType for TokenResponse<GitHub> {
    fn get_login_type(&self) -> i32 {
        return GITHUB_LOGIN_TYPE;
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

    async fn fetch_accessible_repositories(&self) -> Result<Vec<RepoCredentials>, Error> {
        let repos = fetch_repos_from_github(&self.access_token()).await?;
        Ok(repos.into_iter()
            .filter_map(|r| r.get_repo_credentials())
            .collect())
    }
}

#[async_trait]
impl LoginType for TokenResponse<GitLab> {
    fn get_login_type(&self) -> i32 {
        return GITLAB_LOGIN_TYPE;
    }

    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error> {
        let user = fetch_gitlab_user(&self.access_token(), GITLAB_COM_DOMAIN).await?;
        Ok(user.get_identity_hash().to_string())
    }

    async fn fetch_username(&self) -> Result<String, Error> {
        let user = fetch_gitlab_user(&self.access_token(), GITLAB_COM_DOMAIN).await?;
        Ok(user.username)
    }

    async fn fetch_emails(&self) -> Result<Vec<String>, Error> {
        let emails_res = fetch_emails_from_gitlab(&self.access_token(), GITLAB_COM_DOMAIN).await?;
        let emails = emails_res.iter().map(|email| email.email.clone()).collect();
        Ok(emails)
    }

    async fn fetch_accessible_repositories(&self) -> Result<Vec<RepoCredentials>, Error> {
        let repos = fetch_repos_from_gitlab(&self.access_token(), GITLAB_COM_DOMAIN).await?;
        Ok(repos.into_iter()
            .filter_map(|r| r.get_repo_credentials())
            .collect())
    }
}

#[async_trait]
impl LoginType for TokenResponse<GitLabTalTech> {
    fn get_login_type(&self) -> i32 {
        return TALTECH_LOGIN_TYPE;
    }

    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error> {
        let user = fetch_gitlab_user(&self.access_token(), GITLAB_TALTECH_DOMAIN).await?;
        Ok(user.get_identity_hash().to_string())
    }

    async fn fetch_username(&self) -> Result<String, Error> {
        let user = fetch_gitlab_user(&self.access_token(), GITLAB_TALTECH_DOMAIN).await?;
        Ok(user.username)
    }

    async fn fetch_emails(&self) -> Result<Vec<String>, Error> {
        let emails_res = fetch_emails_from_gitlab(&self.access_token(), GITLAB_TALTECH_DOMAIN).await?;
        let emails = emails_res.iter().map(|email| email.email.clone()).collect();
        Ok(emails)
    }

    async fn fetch_accessible_repositories(&self) -> Result<Vec<RepoCredentials>, Error> {
        let repos = fetch_repos_from_gitlab(&self.access_token(), GITLAB_TALTECH_DOMAIN).await?;
        Ok(repos.into_iter()
            .filter_map(|r| r.get_repo_credentials())
            .collect())
    }
}

#[async_trait]
impl LoginType for TokenResponse<Microsoft> {
    fn get_login_type(&self) -> i32 {
        return MICROSOFT_LOGIN_TYPE;
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

    async fn fetch_accessible_repositories(&self) -> Result<Vec<RepoCredentials>, Error> {
        Ok(vec![])
    }
}

#[async_trait]
impl LoginType for TokenResponse<Bitbucket> {
    fn get_login_type(&self) -> i32 {
        return BITBUCKET_LOGIN_TYPE;
    }

    async fn fetch_identity_hash(&self) -> Result<String, reqwest::Error> {
        let user = fetch_bitbucket_user(&self.access_token()).await?;
        Ok(user.get_identity_hash().to_string())
    }

    async fn fetch_username(&self) -> Result<String, reqwest::Error> {
        let user = fetch_bitbucket_user(&self.access_token()).await?;
        Ok(user.username)
    }

    async fn fetch_emails(&self) -> Result<Vec<String>, Error> {
        let emails_res = fetch_emails_from_bitbucket(&self.access_token()).await?;
        let emails = emails_res.iter()
            .filter(|email| email.is_confirmed)
            .map(|email| email.email.clone()).collect();
        Ok(emails)
    }

    async fn fetch_accessible_repositories(&self) -> Result<Vec<RepoCredentials>, Error> {
        let repos = fetch_repos_from_bitbucket(&self.access_token()).await?;
        Ok(repos.into_iter()
            .filter_map(|r| r.get_repo_credentials())
            .collect())
    }
}

pub trait IdentityUser {
    fn get_identity_hash(&self) -> String;
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

impl IdentityUser for BitbucketUser {
    fn get_identity_hash(&self) -> String {
        return self.account_id.to_string();
    }
}
