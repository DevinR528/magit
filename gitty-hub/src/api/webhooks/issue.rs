use github_derive::StringEnum;
use serde::Deserialize;

use crate::api::{Changes, Installation, Issue, Label, Org, Repository, User};

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
    pub repository: Repository<'a>,

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

/// The actions that can be taken for an issue event.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
#[non_exhaustive]
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
