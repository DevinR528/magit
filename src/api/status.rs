use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::common::{Branch, Commit, Org, Repo, User};

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
pub struct StatusEvent {
    /// The unique identifier of the status.
    pub id: UInt,

    /// The commit sha
    pub sha: String,

    /// One of `pending`, `success`, `failure`, or `error`.
    pub state: StatusState,

    /// Information about this particular commit.
    pub commit: Commit,

    /// The optional human-readable description added to the status.
    pub description: Option<String>,

    /// The optional link added to the status.
    pub target_url: Option<Url>,

    /// A `Vec<Branch>` containing the branches information.
    pub branches: Vec<Branch>,

    /// Detailed information about the repository that was stared.
    pub repository: Repo,

    /// Detailed information about the organization the repo that was stared
    /// belongs to.
    pub organization: Option<Org>,

    /// Detailed information about the user who stared the repo.
    pub sender: User,
}
