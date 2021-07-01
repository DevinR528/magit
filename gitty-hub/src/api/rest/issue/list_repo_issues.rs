use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::{
    rest::{ApplicationV3Json, Direction, MilestoneQuery, Sort, StateQuery},
    Dt, IncomingIssue,
};

github_rest_api! {
    metadata: {
        description: "List all the issues of a repository",
        method: GET,
        path: "/repos/:owner/:repo/issues",
        name: "list_issues",
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

        /// Filter by milestone, see [`crate::api::rest::MilestoneQuery`] for more information.
        #[github(query)]
        pub milestone: MilestoneQuery,

        /// Filter by state, see [`crate::api::rest::StateQuery`] for more information.
        ///
        /// Defaults to `StateQuery::Open`.
        #[github(query)]
        #[serde(serialize_with = "crate::api::rest::opt_default")]
        pub state: Option<StateQuery>,

        /// Filter by the user that created the issue.
        #[github(query)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub creator: Option<&'a str>,

        /// Filter by a user that is mentioned in the issue.
        #[github(query)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mentioned: Option<&'a str>,

        /// Filter by labels.
        #[github(query)]
        #[serde(
            skip_serializing_if = "<[_]>::is_empty",
            serialize_with = "crate::api::rest::comma_list"
        )]
        pub labels: Vec<&'a str>,

        /// The user that created the issue.
        #[github(query)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub since: Option<Dt>,

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

    #[github(with = issues)]
    response: {
        /// A list of issues that follow the filter from the request.
        pub issues: Vec<IncomingIssue>,
    }
}

pub fn issues<'de, D>(deser: D) -> Result<Response, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Response { issues: serde::Deserialize::deserialize(deser)? })
}

#[test]
fn list_issues() {
    let json = include_str!("../../../../test_json/rest/list_issues.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let _repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
