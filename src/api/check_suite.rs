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
pub struct CheckSuiteEvent {
    /// The action that was performed.
    pub action: CheckAction,

    /// The suite of checks.
    pub check_suite: CheckSuite,

    /// Information about the repositories this app has access to.
    pub repository: Repo,

    /// Detailed information about the organization the app
    /// belongs to.
    pub organization: Option<Org>,

    /// Information about Github app installation.
    ///
    /// This is only present if the event is sent from said app.
    pub installation: Option<Installation>,

    /// Detailed information about the user of the app.
    pub sender: User,

    /// Detailed information about the requester of the app.
    pub requester: Option<User>,
}

/// Information about a suite of checks.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckSuite {
    /// Numeric Id of this installation.
    pub id: UInt,

    /// Numeric identifier of the repository.
    pub node_id: String,

    /// Name of the head branch.
    pub head_branch: String,

    /// The SHA of the head branch.
    pub head_sha: String,

    /// The status of this check.
    #[serde(default, deserialize_with = "default_null")]
    pub status: CheckStatus,

    /// If this is not none then the check has finished with a status.
    #[serde(default, deserialize_with = "default_null")]
    pub conclusion: ConclusionStatus,

    /// The SHA of the branch before.
    pub before: String,

    /// The SHA of the branch after.
    pub after: String,

    /// The pull request being checked.
    pub pull_requests: Vec<CheckPullRequest>,

    /// The app that generated this check.
    pub app: App,

    /// The number of check runs.
    #[serde(default, deserialize_with = "default_null")]
    pub latest_check_runs_count: UInt,

    /// The github API url of this check.
    pub check_runs_url: Option<Url>,

    /// The head commit.
    pub head_commit: Option<HeadCommit>,

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
pub struct CheckPullRequest {
    /// The github API url of the pull request.
    pub url: Url,

    /// Numeric Id of this installation.
    pub id: UInt,

    /// Number of this pull request.
    pub number: UInt,

    /// The head of this pull request.
    pub head: HeadRef,

    /// The base of this pull request.
    pub base: BaseRef,
}

/// Information about the head.
#[derive(Clone, Debug, Deserialize)]
pub struct HeadRef {
    /// The github API url of the head.
    #[serde(rename = "ref")]
    pub ref_: String,

    /// The SHA of this head.
    pub sha: String,

    /// Information about the related head.
    pub repo: RepoRef,
}

/// Information about the base.
#[derive(Clone, Debug, Deserialize)]
pub struct BaseRef {
    /// The github API url of the base.
    #[serde(rename = "ref")]
    pub ref_: String,

    /// The SHA of this base.
    pub sha: String,

    /// Information about the related base.
    pub repo: RepoRef,
}

/// Information about the repository.
#[derive(Clone, Debug, Deserialize)]
pub struct RepoRef {
    /// Numeric Id of this repository.
    pub id: UInt,

    /// The github API url of the repository.
    pub url: Url,

    /// The name of this repository.
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct HeadCommit {
    /// SHA of the head commit.
    pub id: String,

    /// SHA of the tree this commit is a part of.
    pub tree_id: String,

    /// Commit message.
    pub message: String,

    /// Timestamp of this commit.
    pub timestamp: Dt,

    /// Name and email of the commit author.
    pub author: Committer,

    /// Name and email of the commit committer :p
    pub committer: Committer,
}

/// The author of a commit, identified by its name and email.
#[derive(Clone, Debug, Deserialize)]
pub struct Committer {
    pub name: String,
    pub email: String,
}
