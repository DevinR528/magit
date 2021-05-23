use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::common::{
    datetime, datetime_opt, AuthorAssociation, Changes, Dt, Installation, IssueState,
    Label, LockReason, Milestone, Org, Repo, User,
};

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
pub struct IssueEvent<'a> {
    /// The action that was performed.
    pub action: IssueAction,

    /// Information about the issue.
    #[serde(borrow)]
    pub issue: Issue<'a>,

    /// The changes to the comment if the action was edited.
    ///
    /// Only present for [`crate::api::common::PullAction::Edited`].
    #[serde(borrow)]
    pub changes: Option<Changes<'a>>,

    /// The [`User`] who is assigned this issue.
    #[serde(borrow)]
    pub assignee: Option<User<'a>>,

    /// The [`Label`] assigned to this issue.
    #[serde(borrow)]
    pub label: Option<Label<'a>>,

    /// Detailed information about the repository that was stared.
    #[serde(borrow)]
    pub repository: Repo<'a>,

    /// Information about Github app installation.
    ///
    /// This is only present if the event is sent from said app.
    #[serde(borrow)]
    pub installation: Option<Installation<'a>>,

    /// Detailed information about the organization the repo that was stared
    /// belongs to.
    #[serde(borrow)]
    pub organization: Option<Org<'a>>,

    /// Detailed information about the user who stared the repo.
    #[serde(borrow)]
    pub sender: User<'a>,
}

/// Information about an issue.
///
/// This can be used to represent pull request related responses.
#[derive(Clone, Debug, Deserialize)]
pub struct Issue<'a> {
    /// Numeric Id of this repository.
    pub id: UInt,

    /// String identifier of the repository.
    pub node_id: &'a str,

    /// The api url of the issue.
    pub url: &'a str,

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
    pub title: &'a str,

    /// Information about the user.
    #[serde(borrow)]
    pub user: User<'a>,

    /// The body of the issue.
    pub body: &'a str,

    /// A list of labels attached to this issue.
    #[serde(default, borrow)]
    pub labels: Vec<Label<'a>>,

    /// The [`User`] who is assigned to this issue.
    #[serde(borrow)]
    pub assignee: Option<User<'a>>,

    /// The [`User`]s who are assigned to this issue.
    #[serde(default, borrow)]
    pub assignees: Vec<User<'a>>,

    /// Milestone that have been added.
    #[serde(borrow)]
    pub milestone: Option<Milestone<'a>>,

    /// Number of comments.
    #[serde(default)]
    pub comments: UInt,

    /// Information about any linked pull requests.
    #[serde(borrow)]
    pub pull_request: Option<IssuePullRequest<'a>>,

    /// Time in UTC this pull request was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// Time in UTC this pull request was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,

    /// Time in UTC this pull request was closed.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub closed_at: Option<Dt>,

    /// The author associated with this issue.
    pub author_association: AuthorAssociation,

    /// The reason this issue was locked.
    pub active_lock_reason: Option<LockReason>,
}

/// Information about an pull requests linked to this issue.
#[derive(Clone, Debug, Deserialize)]
pub struct IssuePullRequest<'a> {
    /// The api url of the pull request.
    pub url: &'a str,

    /// The public web page url.
    pub html_url: Url,

    /// The url of the diff.
    pub diff_url: Url,

    /// The url of the patch.
    pub patch_url: Url,
}
