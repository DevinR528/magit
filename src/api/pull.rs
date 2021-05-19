use matrix_sdk::UInt;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use url::Url;

use crate::api::common::{
    Dt, IssueState, Label, Org, Repo, RepoPermission, UrlMap, User,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullAction {
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
}

#[derive(Clone, Debug, Deserialize)]
pub struct PullEvent {
    /// The action that was performed.
    pub action: PullAction,

    /// The pull request number.
    pub number: UInt,

    /// The changes to the comment if the action was edited.
    ///
    /// Only present for [`PullAction::Edited`].
    // TODO: what is this
    pub changes: Option<JsonValue>,

    /// Information about the pull request.
    pub pull_request: PullRequest,

    /// Detailed information about the repository that was stared.
    pub repository: Repo,

    /// Detailed information about the organization the repo that was stared
    /// belongs to.
    pub organization: Option<Org>,

    /// Detailed information about the user who stared the repo.
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PullRequest {
    /// The api url of the pull request.
    pub url: String,

    /// Numeric Id of this repository.
    pub id: UInt,

    /// String identifier of the repository.
    pub node_id: String,

    /// The public html web page url.
    pub html_url: Url,

    /// The url of the diff.
    pub diff_url: Url,

    /// The url of the patch.
    pub patch_url: Url,

    /// Pull request number.
    pub number: UInt,

    /// State of this pull request.
    pub state: IssueState,

    /// Is this pull request locked.
    #[serde(default)]
    pub locked: bool,

    /// The title of this pull request.
    pub title: String,

    /// Information about the user.
    pub user: User,

    // TODO: confirm
    /// The body of the commit message.
    pub body: String,

    /// Time in UTC this pull request was created.
    pub created_at: Dt,

    /// Time in UTC this pull request was last updated.
    pub updated_at: Dt,

    /// Time in UTC this pull request was closed.
    pub closed_at: Option<Dt>,

    /// Time in UTC this pull request was last updated.
    pub merged_at: Option<Dt>,

    /// The SHA of the merge commit, if any.
    pub merge_commit_sha: Option<String>,

    /// The association of the user who opened the pull request.
    pub author_association: String,

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
    pub assignee: Option<User>,

    /// The `User`s assigned to the pull request.
    #[serde(default)]
    pub assignees: Vec<User>,

    /// The `User` requested to review the pull request.
    #[serde(default)]
    pub requested_reviewers: Vec<User>,

    /// The `Team`s requested to review the pull request.
    #[serde(default)]
    pub requested_teams: Vec<Team>,

    /// The labels that have been added to this pull request.
    #[serde(default)]
    pub labels: Vec<Label>,

    /// Milestones that have been added.
    #[serde(default)]
    pub milestones: Vec<Milestone>,

    /// Information about the head of this commit.
    pub head: Head,

    /// Information about the base commit.
    pub base: Base,

    /// Information about the repository this pull request is against.
    pub repository: Option<Repo>,
}

#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Head {
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    pub user: Option<User>,
    pub repo: Option<Repo>,
}

#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Base {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    pub user: User,
    pub repo: Option<Repo>,
}

#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Milestone {
    /// Numeric Id of this milestone.
    pub id: UInt,

    /// String identifier of the milestone.
    pub node_id: String,

    /// The name of this milestone.
    pub name: String,

    /// Information about the creator of this milestone.
    pub creator: User,

    /// The public html web page url.
    pub html_url: Url,

    /// The url to the github api of this repo.
    pub url: String,

    /// The url to the github api labels requests.
    pub labels_url: String,

    /// Description of the repo.
    pub description: Option<String>,

    /// The number this milestone is.
    pub number: UInt,

    /// The state of this milestone.
    pub state: Option<IssueState>,

    /// The title of this milestone.
    pub title: String,

    /// The number of open issues related to this milestone.
    #[serde(default)]
    pub open_issues: UInt,

    /// The number of closed issues related to this milestone.
    #[serde(default)]
    pub closed_issues: UInt,

    /// The time in UTC when the milestone was created.
    pub created_at: Dt,

    /// The time in UTC when the milestone was last updated.
    pub updated_at: Option<Dt>,

    /// The time in UTC when the milestone was closed.
    pub closed_at: Option<Dt>,

    /// The time in UTC when the milestone is due.
    pub due_on: Option<Dt>,
}

#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Team {
    /// Numeric Id of this team.
    pub id: UInt,

    /// String identifier of the team.
    pub node_id: String,

    /// The name of this team.
    pub name: String,

    /// The slug of this team.
    pub slug: String,

    /// The public web page url.
    pub html_url: Url,

    /// The url to the github api of this repo.
    pub url: String,

    /// Description of the repo.
    pub description: Option<String>,

    /// The privacy this team is.
    pub privacy: String,

    /// Permissions required for this team.
    pub permissions: RepoPermission,

    /// The title of this team.
    pub title: String,

    /// The number of members on this team.
    #[serde(default)]
    pub members_count: UInt,

    /// The time in UTC when the team was created.
    pub created_at: Option<Dt>,

    /// The time in UTC when the team was last updated.
    pub updated_at: Option<Dt>,

    /// The time in UTC when the team was closed.
    pub organization: Option<Org>,

    /// The time in UTC when the team is due.
    pub parent: Option<Box<Team>>,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}
