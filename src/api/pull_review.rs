use matrix_sdk::UInt;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use url::Url;

use crate::api::{
    common::{AuthorAssociation, Dt, Links, Org, Repo, User},
    pull::PullRequest,
};

/// The actions that can be taken for a pull request review.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestReviewAction {
    /// The pull request review has been created.
    ///
    /// Note: This seems to only apply to the PullRequestReview::state field.
    Created,

    /// A pull request review is submitted into a non-pending state.
    Submitted,

    /// The body of a review has been edited.
    Edited,

    /// A review has been dismissed.
    Dismissed,
}

/// The payload of a pull request review event.
#[derive(Clone, Debug, Deserialize)]
pub struct PullRequestReviewEvent {
    /// The action that was performed.
    pub action: PullRequestReviewAction,

    /// The changes to the comment if the action was edited.
    ///
    /// Only present for [`PullAction::Edited`].
    // TODO: what is this
    pub changes: Option<JsonValue>,

    /// Information about the pull request.
    pub pull_request: PullRequest,

    /// The review that was affected.
    pub review: PullRequestReview,

    /// Detailed information about the repository that was stared.
    pub repository: Repo,

    /// Detailed information about the organization the repo that was stared
    /// belongs to.
    pub organization: Option<Org>,

    /// Detailed information about the user who stared the repo.
    pub sender: User,
}

/// The review of a pull request.
#[derive(Clone, Debug, Deserialize)]
pub struct PullRequestReview {
    /// Numeric Id of this review.
    pub id: UInt,

    /// String identifier of the review.
    pub node_id: String,

    /// Information about the owner of this review.
    pub user: User,

    /// The public web page url.
    pub html_url: Url,

    /// Time in UTC this pull request was submitted.
    pub submitted_at: Dt,

    /// The state of the pull request review.
    // TODO: make this an enum
    pub state: String,

    /// The Github api url for the related pull request.
    pub pull_request_url: Url,

    /// The authors association to this repository.
    pub author_association: AuthorAssociation,

    /// All links related to this pull request.
    #[serde(rename = "_links")]
    pub links: Links,
}
