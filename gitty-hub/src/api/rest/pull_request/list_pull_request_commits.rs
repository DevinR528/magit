use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::{rest::ApplicationV3Json, IncomingCommit};

github_rest_api! {
    metadata: {
        description: "List all the commits of this pull request",
        method: GET,
        path: "/repos/:owner/:repo/pulls/:pull_number/commits",
        name: "list_pull_requests_commits",
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

    #[github(with = pr_commits)]
    response: {
        /// A list of all the commits of a pull request.
        pub pr_commits: Vec<IncomingCommit>,
    }
}

pub fn pr_commits<'de, D>(deser: D) -> Result<Response, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Response { pr_commits: serde::Deserialize::deserialize(deser)? })
}

#[test]
fn list_pull_requests_commits() {
    let json = include_str!("../../../../test_json/rest/list_pull_request_commits.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
