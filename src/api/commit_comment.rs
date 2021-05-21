use std::path::PathBuf;

use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{
    common::{AuthorAssociation, Dt, Org, Repo, User},
    installation::Installation,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")] // TODO: if more variants are added use snake_case
pub enum CommitCommentAction {
    Created,
}

/// The payload of a commit comment.
#[derive(Clone, Debug, Deserialize)]
pub struct CommitCommentEvent {
    /// The action that was performed.
    pub action: CommitCommentAction,

    /// The checks to run.
    pub comment: CommitComment,

    /// Information about the repositories this app has access to.
    pub repository: Repo,

    /// Detailed information about the organization the app
    /// belongs to.
    pub organization: Option<Org>,

    /// Information about Github app installation.
    ///
    /// This is only present if the event is sent from said app.
    pub installation: Option<Installation>,

    /// Detailed information about the user of the app.
    pub sender: User,
}

/// The payload of a check run event.
#[derive(Clone, Debug, Deserialize)]
pub struct CommitComment {
    /// The api url of the pull request.
    pub url: Url,

    /// The public web page url.
    pub html_url: Url,

    /// Numeric Id of this installation.
    pub id: UInt,

    /// String identifier of the repository.
    pub node_id: String,

    /// The user who commented on this commit.
    pub user: User,

    /// The line index in the diff to which this applies.
    pub position: Option<UInt>,

    /// The line number to which this comment applies.
    ///
    /// This is the last line in a multi-line comment.
    pub line: Option<UInt>,

    /// The relative file path.
    pub path: Option<PathBuf>,

    /// The SHA of this commit.
    pub commit_id: String,

    /// Time in UTC this commit was created.
    pub created_at: Dt,

    /// Time in UTC this commit was last updated.
    pub updated_at: Dt,

    /// The associated author of this commit.
    pub author_association: AuthorAssociation,

    /// The body of the message.
    pub body: String,
}
