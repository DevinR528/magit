use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::{rest::ApplicationV3Json, IncomingComment};

github_rest_api! {
    metadata: {
        description: "Create a comment in the given issue",
        method: POST,
        path: "/repos/:owner/:repo/issues/:issue_number/comments",
        name: "create_issue_comment",
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

        /// The title of the new issue.
        #[github(path)]
        pub issue_number: UInt,

        /// Body of the new comment.
        #[github(body)]
        pub body: &'a str,
    }

    response: {
        /// The comment that was just created.
        #[serde(flatten)]
        pub repository: IncomingComment,
    }
}

#[test]
fn create_comment() {
    let json = include_str!("../../../../test_json/rest/create_comment.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let _repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
