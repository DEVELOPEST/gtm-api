use futures::future;

use crate::{github, security};
use crate::bitbucket;
use crate::bitbucket::resource::BitbucketRepo;
use crate::common::git::GitRepo;
use crate::db::Conn;
use crate::errors::Error;
use crate::github::resource::GithubRepo;
use crate::gitlab;
use crate::gitlab::resource::GitlabRepo;
use crate::gitlab::service::{GITLAB_COM_DOMAIN, GITLAB_TALTECH_DOMAIN};
use crate::repository;
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
        .filter_map(|mut r| {
            if let Some(c) = r.repo_credentials.clone() {
                r.tracked = repository::db::exists(conn, &c.user, &c.provider, &c.repo);  // TODO: Optimize somehow
                Some(r)
            } else { None }
        })
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
        VcsRepository {
            full_name: repo.full_name.clone(),
            description: repo.description.clone().unwrap_or("".to_string()),
            url: repo.html_url.clone(),
            ssh_clone_url: repo.ssh_url.clone(),
            repo_credentials: repo.get_repo_credentials(),
            last_activity: repo.updated_at,
            size: repo.size,
            stars: repo.stargazers_count,
            tracked: false,
            private: repo.private
        }
    }
}

impl From<GitlabRepo> for VcsRepository {
    fn from(repo: GitlabRepo) -> Self {
        VcsRepository {
            full_name: repo.name_with_namespace.clone(),
            description: repo.description.clone().unwrap_or("".to_string()),
            url: repo.web_url.clone(),
            ssh_clone_url: repo.ssh_url_to_repo.clone(),
            repo_credentials: repo.get_repo_credentials(),
            last_activity: repo.last_activity_at,
            size: repo.statistics.repository_size,
            stars: repo.star_count,
            tracked: false,
            private: &repo.visibility == "private"
        }
    }
}

impl From<BitbucketRepo> for VcsRepository {
    fn from(repo: BitbucketRepo) -> Self {
        VcsRepository {
            full_name: repo.full_name.clone(),
            description: repo.description.clone(),
            url: repo.links.html.href.clone(),
            ssh_clone_url: repo.links.clone.iter()
                .find(|&c| c.name == "ssh")  // || c.name == "https")
                .unwrap().href.clone(),
            repo_credentials: repo.get_repo_credentials(),
            last_activity: repo.updated_on,
            size: repo.size,
            stars: 0,
            tracked: false,
            private: repo.is_private,
        }
    }
}