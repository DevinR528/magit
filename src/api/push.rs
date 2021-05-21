use serde::Deserialize;

use crate::api::{
    common::{Commit, Committer, Org, Repo, User},
    installation::Installation,
};

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
    pub commits: Vec<Commit<'a>>,

    /// Information about the commits that were pushed.
    #[serde(borrow)]
    pub head_commit: Option<Commit<'a>>,

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
