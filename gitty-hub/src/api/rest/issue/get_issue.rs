use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::{rest::ApplicationV3Json, IncomingIssue};

github_rest_api! {
    metadata: {
        description: "Get information about an issue",
        method: GET,
        path: "/repos/:owner/:repo/issues/:issue_number",
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

        /// The issue number of the issue to fetch.
        #[github(path)]
        pub issue_number: UInt,
    }

    response: {
        /// The requested issue.
        #[serde(flatten)]
        pub repository: IncomingIssue,
    }
}

#[test]
fn get_issue() {
    let json = include_str!("../../../../test_json/rest/get_issue.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
