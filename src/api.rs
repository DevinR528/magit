mod common;
mod installation;
mod issue;
mod pull;
mod star;
mod status;

use installation::InstallationEvent;
use issue::IssueEvent;
use pull::PullRequest;
use star::StarEvent;
use status::StatusEvent;

/// Describes the GitHub webhook event that triggered this request.
///
/// `GithubEvent` does not implement `Deserialize` because there is no information in the
/// payload to find which variant, that information is in the request header.
#[derive(Clone, Debug)]
pub enum GitHubEvent {
    /// The pull request payload of a github webhook.
    PullRequest(PullRequest),

    /// The installation payload of a github webhook.
    InstallationRequest(InstallationEvent),

    /// The issue payload of a github webhook.
    IssueComment(IssueEvent),

    /// The status payload of a github webhook.
    Status(StatusEvent),

    /// The stared payload of a github webhook.
    Star(StarEvent),
}

#[cfg(test)]
mod test {
    use matrix_sdk::uint;

    use crate::api::{
        common::{Commit, CommitInner, IssueState, Repo, Type, User},
        installation::{Installation, InstallationAction, InstallationEvent},
        issue::{Issue, IssueAction, IssueEvent},
        pull::{PullAction, PullEvent, PullRequest},
        star::{StarAction, StarEvent},
        status::{StatusEvent, StatusState},
    };

    #[test]
    fn stared() {
        let json = include_str!("../test_json/star.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let star = serde_path_to_error::deserialize::<_, StarEvent>(jd).unwrap();

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

        let jd = &mut serde_json::Deserializer::from_str(json);
        let status = serde_path_to_error::deserialize::<_, StatusEvent>(jd).unwrap();
        assert!(matches!(
            status,
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

        let jd = &mut serde_json::Deserializer::from_str(json);
        let pull = serde_path_to_error::deserialize::<_, PullEvent>(jd).unwrap();

        assert!(matches!(
            pull,
            PullEvent {
                action: PullAction::Opened,
                number,
                pull_request: PullRequest {
                    state: IssueState::Open, all_urls: pull_urls, ..
                },
                repository: Repo { name, all_urls, .. },
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                ..
            } if name == "Hello-World"
                && number == uint!(2)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
                && !pull_urls.is_empty()

        ))
    }

    #[test]
    fn issue() {
        let json = include_str!("../test_json/issue.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let issue = serde_path_to_error::deserialize::<_, IssueEvent>(jd).unwrap();
        assert!(matches!(
            issue,
            IssueEvent {
                action: IssueAction::Edited,
                issue: Issue { number, .. },
                repository: Repo { name, all_urls, .. },
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                ..
            } if name == "Hello-World"
                && number == uint!(1)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()

        ))
    }

    #[test]
    fn installation() {
        let json = include_str!("../test_json/installation.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let installation =
            serde_path_to_error::deserialize::<_, InstallationEvent>(jd).unwrap();
        assert!(matches!(
            installation,
            InstallationEvent {
                action: InstallationAction::Deleted,
                installation: Installation {
                    events, app_id, account: User { login, ..}, ..
                },
                repositories,
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                ..
            } if login == "octocat"
                && app_id == uint!(5725)
                && !sender_urls.is_empty()
                && !events.is_empty()
                && repositories.len() == 1

        ))
    }
}
