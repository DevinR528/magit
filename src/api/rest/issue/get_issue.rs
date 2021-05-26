use std::borrow::Cow;

use github_derive::github_rest_api;
use reqwest::{header::HeaderMap, Method};
use ruma::{serde::StringEnum, UInt};
use serde::Serialize;

use crate::api::{rest::ApplicationV3Json, IncomingIssue};

github_rest_api! {
    metadata: {
        description: "",
        method: GET,
        path: "/repos/:owner/:repo/issues/:issue_number",
        name: "get_repository",
        authentication: true,
    }

    request: {
        #[github(header = ACCEPT)]
        pub accept: Option<ApplicationV3Json>,

        #[github(path)]
        pub owner: &'a str,

        #[github(path)]
        pub repo: &'a str,

        #[github(path)]
        pub issue_number: UInt,
    }

    response: {
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
