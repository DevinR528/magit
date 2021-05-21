use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

use crate::api::{
    check_suite::{
        CheckAction, CheckPullRequest, CheckStatus, CheckSuite, ConclusionStatus,
    },
    common::{default_null, App, Dt, Org, Repo, User},
    installation::Installation,
};

/// The payload of a check run event.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckRunEvent {
    /// The action that was performed.
    pub action: CheckAction,

    /// The checks to run.
    pub check_run: CheckRun,

    /// Information about the repositories this app has access to.
    pub repository: Repo,

    /// Detailed information about the organization the app
    /// belongs to.
    pub organization: Option<Org>,

    /// Information about Github app installation.
    ///
    /// This is only present if the event is sent from said app.
    pub installation: Option<Installation>,

    /// Detailed information about the user of the app.
    pub sender: User,
}

/// Information about a suite of checks.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckRun {
    /// Numeric Id of this installation.
    pub id: UInt,

    /// Numeric identifier of the repository.
    pub node_id: String,

    /// The SHA of the head branch.
    pub head_sha: String,

    /// The api url of the pull request.
    pub url: String,

    /// A reference for the run on the integrators system.
    pub external_id: String,

    /// The api url of the pull request.
    pub details_url: String,

    /// The status of this check.
    #[serde(default, deserialize_with = "default_null")]
    pub status: CheckStatus,

    /// If this is not none then the check has finished with a status.
    #[serde(default, deserialize_with = "default_null")]
    pub conclusion: ConclusionStatus,

    /// The time in UTC when the check run was started.
    pub started_at: Dt,

    /// The time in UTC when the check run was completed.
    pub completed_at: Option<Dt>,

    /// The pull request being checked.
    pub pull_requests: Vec<CheckPullRequest>,

    /// The app that generated this check.
    pub app: App,

    /// The suite of checks.
    pub check_suite: CheckSuite,

    /// Output from the checks running.
    pub output: Output,

    /// Information about the deployment.
    pub deployment: Deployment,
}

/// The output from a check.
#[derive(Clone, Debug, Deserialize)]
pub struct Output {
    /// Title of this check run.
    pub title: Option<String>,

    /// Summary of the check.
    pub summary: Option<String>,

    /// The api url of the pull request.
    pub text: Option<String>,

    /// Number of annotations on this check.
    pub annotations_count: UInt,

    /// The api url of the annotations.
    pub annotations_url: Url,
}

/// Information about the deployment of this check.
#[derive(Clone, Debug, Deserialize)]
pub struct Deployment {
    /// The api url of the check run.
    pub url: Url,

    /// Numeric Id of this deployment.
    pub id: UInt,

    /// Numeric identifier of the deployment.
    pub node_id: String,

    /// The task being run.
    pub task: String,

    /// The current environment for this check.
    pub environment: String,

    /// The original environment for this check.
    pub original_environment: String,

    /// The api url of the check run.
    pub description: Option<String>,

    /// The time in UTC when the check run was created.
    pub created_at: Dt,

    /// The time in UTC when the check run was updated.
    pub updated_at: Dt,
}
