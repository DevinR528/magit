use github_derive::StringEnum;
use js_int::UInt;
use serde::Deserialize;

use crate::api::{Changes, Installation, Org, PullRequest, Repository, User};

/// The payload of a pull request event.
#[derive(Clone, Debug, Deserialize)]
pub struct PullRequestEvent<'a> {
    /// The action that was performed.
    pub action: PullRequestAction,

    /// The pull request number.
    pub number: UInt,

    /// The changes to the comment if the action was edited.
    ///
    /// Only present for [`PullAction::Edited`].
    #[serde(borrow)]
    pub changes: Option<Changes<'a>>,

    /// Information about the pull request.
    #[serde(borrow)]
    pub pull_request: PullRequest<'a>,

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

/// The actions that can be taken for a pull request.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "snake_case")]
pub enum PullRequestAction {
    /// Reviewer assigned.
    Assigned,

    /// Automatic merging is disabled.
    AutoMergeDisabled,

    /// Automatic merging is enabled.
    AutoMergeEnabled,

    /// If the action is closed and the merged key is false, the pull request was
    /// closed with unmerged commits. If the action is closed and the merged key
    /// is true, the pull request was merged.
    Closed,

    /// Convert this pull request to a draft.
    ConvertToDraft,

    /// Edited the pull request.
    Edited,

    /// Added a label to the pull request.
    Labeled,

    /// Locked all further changes.
    Locked,

    /// Opened a new pull request.
    Opened,

    /// Marked as ready for review.
    ReadyForReview,

    /// Reopened a pull request.
    Reopened,

    /// A review requested has been removed.
    ReviewRequestedRemoved,

    /// A review has been requested.
    ReviewRequested,

    /// Pull request has been synchronized.
    Synchronize,

    /// Pull request has been unassigned.
    Unassigned,

    /// Pull request has been unlabeled.
    Unlabeled,

    /// Pull request has been unlocked.
    Unlocked,
}
