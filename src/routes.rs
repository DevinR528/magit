use std::fmt::Debug;

use rocket::{
    http::Status,
    post,
    response::{Responder, Result as RocketResult},
    Request, State,
};
use thiserror::Error;
use tracing::{debug, info, warn};

use crate::{
    api::webhooks::{
        issue::IssueEvent,
        pull::{PullRequestAction, PullRequestEvent},
        push::PushEvent,
        GitHubEvent,
    },
    str_fmt, Store,
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
    if !store.config.github.events.contains(&event.as_kind()) {
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
        GitHubEvent::Star(star) => {
            store.to_matrix.send(star.sender.login.to_string()).await?
        }
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
    let additions = pull.pull_request.additions.to_string();
    let additions = &additions;
    let deletions = pull.pull_request.deletions.to_string();
    let deletions = &deletions;
    let changed_files = pull.pull_request.changed_files.to_string();
    let changed_files = &changed_files;
    let commits = pull.pull_request.commits.to_string();
    let commits = &commits;
    let mergable = pull.pull_request.mergeable.unwrap_or_default().to_string();
    let mergable = &mergable;
    let rebaseable = pull.pull_request.rebaseable.unwrap_or_default().to_string();
    let rebaseable = &rebaseable;
    let pull_url = pull.pull_request.html_url.to_string();
    let pull_url = &pull_url;

    let action = match pull.action {
        PullRequestAction::Assigned => {
            let assignee = pull
                .pull_request
                .assignee
                .map(|a| a.login.to_string())
                .unwrap_or_else(|| "<unknown>".to_owned());
            format!("the PR was assigned to {}", assignee)
        }
        PullRequestAction::AutoMergeDisabled => {
            "this PR's auto merge was disabled".to_owned()
        }
        PullRequestAction::AutoMergeEnabled => {
            "this PR's auto merge was enabled".to_owned()
        }
        PullRequestAction::Closed => {
            if pull.pull_request.merged.unwrap_or_default() {
                "the PR was merged".to_owned()
            } else {
                "the PR was closed without merging".to_owned()
            }
        }
        PullRequestAction::ConvertToDraft => "the PR was converted to a draft".to_owned(),
        PullRequestAction::Edited => "the PR has been edited".to_owned(),
        PullRequestAction::Labeled => "the PR has been labeled".to_owned(),
        PullRequestAction::Locked => "the PR has been locked".to_owned(),
        PullRequestAction::Opened => "the PR has been opened".to_owned(),
        PullRequestAction::ReadyForReview => "the PR is ready for review".to_owned(),
        PullRequestAction::Reopened => "the PR has been reopened".to_owned(),
        PullRequestAction::ReviewRequestedRemoved => {
            "requested review for this PR has been removed".to_owned()
        }
        PullRequestAction::ReviewRequested => "review has been requested".to_owned(),
        PullRequestAction::Synchronize => "the PR was synchronized".to_owned(),
        PullRequestAction::Unassigned => "the PR was unassigned".to_owned(),
        PullRequestAction::Unlabeled => "the PR was unlabeled".to_owned(),
        PullRequestAction::Unlocked => "the PR was unlocked".to_owned(),
        _ => "<unknown>".to_owned(),
    };
    let action = &action;

    if let Some(fmt_str) = store.config.github.format_strings.get("pull_request") {
        store
            .to_matrix
            .send(str_fmt!(
                fmt_str, repo_name, username, action, pull_url, current, base, commits,
                additions, deletions, mergable, rebaseable,
            ))
            .await
            .map_err(|e| e.into())
    } else {
        store
            .to_matrix
            .send(format!(
                r#"[{}] {}'s PR has new activity: {}
[Check out the pull request!]({})
{} was opened against {}, has {} commits
++ {}
-- {}
{} changed files
is able to merge {}, can be rebased {}"#,
                repo_name,
                username,
                action,
                pull_url,
                current,
                base,
                commits,
                additions,
                deletions,
                changed_files,
                mergable,
                rebaseable,
            ))
            .await
            .map_err(|e| e.into())
    }
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
