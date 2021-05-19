use serde::Deserialize;

mod common;
mod pull;
mod star;
mod status;

use pull::PullRequest;
use star::StarEvent;
use status::StatusEvent;

/// Describes the GitHub webhook event that triggered this request.
#[derive(Clone, Debug, Deserialize)]
pub enum GitHubEvent {
    PullRequest(PullRequest),
    IssueComment(),
    Status(StatusEvent),
    Star(StarEvent),
}

#[cfg(test)]
mod test {
    use matrix_sdk::uint;

    use crate::api::{
        common::{Commit, CommitInner, IssueState, Repo, Type, User},
        pull::{PullAction, PullEvent, PullRequest},
        star::{StarAction, StarEvent},
        status::{StatusEvent, StatusState},
    };

    #[test]
    fn stared() {
        let json = include_str!("../test_json/star.json");
        let star = serde_json::from_str::<StarEvent>(json).unwrap();
        assert!(matches!(
            star,
            StarEvent {
                action: StarAction::Created,
                repository: Repo { name, all_urls, .. },
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                ..
            } if name == "cargo-sort"
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
        ))
    }

    #[test]
    fn status() {
        let json = include_str!("../test_json/status.json");
        let star = serde_json::from_str::<StatusEvent>(json).unwrap();
        assert!(matches!(
            star,
            StatusEvent {
                state: StatusState::Success,
                branches,
                commit: Commit { commit: CommitInner { message, .. }, .. },
                repository: Repo { name, all_urls, .. },
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                ..
            } if name == "Hello-World"
                && message == "Initial commit"
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
                && branches.len() == 3
        ))
    }

    #[test]
    fn pull() {
        let json = include_str!("../test_json/pull.json");
        let star = serde_json::from_str::<PullEvent>(json).unwrap();
        assert!(matches!(
            star,
            PullEvent {
                action: PullAction::Opened,
                number,
                pull_request: PullRequest {
                    state: IssueState::Open, ..
                },
                repository: Repo { name, all_urls, .. },
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                ..
            } if name == "Hello-World"
                && number == uint!(2)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()

        ))
    }
}
