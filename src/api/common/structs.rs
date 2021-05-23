use std::{borrow::Cow, fmt};

use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::common::{
    datetime, datetime_opt, default_null,
    enums::{
        EventKind, IssueState, RepoCreationType, RepoPermission, RepoSelection, Type,
    },
    Dt, UrlMap,
};

/// Permissions given to the installed app for accessing metadata, contents, and issues.
#[derive(Clone, Debug, Deserialize)]
pub struct AccessPermissions {
    /// Permission for accessing actions.
    #[serde(default, deserialize_with = "default_null")]
    pub actions: RepoPermission,

    /// Permission for accessing administration.
    #[serde(default, deserialize_with = "default_null")]
    pub administration: RepoPermission,

    /// Permission for accessing checks.
    #[serde(default, deserialize_with = "default_null")]
    pub checks: RepoPermission,

    /// Permission for accessing contents.
    #[serde(default, deserialize_with = "default_null")]
    pub contents: RepoPermission,

    /// Permission for accessing content references.
    #[serde(default, deserialize_with = "default_null")]
    pub content_references: RepoPermission,

    /// Permission for accessing deployments.
    #[serde(default, deserialize_with = "default_null")]
    pub deployments: RepoPermission,

    /// Permission for accessing discussions.
    #[serde(default, deserialize_with = "default_null")]
    pub discussions: RepoPermission,

    /// Permission for accessing environments.
    #[serde(default, deserialize_with = "default_null")]
    pub environments: RepoPermission,

    /// Permission for accessing issues.
    #[serde(default, deserialize_with = "default_null")]
    pub issues: RepoPermission,

    /// Permission for accessing members.
    #[serde(default, deserialize_with = "default_null")]
    pub members: RepoPermission,

    /// Permission for accessing metadata.
    #[serde(default, deserialize_with = "default_null")]
    pub metadata: RepoPermission,

    /// Permission for accessing organization administration.
    #[serde(default, deserialize_with = "default_null")]
    pub organization_administration: RepoPermission,

    /// Permission for accessing organization hooks.
    #[serde(default, deserialize_with = "default_null")]
    pub organization_hooks: RepoPermission,

    /// Permission for accessing organization packages.
    #[serde(default, deserialize_with = "default_null")]
    pub organization_packages: RepoPermission,

    /// Permission for accessing organization plan.
    #[serde(default, deserialize_with = "default_null")]
    pub organization_plan: RepoPermission,

    /// Permission for accessing organization projects.
    #[serde(default, deserialize_with = "default_null")]
    pub organization_projects: RepoPermission,

    /// Permission for accessing organization secrets.
    #[serde(default, deserialize_with = "default_null")]
    pub organization_secrets: RepoPermission,

    /// Permission for accessing organization self hosted runners.
    #[serde(default, deserialize_with = "default_null")]
    pub organization_self_hosted_runners: RepoPermission,

    /// Permission for accessing organization user blocking.
    #[serde(default, deserialize_with = "default_null")]
    pub organization_user_blocking: RepoPermission,

    /// Permission for accessing pages.
    #[serde(default, deserialize_with = "default_null")]
    pub pages: RepoPermission,

    /// Permission for accessing packages.
    #[serde(default, deserialize_with = "default_null")]
    pub packages: RepoPermission,

    /// Permission for accessing pull requests.
    #[serde(default, deserialize_with = "default_null")]
    pub pull_requests: RepoPermission,

    /// Permission for accessing repository hooks.
    #[serde(default, deserialize_with = "default_null")]
    pub repository_hooks: RepoPermission,

    /// Permission for accessing repository projects.
    #[serde(default, deserialize_with = "default_null")]
    pub repository_projects: RepoPermission,

    /// Permission for accessing secrets.
    #[serde(default, deserialize_with = "default_null")]
    pub secrets: RepoPermission,

    /// Permission for accessing secret scanning alerts.
    #[serde(default, deserialize_with = "default_null")]
    pub secret_scanning_alerts: RepoPermission,

