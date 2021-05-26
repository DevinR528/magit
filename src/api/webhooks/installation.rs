use matrix_sdk::UInt;
use serde::Deserialize;

use crate::api::{Installation, Org, User};

/// The actions that can be taken in an installation event.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallationAction {
    /// Someone installs a GitHub App.
    Created,

    /// Someone uninstalls a GitHub App.
    Deleted,

    /// Someone suspends a GitHub App installation.
    Suspend,

    /// Someone un-suspends a GitHub App installation.
    Unsuspend,

    /// Someone accepts new permissions for a GitHub App installation.
    NewPermissionsAccepted,
}

/// The payload of an installation event.
#[derive(Clone, Debug, Deserialize)]
pub struct InstallationEvent<'a> {
    /// The action that was performed.
    pub action: InstallationAction,

    /// The GitHub App installation.
    #[serde(borrow)]
    pub installation: Installation<'a>,

    /// Brief information about the repositories this app has access to.
    #[serde(default, borrow)]
    pub repositories: Vec<ShortRepo<'a>>,

    /// Detailed information about the organization the app
    /// belongs to.
    pub organization: Option<Org<'a>>,

    /// Detailed information about the user of the app.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// Detailed information about the requester of the app.
    pub requester: Option<User<'a>>,
}

/// Information about repositories that the installation can access.
#[derive(Clone, Debug, Deserialize)]
pub struct ShortRepo<'a> {
    /// Numeric Id of this repository.
    pub id: UInt,

    /// String identifier of the repository.
    pub node_id: &'a str,

    /// The name of this repository.
    pub name: &'a str,

    /// The full name of this repository ie `owner/repo`.
    pub full_name: &'a str,

    /// Whether the repository is private or public.
    pub private: bool,
}
