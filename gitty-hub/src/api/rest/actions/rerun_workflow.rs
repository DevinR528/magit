use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::rest::ApplicationV3Json;

github_rest_api! {
    metadata: {
        description: "Re-run a job in the workflow",
        method: POST,
        path: "/repos/:owner/:repo/actions/runs/:run_id/rerun",
        name: "rerun_job",
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

        /// The identifier for this run.
        #[github(path)]
        pub run_id: UInt,
    }

    /// Empty response with status 201 to indicate the workflow is being rerun.
    response: {}
}