    /// Permission needed for accessing security events.
    #[serde(default, deserialize_with = "default_null")]
    pub security_events: RepoPermission,

    /// Permission needed for accessing single file.
    #[serde(default, deserialize_with = "default_null")]
    pub single_file: RepoPermission,

    /// Permission needed for accessing statuses.
    #[serde(default, deserialize_with = "default_null")]
    pub statuses: RepoPermission,

    /// Permission needed for accessing team discussions.
    #[serde(default, deserialize_with = "default_null")]
    pub team_discussions: RepoPermission,

    /// Permission needed for accessing workflows.
    #[serde(default, deserialize_with = "default_null")]
    pub workflows: RepoPermission,

    /// Permission needed for accessing vulnerability alerts.
    #[serde(default, deserialize_with = "default_null")]
    pub vulnerability_alerts: RepoPermission,
}

/// Information about the installed app.
#[derive(Clone, Debug, Deserialize)]
pub struct App<'a> {
    /// Numeric Id of this team.
    pub id: UInt,

    /// String identifier of the team.
    pub node_id: &'a str,

    /// The name of this team.
    pub name: &'a str,

    /// The slug of this team.
    pub slug: Option<&'a str>,

    /// The owner of this app.
    #[serde(borrow)]
    pub owner: User<'a>,

    /// The public web page url.
    pub html_url: Url,

    /// The external url related to this app.
    pub external_url: Url,

    /// Description of the repo.
    pub description: Option<&'a str>,

    /// Permissions required for this team.
    pub permissions: AccessPermissions,

    /// The time in UTC when the team was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// The time in UTC when the team was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,

    /// Events that this app has access to.
    pub events: Vec<EventKind>,
}

/// The base branch of a commit.
#[derive(Clone, Debug, Deserialize)]
pub struct Base<'a> {
    /// A name for this base `username:branch`.
    pub label: &'a str,

    /// The name of the branch.
    #[serde(rename = "ref")]
    pub ref_: &'a str,

    /// The SHA of this commit on a branch.
    pub sha: &'a str,

    /// The user who's base branch this is from.
    #[serde(borrow)]
    pub user: User<'a>,

    /// The repository the branch is from.
    pub repo: Option<Repo<'a>>,
}

/// Information about a branch.
#[derive(Clone, Debug, Deserialize)]
pub struct Branch<'a> {
    /// The name of this branch.
    pub name: &'a str,

    /// The last commit to this branch.
    #[serde(borrow)]
    pub commit: CommitTree<'a>,

    /// Is this branch protected.
    pub protected: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Changes<'a> {
    /// The changes made to the body.
    ///
    /// The is present for issues, pulls, and comments.
    #[serde(borrow)]
    pub body: Option<Body<'a>>,

    /// The changes made to the title.
    #[serde(borrow)]
    pub title: Option<Body<'a>>,

    /// The changes made to the name.
    ///
    /// This is present for releases, there may be other uses of it also, Github's API
    /// docs are so-so.
    #[serde(borrow)]
    pub name: Option<Body<'a>>,

    /// The changes made to the due_on attribute.
    ///
    /// This is present for milestones, there may be other uses of it also, Github's API
    /// docs are so-so.
    #[serde(borrow)]
    pub due_on: Option<Body<'a>>,

    /// The changes made to the description.
    ///
    /// This is present for milestones, there may be other uses of it also, Github's API
    /// docs are so-so.
    #[serde(borrow)]
    pub description: Option<Body<'a>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Body<'a> {
    /// The previous version of the body.
    pub from: &'a str,
}

