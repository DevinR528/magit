use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::{rest::ApplicationV3Json, IncomingIssue};

github_rest_api! {
    metadata: {
        description: "Add assignees to the given issue",
        method: POST,
        path: "/repos/:owner/:repo/issues/:issue_number/assignees",
        name: "assign_issue",
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

        /// Usernames of the people to assign to this issue.
        ///
        /// Limited to 10 users.
        // TODO: ^^^^
        #[github(body)]
        pub assignees: Vec<&'a str>,
    }

    response: {
        /// The comment that was just created.
        #[serde(flatten)]
        pub repository: IncomingIssue,
    }
}

#[test]
fn create_comment() {
    let json = include_str!("../../../../test_json/rest/assign_issue.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let _repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
