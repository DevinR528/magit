use serde::Deserialize;

use crate::api::common::{Changes, Installation, Milestone, Org, Repo, User};

/// The action that was performed on the milestone.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MilestoneAction {
    /// A new milestone was added.
    Created,

    /// The milestone was closed.
    Closed,

    /// The milestone was opened.
    Opened,

    /// The milestone was edited.
    Edited,

    /// The milestone was deleted.
    Deleted,
}

/// The payload of a delete event.
#[derive(Clone, Debug, Deserialize)]
pub struct MilestoneEvent<'a> {
    /// The action that was performed.
    pub action: MilestoneAction,

    /// The type of git object deleted in the repository.
    #[serde(borrow)]
    pub milestone: Milestone<'a>,

    /// The pusher type for the event.
    #[serde(borrow)]
    pub changes: Option<Changes<'a>>,

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
