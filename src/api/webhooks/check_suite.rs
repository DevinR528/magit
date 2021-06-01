use github_derive::StringEnum;
use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{
    datetime, default_null, App, CheckStatus, Committer, ConclusionStatus, Dt,
    Installation, Org, Repository, SimpleCommit, User,
};

/// The payload of a check suite event.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckSuiteEvent<'a> {
    /// The action that was performed.
    pub action: CheckAction,

    /// The suite of checks.
    #[serde(borrow)]
    pub check_suite: CheckSuite<'a>,

    /// Information about the repositories this app has access to.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// Detailed information about the organization the app
    /// belongs to.
    #[serde(borrow)]
    pub organization: Option<Org<'a>>,

    /// Information about Github app installation.
    ///
    /// This is only present if the event is sent from said app.
    #[serde(borrow)]
    pub installation: Option<Installation<'a>>,

    /// Detailed information about the user of the app.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// Detailed information about the requester of the app.
    #[serde(borrow)]
    pub requester: Option<User<'a>>,
}

/// The actions that can be taken in a check event.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "snake_case")]
pub enum CheckAction {
    /// A new check run was created.
    Created,

    /// The status of the check run is completed.
    Completed,

    /// Occurs when new code is pushed to the app's repository.
    Requested,

    /// Someone requested to re-run your check run from the pull request UI.
    Rerequested,

    /// Someone requested an action your app provides to be taken.
    RequestedAction,
}

/// Information about a suite of checks.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckSuite<'a> {
    /// Numeric Id of this installation.
    pub id: UInt,

    /// Numeric identifier of the repository.
    pub node_id: &'a str,

    /// Name of the head branch.
    pub head_branch: &'a str,

    /// The SHA of the head branch.
    pub head_sha: &'a str,

    /// The status of this check.
    #[serde(default, deserialize_with = "default_null")]
    pub status: CheckStatus,

    /// If this is not none then the check has finished with a status.
    #[serde(default, deserialize_with = "default_null")]
    pub conclusion: ConclusionStatus,

    /// The SHA of the branch before.
    pub before: &'a str,

    /// The SHA of the branch after.
    pub after: &'a str,

    /// The pull request being checked.
    pub pull_requests: Vec<CheckPullRequest<'a>>,

    /// The app that generated this check.
    #[serde(borrow)]
    pub app: App<'a>,

    /// The number of check runs.
    #[serde(default, deserialize_with = "default_null")]
    pub latest_check_runs_count: UInt,

    /// The github API url of this check.
    pub check_runs_url: Option<Url>,

    /// The head commit.
    pub head_commit: Option<SimpleCommit<'a>>,

    /// The time in UTC when the check was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// The time in UTC when the check was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,
}

/// Information about pull requests being checked.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckPullRequest<'a> {
    /// The github API url of the pull request.
    pub url: Url,

    /// Numeric Id of this installation.
    pub id: UInt,

    /// Number of this pull request.
    pub number: UInt,

    /// The head of this pull request.
    #[serde(borrow)]
    pub head: HeadRef<'a>,

    /// The base of this pull request.
    #[serde(borrow)]
    pub base: BaseRef<'a>,
}

/// Information about the head.
#[derive(Clone, Debug, Deserialize)]
pub struct HeadRef<'a> {
    /// The github API url of the head.
    #[serde(rename = "ref")]
    pub ref_: &'a str,

    /// The SHA of this head.
    pub sha: &'a str,

    /// Information about the related head.
    #[serde(borrow)]
    pub repo: RepoRef<'a>,
}

/// Information about the base.
#[derive(Clone, Debug, Deserialize)]
pub struct BaseRef<'a> {
    /// The github API url of the base.
    #[serde(rename = "ref")]
    pub ref_: &'a str,

    /// The SHA of this base.
    pub sha: &'a str,

    /// Information about the related base.
    #[serde(borrow)]
    pub repo: RepoRef<'a>,
}

/// Information about the repository.
#[derive(Clone, Debug, Deserialize)]
pub struct RepoRef<'a> {
    /// Numeric Id of this repository.
    pub id: UInt,

    /// The github API url of the repository.
    pub url: Url,

    /// The name of this repository.
    pub name: &'a str,
}