/// Information about a specific commit.
#[derive(Clone, Debug, Deserialize)]
pub struct Commit<'a> {
    /// The sha of this commit.
    pub sha: &'a str,

    /// The identifier of this commit.
    pub node_id: &'a str,

    /// Information about this commit.
    #[serde(borrow)]
    pub commit: CommitInner<'a>,

    /// The api url of the commit referenced.
    pub url: &'a str,

    /// The url to github webpage associated with this commit.
    pub html_url: Url,

    /// The api url to request information about comments.
    pub comments_url: &'a str,

    /// The author of this commit.
    #[serde(borrow)]
    pub author: Committer<'a>,

    /// The user who committed the referenced commit.
    #[serde(borrow)]
    pub committer: Committer<'a>,

    /// A list of parents of this commit if any.
    #[serde(default)]
    pub parents: Vec<&'a str>,

    /// The files that were added.
    #[serde(default)]
    pub added: Vec<&'a str>,

    /// The files that were removed.
    #[serde(default)]
    pub removed: Vec<&'a str>,

    /// The files that were modified.
    #[serde(default)]
    pub modified: Vec<&'a str>,
}

/// Further information about a commit.
#[derive(Clone, Debug, Deserialize)]
pub struct CommitInner<'a> {
    /// The url to this commit.
    pub url: Url,

    /// Information about author of this commit.
    #[serde(borrow)]
    pub author: ShortUser<'a>,

    /// Information about committer.
    #[serde(borrow)]
    pub committer: ShortUser<'a>,

    /// The commit message.
    pub message: &'a str,

    /// SHA and url of the commit.
    #[serde(borrow)]
    pub tree: CommitTree<'a>,

    /// Number of comments associated with this commit.
    pub comment_count: UInt,

    /// Information about the verification of this commit.
    #[serde(borrow)]
    pub verification: Verification<'a>,
}

/// Information about the author/committer.
#[derive(Clone, Debug, Deserialize)]
pub struct Committer<'a> {
    /// The git author's name.
    pub name: Option<&'a str>,

    /// The git author's email.
    pub email: Option<&'a str>,

    /// The UTC date of the latest commit.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub date: Option<Dt>,

    /// The author's github username.
    pub username: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CommitTree<'a> {
    /// SHA of the commit.
    pub sha: &'a str,

    /// The url of this commit.
    pub url: Url,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Head<'a> {
    /// A name for this base `username:branch`.
    pub label: Option<&'a str>,

    /// The name of the branch.
    #[serde(rename = "ref")]
    pub ref_: &'a str,

    /// The SHA of this commit on a branch.
    pub sha: &'a str,

    /// The user who's base branch this is from.
    pub user: Option<User<'a>>,

    /// The repository the branch is from.
    pub repo: Option<Repo<'a>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Installation<'a> {
    /// Numeric Id of this installation.
    pub id: UInt,

    /// Detailed information about the user who installed the app.
    #[serde(borrow)]
    pub account: User<'a>,

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
    pub permissions: AccessPermissions,

    /// Events this app has access to.
    pub events: Vec<EventKind>,

    /// Time in UTC this app was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// Time in UTC this app was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,

    /// The configuration file for this installed app.
    pub single_file_name: &'a str,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}

/// Information any labels.
#[derive(Clone, Debug, Deserialize)]
pub struct Label<'a> {
    /// Numeric Id of this label.
    pub id: UInt,

    /// String identifier of the label.
    pub node_id: &'a str,

    /// The name of this label.
    pub name: &'a str,

    /// The short description of this label.
    pub description: Option<&'a str>,

    /// Background color of the label box.
    pub color: &'a str,

    /// Is this a default label.
    pub default: bool,
}

