use std::borrow::Cow;

use github_derive::github_rest_api;
use reqwest::{header::HeaderMap, Method};
use ruma::{serde::StringEnum, uint, UInt};
use serde::Serialize;

use crate::api::{
    rest::{ApplicationV3Json, Direction, MilestoneQuery, Sort, StateQuery, Type},
    IncomingIssue,
};

github_rest_api! {
    metadata: {
        description: "",
        method: GET,
        path: "/repos/:owner/:repo/issues",
        name: "list_org_repositories",
        authentication: true,
    }

    request: {
        #[github(header = ACCEPT)]
        pub accept: Option<ApplicationV3Json>,

        /// The name of the owner.
        #[github(path)]
        pub owner: &'a str,

        /// Filter the returned repositories by type.
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
        /// Defaults to 1.
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

    #[github(with = "::magit::api::rest::issue::list_repo_issues::issues")]
    response: {
        pub issues: Vec<IncomingIssue>,
    }
}

pub(crate) fn issues<'de, D>(deser: D) -> Result<Response, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::Deserialize;
    let issues = Vec::<IncomingIssue>::deserialize(deser)?;
    Ok(Response { issues })
}

#[test]
fn list_issues() {
    let json = include_str!("../../../../test_json/rest/list_issues.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
