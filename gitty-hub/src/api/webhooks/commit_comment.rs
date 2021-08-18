use std::path::Path;

use github_derive::StringEnum;
use js_int::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{datetime, AuthorAssociation, Dt, Installation, Org, Repository, User};

/// The payload of a commit comment.
#[derive(Clone, Debug, Deserialize)]
pub struct CommitCommentEvent<'a> {
    /// The action that was performed.
    pub action: CommitCommentAction,

    /// The checks to run.
    #[serde(borrow)]
    pub comment: CommitComment<'a>,

    /// Information about the repositories this app has access to.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// Detailed information about the organization the app
    /// belongs to.
    #[serde(borrow)]
    pub organization: Option<Org<'a>>,

    /// Information about Github app installation.
    ///
    /// This is only present if the event is sent from said app.
    #[serde(borrow)]
    pub installation: Option<Installation<'a>>,

    /// Detailed information about the user of the app.
    #[serde(borrow)]
    pub sender: User<'a>,
}

#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
#[non_exhaustive]
pub enum CommitCommentAction {
    Created,
}

/// The payload of a check run event.
#[derive(Clone, Debug, Deserialize)]
pub struct CommitComment<'a> {
    /// The api url of the pull request.
    pub url: Url,

    /// The public web page url.
    pub html_url: Url,

    /// Numeric Id of this installation.
    pub id: UInt,

    /// String identifier of the repository.
    pub node_id: &'a str,

    /// The user who commented on this commit.
    #[serde(borrow)]
    pub user: User<'a>,

    /// The line index in the diff to which this applies.
    pub position: Option<UInt>,

    /// The line number to which this comment applies.
    ///
    /// This is the last line in a multi-line comment.
    pub line: Option<UInt>,

    /// The relative file path.
    pub path: Option<&'a Path>,

    /// The SHA of this commit.
    pub commit_id: &'a str,

    /// Time in UTC this commit was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// Time in UTC this commit was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,

    /// The associated author of this commit.
    pub author_association: AuthorAssociation,

    /// The body of the message.
    pub body: &'a str,
}
