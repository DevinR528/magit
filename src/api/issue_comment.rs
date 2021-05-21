use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{
    common::{datetime, App, AuthorAssociation, Changes, Dt, Label, Org, Repo, User},
    installation::Installation,
    issue::Issue,
};

/// The actions that can be taken for an issue comment event.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueCommentAction {
    /// Created an issue.
    Created,

    /// The issue has been edited.
    Edited,

    /// The issue has been deleted.
    Deleted,
}

/// The payload of an issue event.
#[derive(Clone, Debug, Deserialize)]
pub struct IssueCommentEvent<'a> {
    /// The action that was performed.
    pub action: IssueCommentAction,

    /// Information about the issue.
    #[serde(borrow)]
    pub issue: Issue<'a>,

    /// The comment that triggered the event.
    #[serde(borrow)]
    pub comment: Comment<'a>,

    /// The changes to the comment if the action was edited.
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

    /// Detailed information about the organization the repo that was stared
    /// belongs to.
    #[serde(borrow)]
    pub organization: Option<Org<'a>>,

    /// Information about Github app installation.
    ///
    /// This is only present if the event is sent from said app.
    #[serde(borrow)]
    pub installation: Option<Installation<'a>>,

    /// Detailed information about the user who stared the repo.
    #[serde(borrow)]
    pub sender: User<'a>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Comment<'a> {
    /// Numeric identifier of the comment.
    pub id: UInt,

    /// String identifier of the comment.
    pub node_id: &'a str,

    /// The Github api url.
    pub url: Url,

    /// The public web page url.
    pub html_url: Url,

    /// The body of the comment.
    pub body: Option<&'a str>,

    /// The user who wrote the comment.
    #[serde(borrow)]
    pub user: User<'a>,

    /// Time in UTC this comment was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// Time in UTC this comment was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,

    /// The association of the author to the repository.
    pub author_association: AuthorAssociation,

    /// If present this comment was generated by a github app.
    #[serde(borrow)]
    pub app: Option<App<'a>>,
}