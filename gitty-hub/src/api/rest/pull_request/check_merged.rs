use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::rest::ApplicationV3Json;

github_rest_api! {
    metadata: {
        description: "",
        method: GET,
        path: "/repos/:owner/:repo/pulls/:pull_number",
        name: "get_repository",
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
    }

    /// A response of 204 means the pull request was merged, 404 means the pull request has yet to be merged.
    response: { }
}
