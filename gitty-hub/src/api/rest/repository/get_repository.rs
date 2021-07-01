use github_derive::github_rest_api;

use crate::api::{
    rest::{ApplicationV3Json, Type},
    IncomingRepository,
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
        /// Optional accept header to enable preview features.
        #[github(header = ACCEPT)]
        pub accept: Option<ApplicationV3Json>,

        /// The owner of this repository.
        #[github(path)]
        pub owner: &'a str,

        /// The name of this repository.
        #[github(path)]
        pub repo: &'a str,

        /// The type of repository to return.
        ///
        /// See [`crate::api::rest::Type`] for variants.
        #[github(query)]
        pub r#type: Type,
    }

    response: {
        #[serde(flatten)]
        pub repository: IncomingRepository,
    }
}

#[test]
fn get_repository() {
    let json = include_str!("../../../../test_json/rest/get_repository.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let _repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
