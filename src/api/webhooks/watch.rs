use github_derive::StringEnum;
use serde::Deserialize;

use crate::api::{Installation, Org, Repository, User};

/// The payload of a delete event.
#[derive(Clone, Debug, Deserialize)]
pub struct WatchEvent<'a> {
    /// The action that was performed.
    pub action: WatchAction,

    /// Information about the repository being watched.
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

    /// Detailed information about the user that triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,
}

/// The action that was performed.
///
/// Currently can only be started.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
pub enum WatchAction {
    /// A new watcher was added.
    Started,
}
