use github_derive::StringEnum;
use serde::Deserialize;

use crate::api::{Dt, Installation, Org, Repository, User};

/// The payload of a star event.
#[derive(Clone, Debug, Deserialize)]
pub struct StarEvent<'a> {
    /// One of `created` or `deleted`.
    pub action: StarAction,

    /// The time in UTC when this repo was stared.
    pub starred_at: Dt,

    /// Detailed information about the repository that was stared.
    #[serde(borrow)]
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

/// The specific actions that a star event has.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StarAction {
    /// A star was added to this repository.
    Created,

    /// The star was deleted from this repository.
    Deleted,
}
