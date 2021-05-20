use matrix_sdk::UInt;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use url::Url;

use crate::api::common::{Dt, IssueState, Label, LockReason, Milestone, Org, Repo, User};

/// The actions that can be taken for an issue event.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueAction {
    /// Open an issue.
    Opened,

    /// The issue has been edited.
    Edited,

    /// The issue has been deleted.
    Deleted,

    /// The issue has been pinned.
    Pinned,

    /// The issue has been unpinned.
    Unpinned,

    /// The issue has been closed.
    Closed,

    /// The issue has been reopened.
    Reopened,

    /// The issue has been assigned.
    Assigned,

    /// The issue has been unassigned.
    Unassigned,

    /// A label has been added.
    Labeled,

    /// A label has been removed.
    Unlabeled,

    /// The issue has been locked.
    Locked,

    /// The issue has been unlocked.
    Unlocked,

    /// The issue has been transferred.
    Transferred,

    /// A milestone has been added to this issue.
    Milestoned,

    /// A milestone has been removed from this issue.
    Demilestoned,
}

/// The payload of an issue event.
#[derive(Clone, Debug, Deserialize)]
pub struct IssueEvent {
    /// The action that was performed.
    pub action: IssueAction,

    /// Information about the issue.
    pub issue: Issue,

    /// The changes to the comment if the action was edited.
    ///
    /// Only present for [`crate::api::common::PullAction::Edited`].
    // TODO: what is this
    pub changes: Option<JsonValue>,

    /// The [`User`] who is assigned this issue.
    pub assignee: Option<User>,

    /// The [`Label`] assigned to this issue.
    pub label: Option<Label>,

    /// Detailed information about the repository that was stared.
    pub repository: Repo,

    /// Detailed information about the organization the repo that was stared
    /// belongs to.
    pub organization: Option<Org>,

    /// Detailed information about the user who stared the repo.
    pub sender: User,
}

/// Information about an issue.
///
/// This can be used to represent pull request related responses.
#[derive(Clone, Debug, Deserialize)]
pub struct Issue {
    /// Numeric Id of this repository.
    pub id: UInt,

    /// String identifier of the repository.
    pub node_id: String,

    /// The api url of the issue.
    pub url: String,

    /// The public web page url.
    pub html_url: Url,

    /// Issue number.
    pub number: UInt,

    /// State of this issue.
    pub state: IssueState,

    /// Is this issue locked.
    #[serde(default)]
    pub locked: bool,

    /// The title of this issue.
    pub title: String,

    /// Information about the user.
    pub user: User,

    /// The body of the issue.
    pub body: String,

    /// A list of labels attached to this issue.
    #[serde(default)]
    pub labels: Vec<Label>,

    /// The [`User`] who is assigned to this issue.
    pub assignee: Option<User>,

    /// The [`User`]s who are assigned to this issue.
    #[serde(default)]
    pub assignees: Vec<User>,

    /// Milestone that have been added.
    pub milestone: Option<Milestone>,

    /// Number of comments.
    #[serde(default)]
    pub comments: UInt,

    /// Information about any linked pull requests.
    pub pull_request: Option<IssuePullRequest>,

    /// Time in UTC this pull request was created.
    pub created_at: Dt,

    /// Time in UTC this pull request was last updated.
    pub updated_at: Dt,

    /// Time in UTC this pull request was closed.
    pub closed_at: Option<Dt>,

    /// The author associated with this issue.
    pub author_association: String,

    /// The reason this issue was locked.
    pub active_lock_reason: LockReason,
}

/// Information about an pull requests linked to this issue.
#[derive(Clone, Debug, Deserialize)]
pub struct IssuePullRequest {
    /// The api url of the pull request.
    pub url: String,

    /// The public web page url.
    pub html_url: Url,

    /// The url of the diff.
    pub diff_url: Url,

    /// The url of the patch.
    pub patch_url: Url,
}