/// The links related to an issue or pull request.
#[derive(Clone, Debug)]
pub struct Links<'a> {
    pub self_link: Option<&'a str>,
    pub html_link: Option<&'a str>,
    pub issue_link: Option<&'a str>,
    pub comments_link: Option<&'a str>,
    pub review_comments_link: Option<&'a str>,
    pub review_comment_link: Option<&'a str>,
    pub commits_link: Option<&'a str>,
    pub statuses_link: Option<&'a str>,
    pub pull_request_link: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Milestone<'a> {
    /// Numeric Id of this milestone.
    pub id: UInt,

    /// String identifier of the milestone.
    pub node_id: &'a str,

    /// The name of this milestone.
    pub name: Option<&'a str>,

    /// Information about the creator of this milestone.
    #[serde(borrow)]
    pub creator: User<'a>,

    /// The public web page url.
    pub html_url: Url,

    /// The url to the github api of this repo.
    pub url: &'a str,

    /// The url to the github api labels requests.
    pub labels_url: &'a str,

    /// Description of the repo.
    pub description: Option<&'a str>,

    /// The number this milestone is.
    pub number: UInt,

    /// The state of this milestone.
    #[serde(default, deserialize_with = "default_null")]
    pub state: IssueState,

    /// The title of this milestone.
    pub title: &'a str,

    /// The number of open issues related to this milestone.
    #[serde(default)]
    pub open_issues: UInt,

    /// The number of closed issues related to this milestone.
    #[serde(default)]
    pub closed_issues: UInt,

    /// The time in UTC when the milestone was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// The time in UTC when the milestone was last updated.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub updated_at: Option<Dt>,

    /// The time in UTC when the milestone was closed.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub closed_at: Option<Dt>,

    /// The time in UTC when the milestone is due.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub due_on: Option<Dt>,
}

/// Information about a github organization.
#[derive(Clone, Debug, Deserialize)]
pub struct Org<'a> {
    /// The name of the organization.
    pub login: &'a str,

    /// Numeric identifier of the organization.
    #[serde(default)]
    pub id: UInt,

    /// String identifier of the organization.
    pub node_id: &'a str,

    /// The url to the organizations github api.
    pub url: Url,

    /// Url to the organizations avatar image.
    pub avatar_url: Url,

    /// A description of the organization.
    #[serde(default)]
    pub description: &'a str,

    /// The name of the organization.
    pub name: &'a str,

    /// The name of the company associated with this organization.
    pub company: Option<&'a str>,

    /// Url to a blog associated with this organization.
    pub blog: Option<&'a str>,

    /// The location of this organization.
    pub location: Option<&'a str>,

    /// An email address for this organization.
    pub email: Option<&'a str>,

    /// The twitter user associated with this organization.
    pub twitter_username: Option<&'a str>,

    /// Is this organization verified.
    pub is_verified: bool,

    /// Does this organization have projects.
    #[serde(default)]
    pub has_organization_projects: bool,
    /// Does this organization have repository projects.
    #[serde(default)]
    pub has_repository_projects: bool,

    /// Number of public repositories.
    #[serde(default)]
    pub public_repos: UInt,

    /// Number of public gists.
    #[serde(default)]
    pub public_gists: UInt,

    /// Number of followers.
    #[serde(default)]
    pub followers: UInt,

    /// Number of users the organization is following.
    #[serde(default)]
    pub following: UInt,

    /// Url to the organizations github account.
    pub html_url: Url,

    /// Time in UTC this organization was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// Time in UTC this organization was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,

    /// Type of resource this is.
    #[serde(rename = "type")]
    pub kind: Type,

    /// The number of private repos.
    #[serde(default)]
    pub total_private_repos: UInt,

    /// The number of owned private repos.
    #[serde(default)]
    pub owned_private_repos: UInt,

    /// The number of private gists.
    #[serde(default)]
    pub private_gists: UInt,

    /// How much space this organization takes up on disk.
    ///
    /// This measures the size of repos, gists, and any other resources an
    /// organization has.
    #[serde(default)]
    pub disk_usage: UInt,

    /// Number of collaborators.
    #[serde(default)]
    pub collaborators: UInt,

    /// The email of the person who pays.
    pub billing_email: Option<&'a str>,

    /// The plan this organization is using.
    pub plan: Option<Plan<'a>>,

    /// The default permissions of a repository.
    #[serde(default, deserialize_with = "default_null")]
    pub default_repository_permission: RepoPermission,

    /// Can organization members create new repos.
    ///
    /// Note: defaults to true.
    #[serde(default = "crate::api::common::true_fn")]
    pub members_can_create_repositories: bool,

    /// Does this organization require 2fa.
    #[serde(default)]
    pub two_factor_requirement_enabled: bool,

    /// The creation type for repositories in this organization.
    #[serde(default, deserialize_with = "default_null")]
    pub members_allowed_repository_creation_type: RepoCreationType,

    /// Note: defaults to true.
    #[serde(default = "crate::api::common::true_fn")]
    pub members_can_create_public_repositories: bool,
    /// Note: defaults to true.
    #[serde(default = "crate::api::common::true_fn")]
    pub members_can_create_private_repositories: bool,
    /// Note: defaults to true.
    #[serde(default = "crate::api::common::true_fn")]
    pub members_can_create_internal_repositories: bool,
    /// Note: defaults to true.
    #[serde(default = "crate::api::common::true_fn")]
    pub members_can_create_pages: bool,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}

