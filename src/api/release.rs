use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::common::{
    datetime_opt, default_null, Changes, Dt, Installation, Org, Repo, UrlMap, User,
};

/// The specific actions that a release event has.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseAction {
    /// A release, pre-release, or draft of a release is published.
    Published,

    /// A release or pre-release is deleted.
    Unpublished,

    /// A draft is saved, or a release or pre-release is published without previously
    /// being saved as a draft.
    Created,

    /// A release, pre-release, or draft release is edited.
    Edited,

    /// A release, pre-release, or draft release is deleted.
    Deleted,

    /// A pre-release is created.
    Prereleased,

    /// A release or draft of a release is published, or a pre-release is changed to a
    /// release.
    Release,
}

/// The payload of a release event.
#[derive(Clone, Debug, Deserialize)]
pub struct ReleaseEvent<'a> {
    /// One of `created` or `deleted`.
    pub action: ReleaseAction,

    /// Information about this release.
    #[serde(borrow)]
    pub release: Release<'a>,

    /// Changes are only present if the action was edited.
    #[serde(borrow)]
    pub changes: Option<Changes<'a>>,

    /// Detailed information about the repository that was stared.
    #[serde(borrow)]
    pub repository: Repo<'a>,

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

/// Information about a release.
#[derive(Clone, Debug, Deserialize)]
pub struct Release<'a> {
    /// The Github API url.
    pub url: Url,

    /// The Github assets API url.
    pub assets_url: Url,

    /// The public web page url.
    pub html_url: Url,

    /// Numeric Id of this release.
    pub id: UInt,

    /// String identifier of the release.
    pub node_id: &'a str,

    /// The name of this release tag.
    pub tag_name: &'a str,

    /// The branch this release came from.
    pub target_commitish: &'a str,

    /// The name of this release.
    pub name: Option<&'a str>,

    /// Is this release a draft.
    #[serde(default, deserialize_with = "default_null")]
    pub draft: bool,

    /// The author of this release.
    #[serde(borrow)]
    pub author: User<'a>,

    /// The time in UTC when the release was created.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub created_at: Option<Dt>,

    /// The time in UTC when the release was published.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub published_at: Option<Dt>,

    /// Is this release a pre-release.
    #[serde(default, deserialize_with = "default_null")]
    pub prerelease: bool,

    /// Any assets that go with this release
    #[serde(default, borrow)]
    pub assets: Vec<Assets<'a>>,

    /// The message attached to this release.
    pub body: Option<&'a str>,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}

/// Assets including in a release.
#[derive(Clone, Debug, Deserialize)]
pub struct Assets<'a> {
    /// The Github API url.
    pub url: Url,

    /// The Github API url.
    pub browser_download_url: Url,

    /// Numeric Id of this asset.
    pub id: UInt,

    /// String identifier of the asset.
    pub node_id: &'a str,

    /// The label on this asset.
    pub label: &'a str,

    /// The type of the asset.
    pub content_type: &'a str,

    /// The file name of this asset.
    pub name: Option<&'a str>,

    /// The size of this asset.
    pub size: UInt,

    /// Number of times this asset has been downloaded.
    pub download_count: UInt,

    /// The time in UTC when the release was created.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub created_at: Option<Dt>,

    /// The time in UTC when the release was published.
    #[serde(default, deserialize_with = "datetime_opt")]
    pub uploaded_at: Option<Dt>,

    /// The name of this release.
    #[serde(borrow)]
    pub uploader: Option<User<'a>>,
}
