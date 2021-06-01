use github_derive::github_rest_api;
use ruma::UInt;

use crate::api::{
    rest::{ApplicationV3Json, Type},
    IncomingComment,
};

github_rest_api! {
    metadata: {
        description: "Update a comment by comment Id",
        method: PATCH,
        path: "/repos/:owner/:repo/issues/comments/:comment_id",
        name: "update_issue_comment",
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
        pub comment_id: UInt,

        /// Body of the new issue.
        #[github(body)]
        pub body: &'a str,
    }

    response: {
        /// The comment that was just updated.
        #[serde(flatten)]
        pub comment: IncomingComment,
    }
}

#[test]
fn update_comment() {
    let json = include_str!("../../../../test_json/rest/update_comment.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
