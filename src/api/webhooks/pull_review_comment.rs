use std::{borrow::Cow, path::Path};

use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{
    datetime, webhooks::pull::PullRequest, AuthorAssociation, Changes, Dt, Installation,
    Links, Org, Repo, User,
};

/// The actions that can be taken for a pull request review.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestReviewCommentAction {
    /// The pull request review has been created.
    Created,

    /// The body of a review has been edited.
    Edited,

    /// A review has been dismissed.
    Dismissed,
}

/// The payload of a pull request review comment event.
#[derive(Clone, Debug, Deserialize)]
pub struct PullRequestReviewCommentEvent<'a> {
    /// The action that was performed.
    pub action: PullRequestReviewCommentAction,

    /// The changes to the comment if the action was edited.
    ///
    /// Only present for [`PullAction::Edited`].
    #[serde(borrow)]
    pub changes: Option<Changes<'a>>,

    /// The pull request review comment.
    #[serde(borrow)]
    pub comment: PullRequestReviewComment<'a>,

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
    pub organization: Option<Org<'a>>,

    /// Detailed information about the user who stared the repo.
    #[serde(borrow)]
    pub sender: User<'a>,
}

/// The review of a pull request.
#[derive(Clone, Debug, Deserialize)]
pub struct PullRequestReviewComment<'a> {
    /// Numeric Id of this review comment.
    pub id: UInt,

    /// Numeric Id of this pull request review to which the comment belongs.
    pub pull_request_review_id: UInt,

    /// String identifier of the review.
    pub node_id: &'a str,

    /// The public web page url.
    pub html_url: Url,

    /// The Github API url.
    pub url: Url,

    /// The diff of the line tat the comment refers to.
    #[serde(borrow)]
    pub diff_hunk: Cow<'a, str>,

    /// The user who commented on this commit.
    #[serde(borrow)]
    pub user: User<'a>,

    /// The line index in the diff to which this applies.
    pub position: Option<UInt>,

    /// The line index in the diff to which this applies.
    pub original_position: UInt,

    /// The relative file path.
    pub path: Option<&'a Path>,

    /// The SHA of this commit.
    pub commit_id: &'a str,

    /// The SHA of the original commit.
    pub original_commit_id: &'a str,

    /// Time in UTC this commit was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// Time in UTC this commit was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,

    /// The body of the message.
    pub body: &'a str,

    /// The Github api url for the related pull request.
    pub pull_request_url: Url,

    /// The authors association to this repository.
    pub author_association: AuthorAssociation,

    /// All links related to this pull request.
    #[serde(rename = "_links", borrow)]
    pub links: Links<'a>,

    /// The line number which this comment starts.
    pub start_line: Option<UInt>,

    /// The line number which this comment originally starts on.
    pub original_start_line: Option<UInt>,

    /// The side the comment starts.
    pub start_side: Option<CommentSide>,

    /// The line number to which this comment applies.
    ///
    /// This is the last line in a multi-line comment.
    pub line: Option<UInt>,

    /// The line number to which this comment originally applied.
    pub original_line: Option<UInt>,

    /// The side this comment starts on.
    pub side: Option<CommentSide>,

    /// The line number to which this comment originally applied.
    pub in_reply_to_id: Option<UInt>,
}

/// The side a multi-line comment starts on.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CommentSide {
    /// A multi-line comment starts on the left side.
    Left,

    /// A multi-line comment starts on the right side.
    Right,
}
