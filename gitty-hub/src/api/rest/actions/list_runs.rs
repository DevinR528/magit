use github_derive::github_rest_api;
use js_int::UInt;

use crate::api::{rest::ApplicationV3Json, IncomingWorkflowRun, WorkflowEvent};

github_rest_api! {
    metadata: {
        description: "List all workflow runs for a workflow",
        method: GET,
        path: "/repos/:owner/:repo/actions/workflows/:workflow_id/runs",
        name: "list_jobs",
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

        /// the identifier of the workflow.
        ///
        /// This can either be a numeric id or the file name of this workflow
        /// from your `.github` folder ie `main.yaml`.
        #[github(path)]
        pub workflow_id: &'a str,

        /// Returns someone's workflow runs.
        ///
        /// Use the login for the user who created the push associated with
        // the check suite or workflow run.
        #[github(query)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub actor: Option<&'a str>,

        /// Returns workflows associated with this branch.
        #[github(query)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub branch: Option<&'a str>,

        /// Returns workflow run triggered by the event you specify.
        #[github(query)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub event: Option<WorkflowEvent>,

        /// Result per page.
        ///
        /// Defaults to 30 and max 100.
        #[github(query)]
        #[serde(serialize_with = "crate::api::rest::per_page")]
        pub per_page: Option<UInt>,

        /// Which page of the results to return.
        ///
        /// Defaults to 1.
        #[github(query)]
        #[serde(serialize_with = "crate::api::rest::page")]
        pub page: Option<UInt>,
    }

    response: {
        /// The number of runs found.
        pub total_count: UInt,

        /// The runs for a workflow.
        pub workflow_runs: Vec<IncomingWorkflowRun>,
    }
}

#[test]
fn list_runs() {
    let json = include_str!("../../../../test_json/rest/list_runs.json");

    let jd = &mut serde_json::Deserializer::from_str(json);
    let _repo = serde_path_to_error::deserialize::<_, Response>(jd).unwrap();
}
