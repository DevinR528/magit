use github_derive::{github_rest_api, Incoming, StringEnum};
use ruma::UInt;
use serde::Deserialize;

use crate::api::rest::ApplicationV3Json;

github_rest_api! {
    metadata: {
        description: "Merge a pull request",
        method: PUT,
        path: "/repos/:owner/:repo/pulls/:pull_number/merge",
        name: "merge_pull_request",
        authentication: true,
    }

    request: {
        /// Optional accept header to enable preview features.
        #[github(header = ACCEPT)]
        pub accept: Option<ApplicationV3Json>,

        /// The owner of this repository.
        #[github(path)]
        pub owner: &'a str,

        /// The name of this repository.
        #[github(path)]
        pub repo: &'a str,

        /// The pull request number.
        #[github(path)]
        pub pull_number: UInt,

        /// The title of this commit for the merge.
        #[github(body)]
        pub commit_title: &'a str,

        /// Extra detail to append to automatic commit message.
        #[github(body)]
        pub commit_message: &'a str,

        /// SHA that pull request head must match to allow merge.
        #[github(body)]
        pub sha: &'a str,

        /// Merge method to use.
        #[github(body)]
        pub merge_method: MergeMethod,
    }

    response: {
        /// Information about the merge.
        #[serde(flatten)]
        pub repository: IncomingMerge,
    }
}

/// Information about a file.
#[derive(Clone, Debug, StringEnum)]
pub enum MergeMethod {
    Merge,
    Squash,
    Rebase,
}

#[derive(Clone, Debug, Deserialize, Incoming)]
pub struct Merge<'a> {
    /// The commit message.
    pub message: &'a str,

    /// The SHA of this commit on a branch.
    pub sha: &'a str,

    /// Was the merge successful.
    pub merged: bool,
}

#[test]
fn merge_pull_request() {
    let json = include_str!("../../../../test_json/rest/merge_pull_request.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
