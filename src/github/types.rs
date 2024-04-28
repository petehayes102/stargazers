use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct License {
    pub key: String,
    pub name: String,
    pub url: Option<String>,
    pub spdx_id: Option<String>,
    pub node_id: String,
    pub html_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum OwnerType {
    User,
    Organization,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub owner_type: OwnerType,
    pub site_admin: bool,
    pub starred_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct Organization {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub owner_type: OwnerType,
    pub site_admin: bool,
    pub starred_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct RepoPermissions {
    pub admin: bool,
    pub pull: bool,
    pub push: bool,
    pub maintain: Option<bool>,
    pub triage: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RepoVisibility {
    Public,
    Private,
    Internal,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SquashMergeCommitTitle {
    PrTitle,
    CommitOrPrTitle,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SquashMergeCommitMessage {
    PrBody,
    CommitMessages,
    Blank,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MergeCommitTitle {
    PrTitle,
    MergeMessage,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MergeCommitMessage {
    PrBody,
    PrTitle,
    Blank,
}

#[derive(Deserialize, Debug)]
pub struct CodeOfConduct {
    pub url: String,
    pub key: String,
    pub name: String,
    pub html_url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SecurityStatus {
    Enabled,
    Disabled,
}

#[derive(Deserialize, Debug)]
pub struct SecurityStatusObj {
    pub status: SecurityStatus,
}

#[derive(Deserialize, Debug)]
pub struct SecurityAndAnalysis {
    pub advanced_security: SecurityStatusObj,
    pub dependabot_security_updates: SecurityStatusObj,
    pub secret_scanning: SecurityStatusObj,
    pub secret_scanning_push_protection: SecurityStatusObj,
}

#[derive(Deserialize, Debug)]
pub struct ParentRepository {
    pub id: u32,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub clone_url: String,
    pub mirror_url: Option<String>,
    pub hooks_url: String,
    pub svn_url: String,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: u32,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub size: u64,
    pub default_branch: String,
    pub open_issues_count: u64,
    pub is_template: bool,
    pub topics: Vec<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub archived: bool,
    pub disabled: bool,
    pub visibility: RepoVisibility,
    pub pushed_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub permissions: RepoPermissions,
    pub temp_clone_token: Option<String>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_auto_merge: Option<bool>,
    pub delete_branch_on_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub allow_update_branch: Option<bool>,
    pub use_squash_pr_title_as_default: Option<bool>,
    pub squash_merge_commit_title: SquashMergeCommitTitle,
    pub squash_merge_commit_message: SquashMergeCommitMessage,
    pub merge_commit_title: MergeCommitTitle,
    pub merge_commit_message: MergeCommitMessage,
    pub allow_forking: bool,
    pub web_commit_signoff_required: bool,
    pub subscribers_count: Option<u32>,
    pub network_count: Option<u32>,
    pub license: Option<License>,
    pub organization: Option<Organization>,
    pub forks: u32,
    pub master_branch: Option<String>,
    pub open_issues: u64,
    pub watchers: u32,
    pub anonymous_access_enabled: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub id: u32,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub clone_url: String,
    pub mirror_url: Option<String>,
    pub hooks_url: String,
    pub svn_url: String,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: u32,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub size: u64,
    pub default_branch: String,
    pub open_issues_count: u64,
    pub is_template: bool,
    pub topics: Vec<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub has_discussions: bool,
    pub archived: bool,
    pub disabled: bool,
    pub visibility: RepoVisibility,
    pub pushed_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub permissions: RepoPermissions,
    pub allow_rebase_merge: Option<bool>,
    pub template_repository: Option<ParentRepository>,
    pub temp_clone_token: Option<String>,
    pub allow_squash_merge: Option<bool>,
    pub allow_auto_merge: Option<bool>,
    pub delete_branch_on_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub allow_update_branch: Option<bool>,
    pub use_squash_pr_title_as_default: Option<bool>,
    pub squash_merge_commit_title: Option<SquashMergeCommitTitle>,
    pub squash_merge_commit_message: Option<SquashMergeCommitMessage>,
    pub merge_commit_title: Option<MergeCommitTitle>,
    pub merge_commit_message: Option<MergeCommitMessage>,
    pub allow_forking: bool,
    pub web_commit_signoff_required: bool,
    pub subscribers_count: Option<u32>,
    pub network_count: Option<u32>,
    pub license: Option<License>,
    pub organization: Option<Organization>,
    pub parent: Option<ParentRepository>,
    pub source: Option<ParentRepository>,
    pub forks: u32,
    pub master_branch: Option<String>,
    pub open_issues: u64,
    pub watchers: u32,
    pub anonymous_access_enabled: Option<bool>,
    pub code_of_conduct: Option<CodeOfConduct>,
    pub security_and_analysis: Option<SecurityAndAnalysis>,
    pub custom_properties: Option<Value>,
}

#[derive(Deserialize, Debug)]
pub struct StarredRepository {
    pub repo: Repository,
    pub starred_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub owner_type: OwnerType,
    pub site_admin: bool,
    pub starred_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct Stargazer {
    pub user: User,
    pub starred_at: DateTime<Utc>,
}

impl Repository {
    pub async fn get_stargazers(&self, client: &Client) -> Result<Vec<Stargazer>> {
        let mut stargazers = Vec::new();

        let mut looping = true;
        let mut page = 1;
        while looping {
            let reply = client
                .get(&self.stargazers_url)
                .query(&[("page", page), ("per_page", 100)])
                .send()
                .await?;

            let mut s: Vec<Stargazer> = match reply.error_for_status() {
                Ok(r) => r.json().await?,
                Err(e)
                    if e.status() == Some(StatusCode::INTERNAL_SERVER_ERROR)
                        || e.status() == Some(StatusCode::BAD_GATEWAY) =>
                {
                    println!("Retrying broken page {page}: {e}");
                    continue;
                }
                Err(e) => return Err(e.into()),
            };

            if s.len() < 100 {
                looping = false;
            }

            stargazers.append(&mut s);
            page += 1;
        }

        Ok(stargazers)
    }
}

impl User {
    pub async fn get_followers(&self, client: &Client) -> Result<Vec<User>> {
        let mut followers = Vec::new();

        let mut looping = true;
        let mut page = 1;
        while looping {
            let reply = client
                .get(&self.followers_url)
                .query(&[("page", page), ("per_page", 100)])
                .send()
                .await?;

            let mut f: Vec<User> = match reply.error_for_status() {
                Ok(r) => r.json().await?,
                Err(e)
                    if e.status() == Some(StatusCode::INTERNAL_SERVER_ERROR)
                        || e.status() == Some(StatusCode::BAD_GATEWAY) =>
                {
                    println!("Retrying broken page {page}: {e}");
                    continue;
                }
                Err(e) => return Err(e.into()),
            };

            if f.len() < 100 {
                looping = false;
            }

            followers.append(&mut f);
            page += 1;
        }

        Ok(followers)
    }

    pub async fn get_starred(&self, client: &Client) -> Result<Vec<StarredRepository>> {
        let mut starred = Vec::new();
        let url = self.starred_url.replace("{/owner}{/repo}", "");

        let mut looping = true;
        let mut page = 1;
        while looping {
            let reply = client
                .get(&url)
                .query(&[("page", page), ("per_page", 100)])
                .send()
                .await?;

            let mut r: Vec<StarredRepository> = match reply.error_for_status() {
                Ok(r) => r.json().await?,
                Err(e)
                    if e.status() == Some(StatusCode::INTERNAL_SERVER_ERROR)
                        || e.status() == Some(StatusCode::BAD_GATEWAY) =>
                {
                    println!("Retrying broken page {page}: {e}");
                    continue;
                }
                Err(e) => return Err(e.into()),
            };

            if r.len() < 100 {
                looping = false;
            }

            starred.append(&mut r);
            page += 1;
        }

        Ok(starred)
    }

    pub async fn get_subscribed(&self, client: &Client) -> Result<Vec<Repository>> {
        let mut subscribed = Vec::new();

        let mut looping = true;
        let mut page = 1;
        while looping {
            let reply = client
                .get(&self.subscriptions_url)
                .query(&[("page", page), ("per_page", 100)])
                .send()
                .await?;

            let mut r: Vec<Repository> = match reply.error_for_status() {
                Ok(r) => r.json().await?,
                Err(e)
                    if e.status() == Some(StatusCode::INTERNAL_SERVER_ERROR)
                        || e.status() == Some(StatusCode::BAD_GATEWAY) =>
                {
                    println!("Retrying broken page {page}: {e}");
                    continue;
                }
                Err(e) => return Err(e.into()),
            };

            if r.len() < 100 {
                looping = false;
            }

            subscribed.append(&mut r);
            page += 1;
        }

        Ok(subscribed)
    }
}
