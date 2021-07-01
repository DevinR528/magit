use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::rest::ApplicationV3Json;

github_rest_api! {
    metadata: {
        description: "Download the logs for a run",
        method: GET,
        path: "/repos/:owner/:repo/actions/runs/:run_id/logs",
        name: "download_runs_log",
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
        pub run_id: UInt,
    }

    #[github(forward_to_body = logs)]
    response: {
        /// The log from the run.
        pub logs: String,
    }
}
