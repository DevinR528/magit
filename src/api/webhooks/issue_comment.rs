use std::path::Path;

use github_derive::StringEnum;
use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{
    datetime, App, AuthorAssociation, Changes, Comment, Dt, Installation, Issue, Label,
    Org, Repository, User,
};

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
    pub repository: Repository<'a>,

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

/// The actions that can be taken for an issue comment event.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
pub enum IssueCommentAction {
    /// Created an issue.
    Created,

    /// The issue has been edited.
    Edited,

    /// The issue has been deleted.
    Deleted,
}
