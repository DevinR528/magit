use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::{rest::ApplicationV3Json, IncomingPullRequest};

github_rest_api! {
    metadata: {
        description: "Fetch metadata about a pull request",
        method: GET,
        path: "/repos/:owner/:repo/pulls/:pull_number",
        name: "get_pull_request",
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

    response: {
        /// Information about the given pull request.
        #[serde(flatten)]
        pub repository: IncomingPullRequest,
    }
}

#[test]
fn get_pull_request() {
    let json = include_str!("../../../../test_json/rest/get_pull_request.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let _repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
