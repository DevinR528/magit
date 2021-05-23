use rocket::{
    http::Status,
    post,
    response::{Responder, Result as RocketResult},
    Request, State,
};
use thiserror::Error;
use tracing::{debug, info, warn};

use crate::{
    api::{
        issue::IssueEvent,
        pull::{PullRequestAction, PullRequestEvent},
        push::PushEvent,
        GitHubEvent,
    },
    Store,
};

pub type ResponseResult<T> = Result<T, ResponseError>;

#[derive(Debug, Error)]
pub enum ResponseError {
    /// JSON deserialization failed.
    #[error("JSON deserialization failed: {0}")]
    Json(#[from] serde_json::Error),

    /// Cannot send messages to Matrix receiver.
    #[error("Cannot send messages to Matrix receiver: {0}")]
    Send(#[from] tokio::sync::mpsc::error::SendError<String>),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ResponseError {
    fn respond_to(self, _: &'r Request<'_>) -> RocketResult<'o> {
        Err(match self {
            ResponseError::Json(err) => {
                warn!("{}", err);
                Status::InternalServerError
            }
            ResponseError::Send(err) => {
                warn!("to matrix error: {}", err);
                Status::InternalServerError
            }
        })
    }
}

#[post("/", data = "<event>")]
pub async fn index<'o: 'r, 'r>(
    event: GitHubEvent<'r>,
    to_matrix: &State<Store>,
) -> ResponseResult<Status> {
    let store: &Store = to_matrix;
    if !store.config.github.events.contains(&event.as_kind().to_string()) {
        return Ok(Status::NoContent);
    }
    match event {
        GitHubEvent::CheckRun(_) => {}
        GitHubEvent::CheckSuite(_) => {}
        GitHubEvent::CommitComment(_) => {}
        GitHubEvent::Create(_) => {}
        GitHubEvent::Delete(_) => {}
        GitHubEvent::Installation(_) => {}
        GitHubEvent::Issue(issue) => handle_issue(issue, store).await?,
        GitHubEvent::IssueComment(_) => {}
        GitHubEvent::Milestone(_) => {}
        GitHubEvent::Ping(ping) => {
            if let Some(zen) = ping["zen"].as_str() {
                store.to_matrix.send(zen.to_string()).await?;
            }
        }
        GitHubEvent::PullRequest(pull) => handle_pull_request(pull, store).await?,
        GitHubEvent::PullRequestReview(_) => {}
        GitHubEvent::PullRequestReviewComment(_) => {}
        GitHubEvent::Push(push) => handle_push(push, store).await?,
        GitHubEvent::Release(_) => {}
        GitHubEvent::Star(_) => {}
        GitHubEvent::Status(_) => {}
        GitHubEvent::Watch(_) => {}
    };

    Ok(Status::Ok)
}

async fn handle_issue(issue: IssueEvent<'_>, store: &Store) -> ResponseResult<()> {
    let username = issue.issue.user.login;
    let repo_name = issue.repository.full_name;

    store
        .to_matrix
        .send(format!(
            "[{}] {} pushed {} commit{} to {}.\n[Check out the diff!]({})",
            repo_name, username, "", "", "", ""
        ))
        .await
        .map_err(|e| e.into())
}

async fn handle_pull_request(
    pull: PullRequestEvent<'_>,
    store: &Store,
) -> ResponseResult<()> {
    let repo_name = pull.repository.full_name;
    let username = pull.pull_request.user.login;
    let current = pull.pull_request.head.ref_;
    let base = pull.pull_request.base.ref_;
    let additions = pull.pull_request.additions;
    let deletions = pull.pull_request.deletions;
    let commits = pull.pull_request.commits;
    let pull_url = pull.pull_request.html_url;

    let action = match pull.action {
        PullRequestAction::Assigned => {
            let assignee = pull
                .pull_request
                .assignee
                .map(|a| a.login.to_string())
                .unwrap_or_else(|| "<unknown>".to_owned());
            format!("'s PR was assigned to {}", assignee)
        }
        PullRequestAction::AutoMergeDisabled => {
            format!("'s PR was AutoMergeDisabled")
        }
        PullRequestAction::AutoMergeEnabled => {
            format!("'s PR was AutoMergeEnabled")
        }
        PullRequestAction::Closed => {
            if pull.pull_request.merged.unwrap_or_default() {
                format!("'s PR was merged!!")
            } else {
                format!("'s PR was closed without merging :(")
            }
        }
        PullRequestAction::ConvertToDraft => {
            format!("'s PR was converted to a draft")
        }
        PullRequestAction::Edited => format!("'s PR was Edited"),
        PullRequestAction::Labeled => format!("'s PR was Labeled"),
        PullRequestAction::Locked => format!("'s PR was Locked"),
        PullRequestAction::Opened => format!("'s PR was Opened"),
        PullRequestAction::ReadyForReview => format!("'s PR was Unassigned"),
        PullRequestAction::Reopened => format!("'s PR was Reopened"),
        PullRequestAction::ReviewRequestedRemoved => {
            format!("'s PR was ReviewRequestedRemoved")
        }
        PullRequestAction::ReviewRequested => {
            format!("'s PR was ReviewRequested")
        }
        PullRequestAction::Synchronize => format!("'s PR was Unassigned"),
        PullRequestAction::Unassigned => format!("'s PR was Unassigned"),
        PullRequestAction::Unlabeled => format!("'s PR was Unlabeled"),
        PullRequestAction::Unlocked => format!("'s PR was Unlocked"),
    };

    store
        .to_matrix
        .send(format!(
            "[{}] {}{}\n[Check out the pull request!]({})",
            repo_name,
            username,
            action,
            pull_url.as_str()
        ))
        .await
        .map_err(|e| e.into())
}

async fn handle_push(push: PushEvent<'_>, store: &Store) -> ResponseResult<()> {
    let username = push.pusher.username.map(|s| s.to_owned()).unwrap_or_default();
    let repo_name = push.repository.full_name;
    let commits_url = push.compare;
    let commits_count = push.commits.len();
    let branch = push.ref_.split('/').last().map(|s| s.to_string()).unwrap_or_default();

    store
        .to_matrix
        .send(format!(
            "[{}] {} pushed {} commit{} to {}.\n[Check out the diff!]({})",
            repo_name,
            username,
            commits_count,
            if commits_count > 1 { "s" } else { "" },
            branch,
            commits_url
        ))
        .await
        .map_err(|e| e.into())
}