/// The permissions a repository has.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Permissions {
    /// Administrative privileges.
    #[serde(default)]
    admin: bool,

    /// Are pushes enabled.
    #[serde(default)]
    push: bool,

    /// Is pulling permitted.
    #[serde(default)]
    pull: bool,
}

/// Information about a user/organizations plan.
#[derive(Clone, Debug, Deserialize)]
pub struct Plan<'a> {
    /// The name of this plan.
    pub name: &'a str,

    /// How much space does the organization have.
    #[serde(default)]
    pub space: UInt,

    /// The number of private repositories this org has.
    #[serde(default)]
    pub private_repos: UInt,

    /// Number of members.
    #[serde(default)]
    pub filled_seats: UInt,

    /// Number of allowed members.
    #[serde(default)]
    pub seats: UInt,
}

/// Information about a repository.
#[derive(Clone, Debug, Deserialize)]
pub struct Repo<'a> {
    /// Numeric Id of this repository.
    pub id: UInt,

    /// String identifier of the repository.
    pub node_id: &'a str,

    /// The name of this repository.
    pub name: &'a str,

    /// The name including owner ie. `owner/repo-name`.
    pub full_name: &'a str,

    /// The visibility of this repo.
    #[serde(default)]
    pub private: bool,

    /// Is this repo a fork.
    #[serde(default)]
    pub fork: bool,

    /// Information about the owner of this repository.
    #[serde(borrow)]
    pub owner: User<'a>,

    /// The public web page url.
    pub html_url: Url,

    /// The url to the github api of this repo.
    pub url: &'a str,

    /// Description of the repo.
    pub description: Option<&'a str>,

    /// The time in UTC when the repo was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// The time in UTC when the repo was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,

    /// The time in UTC when the repo was last pushed to.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub pushed_at: Option<Dt>,

    /// The url used when doing git operations.
    pub git_url: Option<&'a str>,

    /// The url used when doing ssh operations.
    pub ssh_url: Option<&'a str>,

    /// The url used to clone this repo.
    pub clone_url: Option<Url>,

    /// The url used for svn.
    pub svn_url: Option<Url>,

    /// The homepage of this repo, if set.
    pub homepage: Option<&'a str>,

    /// Size of the repository.
    #[serde(default)]
    pub size: UInt,

    /// Number of stargazers (people who have starred the repo).
    #[serde(default)]
    pub stargazers_count: UInt,

    /// Number of people who watch this repo.
    #[serde(default)]
    pub watchers_count: UInt,

    /// The programming language used for this repo.
    pub language: Option<&'a str>,

    /// Does this repo allow issues.
    #[serde(default)]
    pub has_issues: bool,

    /// Does this repo contain projects.
    #[serde(default)]
    pub has_projects: bool,

    /// Does this repo have downloadable resources.
    #[serde(default)]
    pub has_downloads: bool,

    /// Does this repo have a wiki.
    #[serde(default)]
    pub has_wiki: bool,

    /// Does this repo have github pages associated with it.
    #[serde(default)]
    pub has_pages: bool,

    /// How many times has this repo been forked.
    #[serde(default)]
    pub forks_count: UInt,

    /// The url to the repository this repo mirrors.
    pub mirror_url: Option<&'a str>,

    /// Has this repo been archived.
    #[serde(default)]
    pub archived: bool,

    /// Has the repo been disabled.
    #[serde(default)]
    pub disabled: bool,

    /// How many open issues does this repo have.
    #[serde(default)]
    pub open_issues_count: UInt,

    /// License of this repo.
    pub license: Option<&'a str>,

    /// Number of forks for the repo.
    #[serde(default)]
    pub forks: UInt,

    /// Number of open issues.
    #[serde(default)]
    pub open_issues: UInt,

    /// Number of watchers.
    #[serde(default)]
    pub watchers: UInt,

    /// Number of stars.
    #[serde(default)]
    pub stargazers: UInt,

    /// This repositories default branch.
    pub default_branch: Option<&'a str>,

    /// Allow squash and merge in web merge.
    #[serde(default = "crate::api::common::true_fn")]
    pub allow_squash_merge: bool,

    /// Allow merge commit in web merge.
    #[serde(default)]
    pub allow_merge_commit: bool,

    /// Allow rebase and merge in web merge.
    #[serde(default = "crate::api::common::true_fn")]
    pub allow_rebase_merge: bool,

    /// Allow branch to be deleted after merge.
    #[serde(default)]
    pub delete_branch_on_merge: bool,

    /// The topics this repo covers.
    #[serde(default)]
    pub topics: Vec<&'a str>,

    /// The set permissions of this repo.
    #[serde(default)]
    pub permissions: Permissions,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}

