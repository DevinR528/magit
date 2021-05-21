use std::borrow::Cow;

use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::common::{
    AccessPermissions, Dt, EventKind, Org, RepoSelection, Type, UrlMap, User,
};

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

#[derive(Clone, Debug, Deserialize)]
pub struct Installation<'a> {
    /// Numeric Id of this installation.
    pub id: UInt,

    /// Detailed information about the user who installed the app.
    #[serde(borrow)]
    pub account: User<'a>,

    /// Whether all repositories are selected or only a few.
    pub repository_selection: RepoSelection,

    /// The public web page url.
    pub html_url: Url,

    /// Numeric identifier of the installed app.
    pub app_id: UInt,

    /// Numeric identifier for the app target.
    pub target_id: UInt,

    /// The type this app targets.
    pub target_type: Type,

    /// The permissions the app is given for each section.
    pub permissions: AccessPermissions,

    /// Events this app has access to.
    pub events: Vec<EventKind>,

    /// Time in UTC this app was created.
    #[serde(deserialize_with = "datetime")]
    pub created_at: Dt,

    /// Time in UTC this app was last updated.
    #[serde(deserialize_with = "datetime")]
    pub updated_at: Dt,

    /// The configuration file for this installed app.
    pub single_file_name: &'a str,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
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

fn datetime<'de, D>(deser: D) -> Result<Dt, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de::Error;

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum StringOrUInt<'a> {
        UInt(i64),
        String(Cow<'a, str>),
    }

    let ts = StringOrUInt::deserialize(deser)?;
    println!("{:?}", ts);
    match ts {
        StringOrUInt::UInt(timestamp) => Ok(Dt::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0)
                .ok_or_else(|| D::Error::custom("timestamp exceeded bounds"))?,
            chrono::Utc,
        )),
        StringOrUInt::String(datetime) => datetime.parse().map_err(D::Error::custom),
    }
}
