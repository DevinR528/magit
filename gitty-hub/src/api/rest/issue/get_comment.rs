use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::{rest::ApplicationV3Json, IncomingComment};

github_rest_api! {
    metadata: {
        description: "Fetch a comment from an issue",
        method: GET,
        path: "/repos/:owner/:repo/issues/comments/:comment_id",
        name: "get_comment",
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

        #[github(path)]
        pub comment_id: UInt,
    }

    response: {
        /// The comment that was requested.
        #[serde(flatten)]
        pub repository: IncomingComment,
    }
}

#[test]
fn get_comment() {
    let json = include_str!("../../../../test_json/rest/get_comment.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let _repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