/// Simple information about a "user".
#[derive(Clone, Debug, Deserialize)]
pub struct ShortUser<'a> {
    /// Name of the user.
    pub name: &'a str,

    /// Email of the user.
    pub email: &'a str,

    /// The date of the event this user is related to happened.
    pub date: Dt,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Team<'a> {
    /// Numeric Id of this team.
    pub id: UInt,

    /// String identifier of the team.
    pub node_id: &'a str,

    /// The name of this team.
    pub name: &'a str,

    /// The slug of this team.
    pub slug: &'a str,

    /// The public web page url.
    pub html_url: Url,

    /// The url to the github api of this repo.
    pub url: &'a str,

    /// Description of the repo.
    pub description: Option<&'a str>,

    /// The privacy this team is.
    pub privacy: &'a str,

    /// Permissions required for this team.
    pub permissions: RepoPermission,

    /// The title of this team.
    pub title: &'a str,

    /// The number of members on this team.
    #[serde(default)]
    pub members_count: UInt,

    /// The time in UTC when the team was created.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub created_at: Option<Dt>,

    /// The time in UTC when the team was last updated.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub updated_at: Option<Dt>,

    /// An owning organization of this team.
    pub organization: Option<Org<'a>>,

    /// The parent team.
    pub parent: Option<Box<Team<'a>>>,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}

/// Information about a user.
///
/// This can be used for identifying an organization, owner, or
/// sender.
#[derive(Clone, Debug, Deserialize)]
pub struct User<'a> {
    /// The name of the user.
    pub login: &'a str,

    /// The numeric identifier of this user.
    pub id: UInt,

    /// String identifier of the user.
    pub node_id: &'a str,

    /// The users avatar url.
    pub avatar_url: Url,

    pub gravatar_id: &'a str,

    /// Url to the github api for this user.
    pub url: &'a str,

    /// Url to the github webpage of this user.
    pub html_url: &'a str,

    /// The type of user.
    #[serde(rename = "type")]
    pub kind: Type,

    /// Is this the administrator of this resource.
    pub site_admin: bool,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}

