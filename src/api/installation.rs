use std::borrow::Cow;

use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::common::{
    Dt, EventKind, IssueState, Label, LockReason, Org, Repo, RepoPermission,
    RepoSelection, Type, UrlMap, User,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallationAction {
    /// Someone installs a GitHub App.
    Created,

    /// Someone uninstalls a GitHub App.
    Deleted,

    /// Someone suspends a GitHub App installation.
    Suspend,

    /// Someone un-suspends a GitHub App installation.
    Unsuspend,

    /// Someone accepts new permissions for a GitHub App installation.
    NewPermissionsAccepted,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InstallationEvent {
    /// The action that was performed.
    pub action: InstallationAction,

    /// The GitHub App installation.
    pub installation: Installation,

    /// Brief information about the repositories this app has access to.
    #[serde(default)]
    pub repositories: Vec<ShortRepo>,

    /// Detailed information about the organization the app
    /// belongs to.
    pub organization: Option<Org>,

    /// Detailed information about the user of the app.
    pub sender: User,

    /// Detailed information about the requester of the app.
    pub requester: Option<User>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Installation {
    /// Numeric Id of this installation.
    pub id: UInt,

    /// Detailed information about the user who installed the app.
    pub account: User,

    /// Whether all repositories are selected or only a few.
    pub repository_selection: RepoSelection,

    /// The public web page url.
    pub html_url: Url,

    /// Numeric identifier of the installed app.
    pub app_id: UInt,

    /// Numeric identifier for the app target.
    pub target_id: UInt,

    /// The type this app targets.
    pub target_type: Type,

    /// The permissions the app is given for each section.
    pub permissions: Permissions,

    /// Events this app has access to.
    pub events: Vec<EventKind>,

    /// Time in UTC this app was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// Time in UTC this app was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,

    /// The configuration file for this installed app.
    pub single_file_name: String,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}

/// Permissions given to the installed app for accessing metadata, contents, and issues.
#[derive(Clone, Debug, Deserialize)]
pub struct Permissions {
    /// Permission for accessing actions.
    #[serde(default)]
    pub actions: RepoPermission,

    /// Permission for accessing administration.
    #[serde(default)]
    pub administration: RepoPermission,

    /// Permission for accessing checks.
    #[serde(default)]
    pub checks: RepoPermission,

    /// Permission for accessing contents.
    #[serde(default)]
    pub contents: RepoPermission,

    /// Permission for accessing content references.
    #[serde(default)]
    pub content_references: RepoPermission,

    /// Permission for accessing deployments.
    #[serde(default)]
    pub deployments: RepoPermission,

    /// Permission for accessing discussions.
    #[serde(default)]
    pub discussions: RepoPermission,

    /// Permission for accessing environments.
    #[serde(default)]
    pub environments: RepoPermission,

    /// Permission for accessing issues.
    #[serde(default)]
    pub issues: RepoPermission,

    /// Permission for accessing members.
    #[serde(default)]
    pub members: RepoPermission,

    /// Permission for accessing metadata.
    #[serde(default)]
    pub metadata: RepoPermission,

    /// Permission for accessing organization administration.
    #[serde(default)]
    pub organization_administration: RepoPermission,

    /// Permission for accessing organization hooks.
    #[serde(default)]
    pub organization_hooks: RepoPermission,

    /// Permission for accessing organization packages.
    #[serde(default)]
    pub organization_packages: RepoPermission,

    /// Permission for accessing organization plan.
    #[serde(default)]
    pub organization_plan: RepoPermission,

    /// Permission for accessing organization projects.
    #[serde(default)]
    pub organization_projects: RepoPermission,

    /// Permission for accessing organization secrets.
    #[serde(default)]
    pub organization_secrets: RepoPermission,

    /// Permission for accessing organization self hosted runners.
    #[serde(default)]
    pub organization_self_hosted_runners: RepoPermission,

    /// Permission for accessing organization user blocking.
    #[serde(default)]
    pub organization_user_blocking: RepoPermission,

    /// Permission for accessing pages.
    #[serde(default)]
    pub pages: RepoPermission,

    /// Permission for accessing packages.
    #[serde(default)]
    pub packages: RepoPermission,

    /// Permission for accessing pull requests.
    #[serde(default)]
    pub pull_requests: RepoPermission,

    /// Permission for accessing repository hooks.
    #[serde(default)]
    pub repository_hooks: RepoPermission,

    /// Permission for accessing repository projects.
    #[serde(default)]
    pub repository_projects: RepoPermission,

    /// Permission for accessing secrets.
    #[serde(default)]
    pub secrets: RepoPermission,

    /// Permission for accessing secret scanning alerts.
    #[serde(default)]
    pub secret_scanning_alerts: RepoPermission,

    /// Permission needed for accessing security events.
    #[serde(default)]
    pub security_events: RepoPermission,

    /// Permission needed for accessing single file.
    #[serde(default)]
    pub single_file: RepoPermission,

    /// Permission needed for accessing statuses.
    #[serde(default)]
    pub statuses: RepoPermission,

    /// Permission needed for accessing team discussions.
    #[serde(default)]
    pub team_discussions: RepoPermission,

    /// Permission needed for accessing workflows.
    #[serde(default)]
    pub workflows: RepoPermission,

    /// Permission needed for accessing vulnerability alerts.
    #[serde(default)]
    pub vulnerability_alerts: RepoPermission,
}

/// Information about repositories that the installation can access.
#[derive(Clone, Debug, Deserialize)]
pub struct ShortRepo {
    /// Numeric Id of this installation.
    pub id: UInt,

    /// Numeric identifier of the repository.
    pub node_id: String,

    /// The name of this repository.
    pub name: String,

    /// The full name of this repository ie `owner/repo`.
    pub full_name: String,

    /// Whether the repository is private or public.
    pub private: bool,
}

fn datetime<'de, D>(deser: D) -> Result<Dt, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de::Error;

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum StringOrUInt<'a> {
        UInt(i64),
        String(Cow<'a, str>),
    }

    let ts = StringOrUInt::deserialize(deser)?;
    println!("{:?}", ts);
    match ts {
        StringOrUInt::UInt(timestamp) => Ok(Dt::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0)
                .ok_or_else(|| D::Error::custom("timestamp exceeded bounds"))?,
            chrono::Utc,
        )),
        StringOrUInt::String(datetime) => datetime.parse().map_err(D::Error::custom),
    }
}
