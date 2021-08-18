use std::fmt::Debug;

use rocket::{
    http::Status,
    post,
    response::{Responder, Result as RocketResult},
    Request, State,
};
use ruma::RoomId;
use thiserror::Error;
use tracing::{debug, warn};

use crate::{
    api::{
        webhooks::{
            issue::IssueEvent,
            pull::{PullRequestAction, PullRequestEvent},
            push::PushEvent,
            GitHubEvent,
        },
        IssueState,
    },
    from_data::GithubHookEvent,
    str_fmt, RepoRoomMap, Store,
};

#[rocket::catch(404)]
pub fn not_found(r: &Request<'_>) -> String {
    warn!("{:?}", r.uri());
    "not found".to_string()
}

pub type ResponseResult<T> = Result<T, ResponseError>;

#[derive(Debug, Error)]
pub enum ResponseError {
    /// JSON deserialization failed.
    #[error("JSON deserialization failed: {0}")]
    Json(#[from] serde_json::Error),

    /// Cannot send messages to Matrix receiver.
    #[error("Cannot send messages to Matrix receiver: {0}")]
    Send(#[from] tokio::sync::mpsc::error::SendError<(RoomId, String)>),
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
pub async fn index(
    event: GithubHookEvent<'_>,
    to_matrix: &State<Store>,
) -> ResponseResult<Status> {
    let store: &Store = to_matrix;
    if !store.config.github.events.contains(&event.0.as_kind()) {
        debug!("found {:?}, not one of our hooks", event.0.as_kind());
        return Ok(Status::NoContent);
    }
    match event.0 {
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
                for RepoRoomMap { room, .. } in &store.config.github.repos {
                    store.to_matrix.send((room.clone(), zen.to_string())).await?;
                }
            }
        }
        GitHubEvent::PullRequest(pull) => handle_pull_request(pull, store).await?,
        GitHubEvent::PullRequestReview(_) => {}
        GitHubEvent::PullRequestReviewComment(_) => {}
        GitHubEvent::Push(push) => handle_push(push, store).await?,
        GitHubEvent::Release(_) => {}
        GitHubEvent::Star(star) => {
            if let Some(room) = store
                .config
                .github
                .repos
                .iter()
                .find(|map| map.repo == star.repository.full_name)
                .map(|r| r.room.clone())
            {
                store.to_matrix.send((room, star.sender.login.to_string())).await?;
            }
        }
        GitHubEvent::Status(_) => {}
        GitHubEvent::Watch(_) => {}
    };

    Ok(Status::Ok)
}

async fn handle_issue(issue: IssueEvent<'_>, store: &Store) -> ResponseResult<()> {
    let repo_name;
    let username;
    let issue_number;
    let linked_pr;
    let issue_url;
    let title;
    let body;
    let state;
    ready_to_fmt! {
        repo_name = issue.repository.full_name;
        username = issue.issue.user.login;
        ref issue_number = issue.issue.number.to_string();
        ref linked_pr = issue.issue.pull_request
            .map(|pr| format!("a [linked pull request]({})", pr.html_url))
            .unwrap_or_else(|| "no linked pull request".to_owned());
        ref issue_url = issue.issue.html_url.to_string();
        title = issue.issue.title;
        body = issue.issue.body;
        state = match issue.issue.state {
            IssueState::Open => "opened",
            IssueState::Closed => "closed",
            IssueState::Unknown => "<unknown>",
             _ => "<new unknown variant>",
        };
    }

    let fmt_str = store.config.github.format_strings.get("issues");
    let room = store
        .config
        .github
        .repos
        .iter()
        .find(|map| map.repo == repo_name)
        .map(|r| r.room.clone());
    match (fmt_str, room) {
        (Some(fmt_str), Some(room)) => {
            store
                .to_matrix
                .send((
                    room,
                    str_fmt!(
                        fmt_str,
                        repo_name,
                        username,
                        issue_number,
                        linked_pr,
                        issue_url,
                        title,
                        body,
                        state,
                    ),
                ))
                .await?;
        }
        (None, Some(room)) => {
            store
                .to_matrix
                .send((
                    room,
                    format!(
                        r#"[{}] {} {} issue #{}
[Check out the issue!]({})
{}
{}
This PR has {}"#,
                        repo_name,
                        username,
                        state,
                        issue_number,
                        issue_url,
                        title,
                        body,
                        linked_pr,
                    ),
                ))
                .await?;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_pull_request(
    pull: PullRequestEvent<'_>,
    store: &Store,
) -> ResponseResult<()> {
    let repo_name;
    let username;
    let current;
    let base;
    let additions;
    let deletions;
    let changed_files;
    let commits;
    let state;
    let pull_url;
    let title;
    let body;
    ready_to_fmt! {
        repo_name = pull.repository.full_name;
        username = pull.pull_request.user.login;
        current = pull.pull_request.head.ref_;
        base = pull.pull_request.base.ref_;
        ref additions = pull.pull_request.additions.to_string();
        ref deletions = pull.pull_request.deletions.to_string();
        ref changed_files = pull.pull_request.changed_files.to_string();
        ref commits = pull.pull_request.commits.to_string();
        state = match (pull.pull_request.mergeable.unwrap_or_default(), pull.pull_request.rebaseable.unwrap_or_default()) {
            (true, true) => "merge-able and rebase-able",
            (true, false) => "merge-able",
            (false, true) => "rebase-able",
            (false, false) => "in rough shape, neither rebase-able or merge-able",
        };
        ref pull_url = pull.pull_request.html_url.to_string();
        title = pull.pull_request.title;
        body = pull.pull_request.body;
    }

    let action = match pull.action {
        PullRequestAction::Assigned => {
            let assignee = pull
                .pull_request
                .assignee
                .map(|a| a.login.to_string())
                .unwrap_or_else(|| "<unknown>".to_owned());
            format!("PR was assigned to {}", assignee)
        }
        PullRequestAction::AutoMergeDisabled => {
            "PR now has auto merge disabled".to_owned()
        }
        PullRequestAction::AutoMergeEnabled => "PR now has auto merge enabled".to_owned(),
        PullRequestAction::Closed => {
            if pull.pull_request.merged.unwrap_or_default() {
                "PR was merged".to_owned()
            } else {
                "PR was closed without merging".to_owned()
            }
        }
        PullRequestAction::ConvertToDraft => "PR was converted to a draft".to_owned(),
        PullRequestAction::Edited => "PR has been edited".to_owned(),
        PullRequestAction::Labeled => "PR has been labeled".to_owned(),
        PullRequestAction::Locked => "PR has been locked".to_owned(),
        PullRequestAction::Opened => "PR has been opened".to_owned(),
        PullRequestAction::ReadyForReview => "PR is ready for review".to_owned(),
        PullRequestAction::Reopened => "PR has been reopened".to_owned(),
        PullRequestAction::ReviewRequestedRemoved => {
            "PR has review request removed".to_owned()
        }
        PullRequestAction::ReviewRequested => "PR has a review requested".to_owned(),
        PullRequestAction::Synchronize => "PR was synchronized".to_owned(),
        PullRequestAction::Unassigned => "PR was unassigned".to_owned(),
        PullRequestAction::Unlabeled => "PR was unlabeled".to_owned(),
        PullRequestAction::Unlocked => "PR was unlocked".to_owned(),
        _ => "<unknown>".to_owned(),
    };
    let action = &action;

    let fmt_str = store.config.github.format_strings.get("pull_request");
    let room = store
        .config
        .github
        .repos
        .iter()
        .find(|map| map.repo == repo_name)
        .map(|r| r.room.clone());
    match (fmt_str, room) {
        (Some(fmt_str), Some(room)) => {
            store
                .to_matrix
                .send((
                    room,
                    str_fmt!(
                        fmt_str,
                        repo_name,
                        username,
                        action,
                        current,
                        base,
                        additions,
                        deletions,
                        changed_files,
                        commits,
                        state,
                        pull_url,
                        title,
                        body,
                    ),
                ))
                .await?;
        }
        (None, Some(room)) => {
            store
                .to_matrix
                .send((
                    room,
                    format!(
                        r#"[{}] {}'s PR has new activity: {}
[Check out the pull request!]({})
{}
{} was opened against {}, has {} commits
++ {}
-- {}
{} changed files
This PR is {}"#,
                        repo_name,
                        username,
                        action,
                        pull_url,
                        title,
                        current,
                        base,
                        commits,
                        additions,
                        deletions,
                        changed_files,
                        state,
                    ),
                ))
                .await?;
        }
        _ => {}
    }

    Ok(())
}

async fn handle_push(push: PushEvent<'_>, store: &Store) -> ResponseResult<()> {
    let username;
    let repo_name;
    let commits_url;
    let commits_count;
    let plural;
    let branch;
    ready_to_fmt! {
        username = push.sender.login;
        repo_name = push.repository.full_name;
        commits_url = push.compare;
        ref commits_count = push.commits.len().to_string();
        plural = if push.commits.len() > 1 { "s" } else { "" };
        ref branch = push.ref_.split('/').last().map(|s| s.to_string()).unwrap_or_default();
    }

    let fmt_str = store.config.github.format_strings.get("push");
    let room = store
        .config
        .github
        .repos
        .iter()
        .find(|map| map.repo == repo_name)
        .map(|r| r.room.clone());
    match (fmt_str, room) {
        (Some(fmt_str), Some(room)) => {
            store
                .to_matrix
                .send((
                    room,
                    str_fmt!(
                        fmt_str,
                        repo_name,
                        username,
                        commits_count,
                        plural,
                        branch,
                        commits_url
                    ),
                ))
                .await?;
        }
        (None, Some(room)) => {
            store
                .to_matrix
                .send((
                    room,
                    format!(
                        "[{}] {} pushed {} commit{} to {}.\n[Check out the diff!]({})",
                        repo_name, username, commits_count, plural, branch, commits_url
                    ),
                ))
                .await?;
        }
        _ => {}
    }
    Ok(())
}

macro_rules! ready_to_fmt {
    (ref $name:ident = $init:expr; $($rest:tt)+) => {
        let s = $init;
        $name = &s;
        ready_to_fmt!($($rest)+)
    };
    ($name:ident = $init:expr; $($rest:tt)+) => {
        $name = $init;
        ready_to_fmt!($($rest)+)
    };
    (ref $name:ident = $init:expr;) => {
        let s = $init;
        $name = &s;
    };
    ($name:ident = $init:expr;) => {
        $name = $init;
    };
}
pub(crate) use ready_to_fmt;
