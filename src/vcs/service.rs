use futures::future;

use crate::{github, security};
use crate::bitbucket;
use crate::bitbucket::resource::BitbucketRepo;
use crate::db::Conn;
use crate::errors::Error;
use crate::github::resource::GithubRepo;
use crate::gitlab;
use crate::gitlab::resource::GitlabRepo;
use crate::gitlab::service::{GITLAB_COM_DOMAIN, GITLAB_TALTECH_DOMAIN};
use crate::security::constants::{BITBUCKET_LOGIN_TYPE, GITHUB_LOGIN_TYPE, GITLAB_LOGIN_TYPE, TALTECH_LOGIN_TYPE};
use crate::security::model::Login;
use crate::vcs::resource::VcsRepository;

pub async fn fetch_accessible_repositories(conn: &Conn, user_id: i32) -> Result<Vec<VcsRepository>, Error> {
    let logins = security::db::find_all_logins_by_user(conn, user_id)?;
    let repo_futures = future::join_all(logins.iter()
        .map(|login| fetch_repositories_for_login(login))).await;
    let repositories = repo_futures.into_iter()
        .filter_map(|f| f.ok())
        .flatten()
        .collect();
    Ok(repositories)
}

async fn fetch_repositories_for_login(login: &Login) -> Result<Vec<VcsRepository>, Error> {
    match login.login_type {
        GITHUB_LOGIN_TYPE => {
            let repos = github::service::fetch_repos_from_github(
                &login.token,
            ).await?;
            Ok(repos.into_iter().map(VcsRepository::from).collect())
        }
        GITLAB_LOGIN_TYPE => {
            let repos = gitlab::service::fetch_repos_from_gitlab(
                &login.token,
                GITLAB_COM_DOMAIN,
            ).await?;
            Ok(repos.into_iter().map(VcsRepository::from).collect())
        }
        TALTECH_LOGIN_TYPE => {
            let repos = gitlab::service::fetch_repos_from_gitlab(
                &login.token,
                GITLAB_TALTECH_DOMAIN,
            ).await?;
            Ok(repos.into_iter().map(VcsRepository::from).collect())
        }
        BITBUCKET_LOGIN_TYPE => {
            let repos = bitbucket::service::fetch_repos_from_bitbucket(
                &login.token
            ).await?;
            Ok(repos.into_iter().map(VcsRepository::from).collect())
        }
        _ => { Ok(vec![]) }
    }
}

impl From<GithubRepo> for VcsRepository {
    fn from(repo: GithubRepo) -> Self {
        unimplemented!()
    }
}

impl From<GitlabRepo> for VcsRepository {
    fn from(repo: GitlabRepo) -> Self {
        unimplemented!()
    }
}

impl From<BitbucketRepo> for VcsRepository {
    fn from(repo: BitbucketRepo) -> Self {
        unimplemented!()
    }
}