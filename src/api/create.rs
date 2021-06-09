use serde::Deserialize;

use crate::api::common::{Installation, Org, Repo, User};

/// The type of git object created.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RefType {
    /// A tag git object was created.
    Tag,

    /// A branch git object was created.
    Branch,
}

/// The payload of a create event.
#[derive(Clone, Debug, Deserialize)]
pub struct CreateEvent<'a> {
    /// The action that was performed.
    #[serde(rename = "ref")]
    pub ref_: &'a str,

    /// The type of git object created in the repository.
    pub ref_type: RefType,

    /// The name of the repositories master branch.
    pub master_branch: &'a str,

    /// The repositories current description.
    pub description: Option<&'a str>,

    /// The pusher type for the event.
    pub pusher_type: &'a str,

    /// Information about the repositories this app has access to.
    #[serde(borrow)]
    pub repository: Repo<'a>,

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
