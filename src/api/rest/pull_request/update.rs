use github_derive::{github_rest_api, Incoming, StringEnum};
use ruma::UInt;
use serde::Deserialize;

use crate::api::rest::ApplicationLydian;

github_rest_api! {
    metadata: {
        description: "Update a pull request",
        method: PUT,
        path: "/repos/:owner/:repo/pulls/:pull_number/update-branch",
        name: "update_pull_request",
        authentication: true,
    }

    request: {
        /// Required accept header to enable this feature.
        #[github(header = ACCEPT)]
        pub accept: ApplicationLydian,

        /// The owner of this repository.
        #[github(path)]
        pub owner: &'a str,

        /// The name of this repository.
        #[github(path)]
        pub repo: &'a str,

        /// The pull request number.
        #[github(path)]
        pub pull_number: UInt,

        /// The expected SHA of the pull request's HEAD ref.
        ///
        /// This is the most recent commit on the pull request's branch.
        /// If the expected SHA does not match the pull request's HEAD, you will
        /// receive a 422 Unprocessable Entity status. Defaults to the SHA of the
        /// pull request's current HEAD ref.
        #[github(body)]
        pub expected_head_sha: Option<&'a str>,
    }

    response: {
        /// Information about the merge.
        #[serde(flatten)]
        pub repository: IncomingUpdateMerge,
    }
}

#[derive(Clone, Debug, Deserialize, Incoming)]
pub struct UpdateMerge<'a> {
    /// The commit message.
    pub message: &'a str,

    /// The public url to the pull request.
    pub url: &'a str,
}

#[test]
fn update_pull_request() {
    let json = include_str!("../../../../test_json/rest/update_pull_request.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
