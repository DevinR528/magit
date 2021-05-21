use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{
    common::{default_null, App, Dt, Org, Repo, User},
    installation::Installation,
};

/// The actions that can be taken in a check event.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
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
    pub repository: Repo<'a>,

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
    pub head_commit: Option<HeadCommit<'a>>,

    /// The time in UTC when the check was created.
    pub created_at: Dt,

    /// The time in UTC when the check was last updated.
    pub updated_at: Dt,
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    /// There are checks queued to run.
    Queued,

    /// This check has finished.
    Completed,

    /// The check is in progress.
    InProgress,

    /// The check has been requested.
    Requested,

    /// `None` is that same as not present or null.
    None,
}

impl Default for CheckStatus {
    fn default() -> Self { Self::None }
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConclusionStatus {
    /// The check has succeeded.
    Success,

    /// The check has failed.
    Failure,

    /// The check has finished with a neutral result.
    Neutral,

    /// The check has been canceled.
    Cancelled,

    /// The check has timed out.
    TimeOut,

    /// The check needs attention.
    ActionRequired,

    /// The check has gone stale.
    ///
    /// Something has changed while the check was running.
    Stale,

    /// `None` is that same as not present or null.
    None,
}

impl Default for ConclusionStatus {
    fn default() -> Self { Self::None }
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

#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct HeadCommit<'a> {
    /// SHA of the head commit.
    pub id: &'a str,

    /// SHA of the tree this commit is a part of.
    pub tree_id: &'a str,

    /// Commit message.
    pub message: &'a str,

    /// Timestamp of this commit.
    pub timestamp: Dt,

    /// Name and email of the commit author.
    #[serde(borrow)]
    pub author: Committer<'a>,

    /// Name and email of the commit committer :p
    #[serde(borrow)]
    pub committer: Committer<'a>,
}

/// The author of a commit, identified by its name and email.
#[derive(Clone, Debug, Deserialize)]
pub struct Committer<'a> {
    pub name: &'a str,
    pub email: &'a str,
}
