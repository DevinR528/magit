use serde::Deserialize;

use crate::api::{webhooks::create::RefType, Installation, Org, Repository, User};

/// The payload of a delete event.
#[derive(Clone, Debug, Deserialize)]
pub struct DeleteEvent<'a> {
    /// The action that was performed.
    #[serde(rename = "ref")]
    pub ref_: &'a str,

    /// The type of git object deleted in the repository.
    pub ref_type: RefType,

    /// The pusher type for the event.
    pub pusher_type: &'a str,

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
