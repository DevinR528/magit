use github_derive::StringEnum;
use js_int::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{Branch, Commit, Installation, Org, Repository, User};

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
    pub repository: Repository<'a>,

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

/// The state of the status event.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StatusState {
    /// Status is pending.
    Pending,

    /// Status is success.
    Success,

    /// Status is failure.
    Failure,

    /// Status has error-ed.
    Error,
}
