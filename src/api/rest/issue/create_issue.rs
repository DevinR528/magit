use std::borrow::Cow;

use github_derive::github_rest_api;
use reqwest::{header::HeaderMap, Method};
use ruma::{serde::StringEnum, UInt};
use serde::Serialize;

use crate::api::{rest::ApplicationV3Json, IncomingIssue};

github_rest_api! {
    metadata: {
        description: "",
        method: POST,
        path: "/repos/:owner/:repo/issues",
        name: "create_repository",
        authentication: true,
    }

    request: {
        #[github(header = ACCEPT)]
        pub accept: Option<ApplicationV3Json>,

        /// The owner of this repository.
        #[github(path)]
        pub owner: &'a str,

        /// The name of this repository.
        #[github(path)]
        pub repo: &'a str,

        /// The title of the new issue.
        #[github(body)]
        pub title: &'a str,

        /// Optional body of the new issue.
        #[github(body)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub body: Option<&'a str>,

        /// Associated milestones, by number.
        ///
        /// NOTE: Only users with push access can set the milestone for new issues.
        #[github(body)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub milestone: Option<UInt>,

        /// An optional list of labels.
        ///
        /// NOTE: Only users with push access can set the labels for new issues.
        #[github(body)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub labels: Vec<&'a str>,

        /// An optional list of assignees.
        ///
        /// NOTE: Only users with push access can set the assignees for new issues.
        #[github(body)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub assignees: Vec<&'a str>,
    }

    response: {
        #[serde(flatten)]
        pub issue: IncomingIssue,
    }
}

#[test]
fn get_issue() {
    let json = include_str!("../../../../test_json/rest/get_issue.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
