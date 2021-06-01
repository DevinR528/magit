use std::borrow::Cow;

use github_derive::{github_rest_api, Incoming};
use ruma::UInt;
use serde::Deserialize;

use crate::api::{rest::ApplicationV3Json, FileStatus};

github_rest_api! {
    metadata: {
        description: "List all the files of this pull request",
        method: GET,
        path: "/repos/:owner/:repo/pulls/:pull_number/files",
        name: "list_pull_requests_files",
        authentication: true,
    }

    request: {
        /// Optional accept header to enable preview features.
        #[github(header = ACCEPT)]
        pub accept: Option<ApplicationV3Json>,

        /// The owner of this repository.
        #[github(path)]
        pub owner: &'a str,

        /// The name of the repository.
        #[github(path)]
        pub repo: &'a str,


        /// The number of the pull request.
        #[github(path)]
        pub pull_number: UInt,

        /// Result per page.
        ///
        /// Defaults to 30 and max 100.
        #[github(query)]
        #[serde(serialize_with = "crate::api::rest::per_page")]
        pub per_page: Option<UInt>,

        /// Which page of the results to return.
        ///
        /// Defaults to 1.
        #[github(query)]
        #[serde(serialize_with = "crate::api::rest::page")]
        pub page: Option<UInt>,
    }

    #[github(with = pr_files)]
    response: {
        /// A list of all the commits of a pull request.
        pub pr_files: Vec<IncomingFile>,
    }
}

/// Information about a file.
#[derive(Clone, Debug, Deserialize, Incoming)]
pub struct File<'a> {
    /// The sha of this commit to this file.
    pub sha: &'a str,

    /// The name of this file.
    pub filename: &'a str,

    /// The git status of the file.
    pub status: FileStatus,

    /// Number of additions.
    #[serde(default)]
    pub additions: UInt,

    /// Number of deletions.
    #[serde(default)]
    pub deletions: UInt,

    /// Number of review changed files.
    #[serde(default)]
    pub changes: UInt,

    /// The public blob url.
    pub blob_url: &'a str,

    /// The public url to the raw content.
    pub raw_url: &'a str,

    /// The github api content url.
    pub contents_url: &'a str,

    /// The patch info from git.
    pub patch: Cow<'a, str>,
}

pub fn pr_files<'de, D>(deser: D) -> Result<Response, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Response { pr_files: serde::Deserialize::deserialize(deser)? })
}

#[test]
fn pull_request_files() {
    let json = include_str!("../../../../test_json/rest/list_pull_request_files.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
