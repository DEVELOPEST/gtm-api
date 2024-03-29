use diesel::PgConnection;
use futures::future;
use rand::seq::SliceRandom;

use crate::common::git;
use crate::common::git::GitRepo;
use crate::domain::group_access;
use crate::domain::repository;
use crate::domain::sync;
use crate::errors::Error;
use crate::security;
use crate::security::constants::{BITBUCKET_LOGIN_TYPE, GITHUB_LOGIN_TYPE, GITLAB_LOGIN_TYPE, TALTECH_LOGIN_TYPE};
use crate::security::model::Login;
use crate::vcs::bitbucket;
use crate::vcs::bitbucket::resource::BitbucketRepo;
use crate::vcs::github;
use crate::vcs::github::resource::GithubRepo;
use crate::vcs::gitlab;
use crate::vcs::gitlab::resource::GitlabRepo;
use crate::vcs::gitlab::service::{GITLAB_COM_DOMAIN, GITLAB_TALTECH_DOMAIN};
use crate::vcs::resource::{TrackedRepository, VcsRepository};

pub async fn fetch_accessible_repositories(conn: &PgConnection, user_id: i32, name: Option<&str>) -> Result<Vec<VcsRepository>, Error> {
    let logins = security::db::find_all_logins_by_user(conn, user_id)?;
    let repo_futures = future::join_all(logins.iter()
        .map(|login| fetch_repositories_for_login(login, name))).await;
    let mut repositories: Vec<VcsRepository> = repo_futures.into_iter()
        .filter_map(|f| f.ok())
        .flatten()
        .filter_map(|mut r| {
            if let Some(c) = r.repo_credentials.clone() {
                let repo = repository::db::find(conn, &c.user, &c.provider, &c.repo);  // TODO: Optimize somehow
                r.tracked = repo.is_ok();
                r.id = repo.ok().map(|x| x.id);
                Some(r)
            } else { None }
        })
        .collect();
    repositories.sort_by_key(|a| (a.stars, a.size));
    repositories.reverse();
    Ok(repositories)
}

async fn fetch_repositories_for_login(login: &Login, repo_name: Option<&str>) -> Result<Vec<VcsRepository>, Error> {
    match login.login_type {
        GITHUB_LOGIN_TYPE => {
            let repos = github::service::fetch_repos_from_github(
                &login.token,
                repo_name,
            ).await?;
            Ok(repos.into_iter().map(VcsRepository::from).collect())
        }
        GITLAB_LOGIN_TYPE => {
            let repos = gitlab::service::fetch_repos_from_gitlab(
                &login.token,
                GITLAB_COM_DOMAIN,
                repo_name,
            ).await?;
            Ok(repos.into_iter().map(VcsRepository::from).collect())
        }
        TALTECH_LOGIN_TYPE => {
            let repos = gitlab::service::fetch_repos_from_gitlab(
                &login.token,
                GITLAB_TALTECH_DOMAIN,
                repo_name,
            ).await?;
            Ok(repos.into_iter().map(VcsRepository::from).collect())
        }
        BITBUCKET_LOGIN_TYPE => {
            let repos = bitbucket::service::fetch_repos_from_bitbucket(
                &login.token,
                repo_name,
            ).await?;
            Ok(repos.into_iter().map(VcsRepository::from).collect())
        }
        _ => { Ok(vec![]) }
    }
}

pub async fn start_tracking_repository(
    conn: &PgConnection,
    clone_url: &str,
    user_id: i32,
) -> Result<TrackedRepository, Error> {
    if let Some(repo_credentials) = git::generate_credentials_from_clone_url(clone_url) {

        let access_ok: bool = fetch_accessible_repositories(conn, user_id, Some(&repo_credentials.repo)).await?
            .iter()
            .any(|r| r.clone_url == clone_url);

        if !access_ok {
            return Err(Error::AuthorizationError("No repository access! Try linking OAuth"));
        }

        let sync_clients = sync::db::find_all_sync_clients_by_type(conn, sync::TYPE_PUBLIC)?;
        if let Some(chosen_client) = sync_clients.choose(&mut rand::thread_rng()) {
            let sync_url = sync::service::track_repository(chosen_client, clone_url).await?;
            group_access::service::create_group_accesses_for_user(
                conn,
                vec![repo_credentials],
                user_id,
            )?;
            return Ok(TrackedRepository { sync_url })
        };
    }

    Err(Error::Custom("Unable to find public sync client."))
}

impl From<GithubRepo> for VcsRepository {
    fn from(repo: GithubRepo) -> Self {
        VcsRepository {
            full_name: repo.full_name.clone(),
            description: repo.description.clone().unwrap_or("".to_string()),
            url: repo.html_url.clone(),
            clone_url: repo.ssh_url.clone(),
            repo_credentials: repo.get_repo_credentials(),
            last_activity: repo.updated_at,
            size: repo.size,
            stars: repo.stargazers_count,
            tracked: false,
            private: repo.private,
            id: None
        }
    }
}

impl From<GitlabRepo> for VcsRepository {
    fn from(repo: GitlabRepo) -> Self {
        VcsRepository {
            full_name: repo.name_with_namespace.clone(),
            description: repo.description.clone().unwrap_or("".to_string()),
            url: repo.web_url.clone(),
            clone_url: repo.ssh_url_to_repo.clone(),
            repo_credentials: repo.get_repo_credentials(),
            last_activity: repo.last_activity_at,
            size: repo.statistics.repository_size,
            stars: repo.star_count,
            tracked: false,
            private: &repo.visibility == "private",
            id: None
        }
    }
}

impl From<BitbucketRepo> for VcsRepository {
    fn from(repo: BitbucketRepo) -> Self {
        VcsRepository {
            full_name: repo.full_name.clone(),
            description: repo.description.clone(),
            url: repo.links.html.href.clone(),
            clone_url: repo.links.clone.iter()
                .find(|&c| c.name == "ssh")  // || c.name == "https")
                .unwrap().href.clone(),
            repo_credentials: repo.get_repo_credentials(),
            last_activity: repo.updated_on,
            size: repo.size,
            stars: 0,
            tracked: false,
            private: repo.is_private,
            id: None
        }
    }
}