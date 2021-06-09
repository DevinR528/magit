use ruma::{serde::StringEnum, UInt};
use serde::Deserialize;
use url::Url;

use crate::api::common::{
    datetime_opt, default_null, AuthorAssociation, Base, Changes, Dt, Head, Installation,
    IssueState, Label, Links, Milestone, Org, Repo, Team, UrlMap, User,
};

/// The actions that can be taken for a pull request.
#[derive(Clone, Debug, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
pub enum PullRequestAction {
    /// Reviewer assigned.
    Assigned,

    /// Automatic merging is disabled.
    AutoMergeDisabled,

    /// Automatic merging is enabled.
    AutoMergeEnabled,

    /// If the action is closed and the merged key is false, the pull request was closed
    /// with unmerged commits. If the action is closed and the merged key is true, the
    /// pull request was merged.
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

    #[doc(hidden)]
    _Custom(String),
}

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

#[derive(Clone, Debug, Deserialize)]
pub struct PullRequest<'a> {
    /// The api url of the pull request.
    pub url: &'a str,

    /// Numeric Id of this repository.
    pub id: UInt,

    /// String identifier of the repository.
    pub node_id: &'a str,

    /// The public web page url.
    pub html_url: Url,

    /// The url of the diff.
    pub diff_url: Url,

    /// The url of the patch.
    pub patch_url: Url,

    /// Pull request number.
    pub number: UInt,

    /// State of this pull request.
    #[serde(default, deserialize_with = "default_null")]
    pub state: IssueState,

    /// Is this pull request locked.
    #[serde(default)]
    pub locked: bool,

    /// The title of this pull request.
    pub title: &'a str,

    /// Information about the user.
    #[serde(borrow)]
    pub user: User<'a>,

    /// The body of the pull request message.
    pub body: &'a str,

    /// Time in UTC this pull request was created.
    pub created_at: Dt,

    /// Time in UTC this pull request was last updated.
    pub updated_at: Dt,

    /// Time in UTC this pull request was closed.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub closed_at: Option<Dt>,

    /// Time in UTC this pull request was last updated.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub merged_at: Option<Dt>,

    /// The user who merged this pull request.
    pub merged_by: Option<&'a str>,

    /// The SHA of the merge commit, if any.
    pub merge_commit_sha: Option<&'a str>,

    /// The association of the user who opened the pull request.
    pub author_association: AuthorAssociation,

    /// Is this pull request a draft.
    pub draft: Option<bool>,

    /// Has this pull request been merged.
    pub merged: Option<bool>,

    /// Is the pull request in a mergeable state.
    pub mergeable: Option<bool>,

    /// Is the pull request in a rebaseable state.
    pub rebaseable: Option<bool>,

    /// Can the maintainer of the repository modify this pull request.
    pub maintainer_can_modify: Option<bool>,

    /// The state of mergeability of this pull request.
    pub mergeable_state: Option<IssueState>,

    /// Number of comments.
    #[serde(default)]
    pub comments: UInt,

    /// Number of review comments.
    #[serde(default)]
    pub review_comments: UInt,

    /// Number of commits.
    #[serde(default)]
    pub commits: UInt,

    /// Number of review additions.
    #[serde(default)]
    pub additions: UInt,

    /// Number of review deletions.
    #[serde(default)]
    pub deletions: UInt,

    /// Number of review changed files.
    #[serde(default)]
    pub changed_files: UInt,

    /// The `User` assigned to the pull request.
    #[serde(borrow)]
    pub assignee: Option<User<'a>>,

    /// The `User`s assigned to the pull request.
    #[serde(default, borrow)]
    pub assignees: Vec<User<'a>>,

    /// The `User` requested to review the pull request.
    #[serde(default, borrow)]
    pub requested_reviewers: Vec<User<'a>>,

    /// The `Team`s requested to review the pull request.
    #[serde(default, borrow)]
    pub requested_teams: Vec<Team<'a>>,

    /// The labels that have been added to this pull request.
    #[serde(default, borrow)]
    pub labels: Vec<Label<'a>>,

    /// Milestones that have been added.
    #[serde(default, borrow)]
    pub milestones: Vec<Milestone<'a>>,

    /// Information about the head of this commit.
    #[serde(borrow)]
    pub head: Head<'a>,

    /// Information about the base branch.
    #[serde(borrow)]
    pub base: Base<'a>,

    /// Information about the repository this pull request is against.
    #[serde(borrow)]
    pub repository: Option<Repo<'a>>,

    /// All links related to this pull request.
    #[serde(rename = "_links", borrow)]
    pub links: Links<'a>,

    /// A map of all the github api urls.
    ///
    /// [`PullRequest`] has only a few REST api urls, they relate to commits, reviews,
    /// and issues.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}
