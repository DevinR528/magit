use github_derive::StringEnum;
use js_int::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{
    AuthorAssociation, Changes, Dt, Installation, Links, Org, PullRequest, Repository,
    User,
};

/// The payload of a pull request review event.
#[derive(Clone, Debug, Deserialize)]
pub struct PullRequestReviewEvent<'a> {
    /// The action that was performed.
    pub action: PullRequestReviewAction,

    /// The changes to the comment if the action was edited.
    ///
    /// Only present for [`PullAction::Edited`].
    #[serde(borrow)]
    pub changes: Option<Changes<'a>>,

    /// Information about the pull request.
    #[serde(borrow)]
    pub pull_request: PullRequest<'a>,

    /// The review that was affected.
    #[serde(borrow)]
    pub review: PullRequestReview<'a>,

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
    pub organization: Option<Org<'a>>,

    /// Detailed information about the user who stared the repo.
    #[serde(borrow)]
    pub sender: User<'a>,
}

/// The actions that can be taken for a pull request review.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
#[non_exhaustive]
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

/// The review of a pull request.
#[derive(Clone, Debug, Deserialize)]
pub struct PullRequestReview<'a> {
    /// Numeric Id of this review.
    pub id: UInt,

    /// String identifier of the review.
    pub node_id: &'a str,

    /// Information about the owner of this review.
    #[serde(borrow)]
    pub user: User<'a>,

    /// The public web page url.
    pub html_url: Url,

    /// Time in UTC this pull request was submitted.
    pub submitted_at: Dt,

    /// The state of the pull request review.
    // TODO: make this an enum
    pub state: &'a str,

    /// The Github api url for the related pull request.
    pub pull_request_url: Url,

    /// The authors association to this repository.
    pub author_association: AuthorAssociation,

    /// All links related to this pull request.
    #[serde(rename = "_links", borrow)]
    pub links: Links<'a>,
}
