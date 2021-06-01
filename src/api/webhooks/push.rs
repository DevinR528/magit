use serde::Deserialize;

use crate::api::{Committer, Installation, Org, Repository, User};

/// The payload of a push event.
#[derive(Clone, Debug, Deserialize)]
pub struct PushEvent<'a> {
    /// The unique identifier of the status.
    #[serde(rename = "ref")]
    pub ref_: &'a str,

    /// The commit SHA before the push.
    pub before: &'a str,

    /// The commit SHA after the push.
    pub after: &'a str,

    /// Did this push create a new commit.
    pub created: bool,

    /// Did this push delete previous commits.
    pub deleted: bool,

    /// Was this a force push.
    pub forced: bool,

    /// The SHA of the base.
    pub base_ref: Option<&'a str>,

    /// Compare if changes were made.
    pub compare: &'a str,

    /// The author of the last commit for this push.
    #[serde(borrow)]
    pub pusher: Committer<'a>,

    /// Information about the commits that were pushed.
    #[serde(default, borrow)]
    pub commits: Vec<PushCommit<'a>>,

    /// Information about the commits that were pushed.
    #[serde(borrow)]
    pub head_commit: Option<PushCommit<'a>>,

    /// Detailed information about the repository that was pushed to.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// Information about Github app installation.
    ///
    /// This is only present if the event is sent from said app.
    #[serde(borrow)]
    pub installation: Option<Installation<'a>>,

    /// Detailed information about the organization the repository that was pushed to
    /// belongs to.
    #[serde(borrow)]
    pub organization: Option<Org<'a>>,

    /// Detailed information about the user who pushed to the repository.
    #[serde(borrow)]
    pub sender: User<'a>,
}

/// Information about a specific commit.
#[derive(Clone, Debug, Deserialize)]
pub struct PushCommit<'a> {
    /// The sha of this commit.
    pub id: &'a str,

    /// The identifier of this commit.
    pub tree_id: &'a str,

    /// Is this commit distinct from others.
    pub distinct: bool,

    /// The commit message.
    #[serde(borrow)]
    pub message: &'a str,

    /// The api url of the commit referenced.
    pub url: &'a str,

    /// The author of this commit.
    #[serde(borrow)]
    pub author: Committer<'a>,

    /// The user who committed the referenced commit.
    #[serde(borrow)]
    pub committer: Committer<'a>,

    /// The files that were added.
    #[serde(default)]
    pub added: Vec<&'a str>,

    /// The files that were removed.
    #[serde(default)]
    pub removed: Vec<&'a str>,

    /// The files that were modified.
    #[serde(default)]
    pub modified: Vec<&'a str>,
}
