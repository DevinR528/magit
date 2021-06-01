use github_derive::github_rest_api;
use ruma::UInt;

use crate::api::{
    rest::{ApplicationV3Json, Direction, MilestoneQuery, Sort, StateQuery, Type},
    Dt, IncomingPullRequest,
};

github_rest_api! {
    metadata: {
        description: "List all the pull requests for a repository",
        method: GET,
        path: "/repos/:owner/:repo/pulls",
        name: "list_pull_requests",
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

        /// Filter by state, see [`crate::api::rest::StateQuery`] for more information.
        ///
        /// Defaults to `StateQuery::Open`.
        #[github(query)]
        #[serde(serialize_with = "crate::api::rest::opt_default")]
        pub state: Option<StateQuery>,

        /// Filter by the name of the head.
        ///
        /// The format of this query is `user:ref-name` or `organization:ref-name`.
        #[github(query)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub head: Option<&'a str>,

        /// Filter by the name of the base branch.
        #[github(query)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base: Option<&'a str>,

        /// The order the repositories are returned.
        ///
        /// Defaults to `Sort::Created`.
        #[github(query)]
        #[serde(serialize_with = "crate::api::rest::sort")]
        pub sort: Option<Sort>,

        /// The direction repositories are return, ascending or descending.
        ///
        /// Defaults to `Direction::Desc`.
        #[github(query)]
        #[serde(serialize_with = "crate::api::rest::direction")]
        pub direction: Option<Direction>,

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

    #[github(with = pull_requests)]
    response: {
        /// A list of pull requests for the repository.
        pub pull_requests: Vec<IncomingPullRequest>,
    }
}

pub fn pull_requests<'de, D>(deser: D) -> Result<Response, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Response { pull_requests: serde::Deserialize::deserialize(deser)? })
}

#[test]
fn list_pull_requests() {
    let json = include_str!("../../../../test_json/rest/list_pull_requests.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
