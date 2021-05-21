use serde::Deserialize;

use crate::api::common::{Dt, Org, Repo, User};

/// The specific actions that a star event has.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarAction {
    Created,
    Deleted,
}

/// The payload of a star event.
#[derive(Clone, Debug, Deserialize)]
pub struct StarEvent<'a> {
    /// One of `created` or `deleted`.
    pub action: StarAction,

    /// The time in UTC when this repo was stared.
    pub starred_at: Dt,

    /// Detailed information about the repository that was stared.
    #[serde(borrow)]
    pub repository: Repo<'a>,

    /// Detailed information about the organization the repo that was stared
    /// belongs to.
    #[serde(borrow)]
    pub organization: Option<Org<'a>>,

    /// Detailed information about the user who stared the repo.
    #[serde(borrow)]
    pub sender: User<'a>,
}
