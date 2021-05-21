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
pub struct CheckRunEvent<'a> {
    /// The action that was performed.
    pub action: CheckAction,

    /// The checks to run.
    #[serde(borrow)]
    pub check_run: CheckRun<'a>,

    /// Information about the repositories this app has access to.
    #[serde(borrow)]
    pub repository: Repo<'a>,

    /// Detailed information about the organization the app
    /// belongs to.
    pub organization: Option<Org<'a>>,

    /// Information about Github app installation.
    ///
    /// This is only present if the event is sent from said app.
    pub installation: Option<Installation<'a>>,

    /// Detailed information about the user of the app.
    #[serde(borrow)]
    pub sender: User<'a>,
}

/// Information about a suite of checks.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckRun<'a> {
    /// Numeric Id of this installation.
    pub id: UInt,

    /// Numeric identifier of the repository.
    pub node_id: &'a str,

    /// The SHA of the head branch.
    pub head_sha: &'a str,

    /// The api url of the pull request.
    pub url: &'a str,

    /// A reference for the run on the integrators system.
    pub external_id: &'a str,

    /// The api url of the pull request.
    pub details_url: &'a str,

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
    #[serde(borrow)]
    pub pull_requests: Vec<CheckPullRequest<'a>>,

    /// The app that generated this check.
    #[serde(borrow)]
    pub app: App<'a>,

    /// The suite of checks.
    #[serde(borrow)]
    pub check_suite: CheckSuite<'a>,

    /// Output from the checks running.
    #[serde(borrow)]
    pub output: Output<'a>,

    /// Information about the deployment.
    #[serde(borrow)]
    pub deployment: Deployment<'a>,
}

/// The output from a check.
#[derive(Clone, Debug, Deserialize)]
pub struct Output<'a> {
    /// Title of this check run.
    pub title: Option<&'a str>,

    /// Summary of the check.
    pub summary: Option<&'a str>,

    /// The api url of the pull request.
    pub text: Option<&'a str>,

    /// Number of annotations on this check.
    pub annotations_count: UInt,

    /// The api url of the annotations.
    pub annotations_url: Url,
}

/// Information about the deployment of this check.
#[derive(Clone, Debug, Deserialize)]
pub struct Deployment<'a> {
    /// The api url of the check run.
    pub url: Url,

    /// Numeric Id of this deployment.
    pub id: UInt,

    /// Numeric identifier of the deployment.
    pub node_id: &'a str,

    /// The task being run.
    pub task: &'a str,

    /// The current environment for this check.
    pub environment: &'a str,

    /// The original environment for this check.
    pub original_environment: &'a str,

    /// The api url of the check run.
    pub description: Option<&'a str>,

    /// The time in UTC when the check run was created.
    pub created_at: Dt,

    /// The time in UTC when the check run was updated.
    pub updated_at: Dt,
}
