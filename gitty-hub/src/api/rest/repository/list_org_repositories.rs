use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::{
    rest::{ApplicationV3Json, Direction, Sort, Type},
    IncomingRepository,
};

github_rest_api! {
    metadata: {
        description: "",
        method: GET,
        path: "/orgs/:org/repos",
        name: "list_org_repositories",
        authentication: true,
    }

    request: {
        /// Optional accept header to enable preview features.
        #[github(header = ACCEPT)]
        pub accept: Option<ApplicationV3Json>,

        /// The name of the organization.
        #[github(path)]
        pub org: &'a str,

        /// Filter the returned repositories by type.
        #[github(query)]
        pub r#type: Type,

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

    #[github(with = repositories)]
    response: {
        pub repositories: Vec<IncomingRepository>,
    }
}

pub(crate) fn repositories<'de, D>(deser: D) -> Result<Response, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Response { repositories: serde::Deserialize::deserialize(deser)? })
}

#[test]
fn list_repositories() {
    let json = include_str!("../../../../test_json/rest/list_org_repos.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let _repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
