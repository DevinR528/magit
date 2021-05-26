use std::borrow::Cow;

use github_derive::github_rest_api;
use reqwest::{header::HeaderMap, Method};
use ruma::serde::StringEnum;
use serde::Serialize;

use crate::api::{
    rest::{ApplicationV3Json, Type},
    IncomingRepo,
};

github_rest_api! {
    metadata: {
        description: "",
        method: GET,
        path: "/repos/:owner/:repo",
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

        #[github(query)]
        pub r#type: Type,
    }

    response: {
        #[serde(flatten)]
        pub repository: IncomingRepo,
    }
}

#[test]
fn get_repository() {
    let json = include_str!("../../../../test_json/rest/get_repo.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