/// Information about the verification of an object.
#[derive(Clone, Debug, Deserialize)]
pub struct Verification<'a> {
    /// Has this object been verified.
    pub verified: bool,

    /// Reason given about verification.
    ///
    /// "valid" on success, may give an error on failure.
    pub reason: &'a str,

    /// The PGP signature of this commit.
    #[serde(borrow)]
    pub signature: Option<Cow<'a, str>>,

    /// The payload of this commit.
    ///
    /// Often source control specific information.
    #[serde(borrow)]
    pub payload: Option<Cow<'a, str>>,
}

impl<'de: 'a, 'a> Deserialize<'de> for Links<'a> {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, MapAccess, Visitor};

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            #[serde(rename = "self")]
            Self_,
            Html,
            Issue,
            Comments,
            ReviewComments,
            ReviewComment,
            Commits,
            Statuses,
            PullRequest,
        }

        #[derive(Deserialize)]
        struct Href<'a> {
            href: Option<&'a str>,
        }

        struct LinksVisitor<'a>(std::marker::PhantomData<&'a ()>);
        impl<'de: 'a, 'a> Visitor<'de> for LinksVisitor<'a> {
            type Value = Links<'a>;

            // TODO: finish list
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("one of User, Owner, TODO")
            }

            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut self_link = None;
                let mut html_link = None;
                let mut issue_link = None;
                let mut comments_link = None;
                let mut review_comments_link = None;
                let mut review_comment_link = None;
                let mut commits_link = None;
                let mut statuses_link = None;
                let mut pull_request_link = None;
                // While there are entries remaining in the input, add them
                // into our map.
                while let Some((key, value)) = map.next_entry::<Field, Href<'a>>()? {
                    match key {
                        Field::Self_ => {
                            if self_link.is_some() {
                                return Err(Error::duplicate_field("self_link"));
                            }
                            self_link = value.href;
                        }
                        Field::Html => {
                            if html_link.is_some() {
                                return Err(Error::duplicate_field("html_link"));
                            }
                            html_link = value.href;
                        }
                        Field::Issue => {
                            if issue_link.is_some() {
                                return Err(Error::duplicate_field("issue_link"));
                            }
                            issue_link = value.href;
                        }
                        Field::Comments => {
                            if comments_link.is_some() {
                                return Err(Error::duplicate_field("comments_link"));
                            }
                            comments_link = value.href;
                        }
                        Field::ReviewComments => {
                            if review_comments_link.is_some() {
                                return Err(Error::duplicate_field(
                                    "review_comments_link",
                                ));
                            }
                            review_comments_link = value.href;
                        }
                        Field::ReviewComment => {
                            if review_comment_link.is_some() {
                                return Err(Error::duplicate_field(
                                    "review_comment_link",
                                ));
                            }
                            review_comment_link = value.href;
                        }
                        Field::Commits => {
                            if commits_link.is_some() {
                                return Err(Error::duplicate_field("commits_link"));
                            }
                            commits_link = value.href;
                        }
                        Field::Statuses => {
                            if statuses_link.is_some() {
                                return Err(Error::duplicate_field("statuses_link"));
                            }
                            statuses_link = value.href;
                        }
                        Field::PullRequest => {
                            if pull_request_link.is_some() {
                                return Err(Error::duplicate_field("pull_request_link"));
                            }
                            pull_request_link = value.href;
                        }
                    }
                }

                Ok(Links {
                    self_link,
                    html_link,
                    issue_link,
                    comments_link,
                    review_comments_link,
                    review_comment_link,
                    commits_link,
                    statuses_link,
                    pull_request_link,
                })
            }
        }
        const FIELDS: &[&str] = &[
            "self_link",
            "html_link",
            "issue_link",
            "comments_link",
            "review_comments_link",
            "review_comment_link",
            "commits_link",
            "statuses_link",
            "pull_request_link",
        ];
        d.deserialize_struct("Links", FIELDS, LinksVisitor(std::marker::PhantomData))
    }
}
