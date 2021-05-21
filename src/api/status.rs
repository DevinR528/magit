use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{
    common::{Branch, Commit, Org, Repo, User},
    installation::Installation,
};

/// The state of the status event.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusState {
    Pending,
    Success,
    Failure,
    Error,
}

/// The payload of a status event.
#[derive(Clone, Debug, Deserialize)]
pub struct StatusEvent<'a> {
    /// The unique identifier of the status.
    pub id: UInt,

    /// The commit sha
    pub sha: &'a str,

    /// One of `pending`, `success`, `failure`, or `error`.
    pub state: StatusState,

    /// Information about this particular commit.
    #[serde(borrow)]
    pub commit: Commit<'a>,

    /// The optional human-readable description added to the status.
    pub description: Option<&'a str>,

    /// The optional link added to the status.
    pub target_url: Option<Url>,

    /// A `Vec<Branch>` containing the branches information.
    #[serde(default, borrow)]
    pub branches: Vec<Branch<'a>>,

    /// Detailed information about the repository that was stared.
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
