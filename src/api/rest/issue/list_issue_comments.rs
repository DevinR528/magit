use github_derive::github_rest_api;
use ruma::UInt;

use crate::api::{rest::ApplicationV3Json, Dt, IncomingComment};

github_rest_api! {
    metadata: {
        description: "List all the comments of an issue",
        method: GET,
        path: "/repos/:owner/:repo/issues/:issue_number/comments",
        name: "list_comments",
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

        /// The issue number of the issue to fetch.
        #[github(path)]
        pub issue_number: UInt,

        /// The user that created the issue.
        #[github(query)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub since: Option<Dt>,

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

    #[github(with = ::magit::api::rest::issue::list_issue_comments::comments)]
    response: {
        /// A list of issues that follow the filter from the request.
        pub comments: Vec<IncomingComment>,
    }
}

pub fn comments<'de, D>(deser: D) -> Result<Response, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Response { comments: serde::Deserialize::deserialize(deser)? })
}

#[test]
fn list_issues() {
    let json = include_str!("../../../../test_json/rest/list_comments.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
