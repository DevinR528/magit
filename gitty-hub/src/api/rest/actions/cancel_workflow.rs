use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::rest::ApplicationV3Json;

github_rest_api! {
    metadata: {
        description: "Download the logs for a job",
        method: GET,
        path: "/repos/:owner/:repo/actions/jobs/:job_id/logs",
        name: "download_jobs_log",
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
        pub job_id: UInt,
    }

    #[github(forward_to_body = logs)]
    response: {
        /// The log from the jobs.
        pub logs: String,
    }
}
