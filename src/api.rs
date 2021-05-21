mod check_run;
mod check_suite;
mod commit_comment;
mod common;
mod installation;
mod issue;
mod issue_comment;
mod pull;
mod pull_review;
mod star;
mod status;

use check_run::CheckRunEvent;
use check_suite::CheckSuiteEvent;
use commit_comment::CommitCommentEvent;
use installation::InstallationEvent;
use issue::IssueEvent;
use issue_comment::IssueCommentEvent;
use pull::PullRequestEvent;
use pull_review::PullRequestReviewEvent;
use star::StarEvent;
use status::StatusEvent;

/// Describes the GitHub webhook event that triggered this request.
///
/// `GithubEvent` does not implement `Deserialize` because there is no information in the
/// payload to find which variant, that information is in the request header.
#[derive(Clone, Debug)]
pub enum GitHubEvent<'req> {
    /// The check run payload of a github webhook.
    CheckRun(CheckRunEvent<'req>),

    /// The check suite payload of a github webhook.
    CheckSuite(CheckSuiteEvent<'req>),

    /// The check suite payload of a github webhook.
    CommitComment(CommitCommentEvent<'req>),

    /// The create payload of a github webhook.
    // TODO: make the struct
    Create(serde_json::Value),

    /// The installation payload of a github webhook.
    Installation(InstallationEvent<'req>),

    /// The issue payload of a github webhook.
    Issue(IssueEvent<'req>),

    /// The issue comment payload of a github webhook.
    IssueComment(IssueCommentEvent<'req>),

    /// The pull request payload of a github webhook.
    PullRequest(PullRequestEvent<'req>),

    /// The pull request review payload of a github webhook.
    PullRequestReview(PullRequestReviewEvent<'req>),

    /// The pull request review comment payload of a github webhook.
    PullRequestReviewComment(serde_json::Value),

    /// The push payload of a github webhook.
    Push(serde_json::Value),

    /// The release comment payload of a github webhook.
    Release(serde_json::Value),

    /// The stared payload of a github webhook.
    Star(StarEvent<'req>),

    /// The status payload of a github webhook.
    Status(StatusEvent<'req>),
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use matrix_sdk::uint;

    use crate::api::{
        check_run::{CheckRun, CheckRunEvent, Deployment, Output},
        check_suite::{CheckAction, CheckSuite, CheckSuiteEvent},
        commit_comment::{CommitComment, CommitCommentAction, CommitCommentEvent},
        common::{
            App, AuthorAssociation, Commit, CommitInner, IssueState, Repo, Type, User,
        },
        installation::{Installation, InstallationAction, InstallationEvent},
        issue::{Issue, IssueAction, IssueEvent},
        issue_comment::{Comment, IssueCommentAction, IssueCommentEvent},
        pull::{PullRequest, PullRequestAction, PullRequestEvent},
        pull_review::{
            PullRequestReview, PullRequestReviewAction, PullRequestReviewEvent,
        },
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
        let pull = serde_path_to_error::deserialize::<_, PullRequestEvent>(jd).unwrap();

        assert!(matches!(
            pull,
            PullRequestEvent {
                action: PullRequestAction::Opened,
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

    #[test]
    fn check_suite() {
        let json = include_str!("../test_json/check_suite.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let check_suite =
            serde_path_to_error::deserialize::<_, CheckSuiteEvent>(jd).unwrap();

        assert!(matches!(
            check_suite,
            CheckSuiteEvent {
                action: CheckAction::Completed,
                check_suite: CheckSuite {
                    id,
                    head_branch,
                    pull_requests,
                    app: App { events, .. },
                    ..
                },
                repository: Repo { all_urls, .. },
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if head_branch == "changes"
                && id == uint!(118578147)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
                && !pull_requests.is_empty()
                && events.is_empty()

        ))
    }

    #[test]
    fn check_run() {
        let json = include_str!("../test_json/check_run.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let check_run = serde_path_to_error::deserialize::<_, CheckRunEvent>(jd).unwrap();

        assert!(matches!(
            check_run,
            CheckRunEvent {
                action: CheckAction::Created,
                check_run: CheckRun {
                    id,
                    pull_requests,
                    deployment: Deployment { environment, .. },
                    output: Output { annotations_count, .. },
                    ..
                },
                repository: Repo { all_urls, .. },
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if environment == "lab"
                && id == uint!(128620228)
                && annotations_count == uint!(0)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
                && !pull_requests.is_empty()

        ))
    }

    #[test]
    fn commit_comment() {
        let json = include_str!("../test_json/commit_comment.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let commit_comment =
            serde_path_to_error::deserialize::<_, CommitCommentEvent>(jd).unwrap();

        assert!(matches!(
            commit_comment,
            CommitCommentEvent {
                action: CommitCommentAction::Created,
                comment: CommitComment {
                    id,
                    position,
                    path,
                    line,
                    author_association: AuthorAssociation::Owner,
                    ..
                },
                repository: Repo { all_urls, .. },
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if path == Some(Path::new("hello/world.rs"))
                && id == uint!(33548674)
                && position == Some(uint!(10))
                && line == Some(uint!(10))
                && !sender_urls.is_empty()
                && !all_urls.is_empty()

        ))
    }

    #[test]
    fn issue_comment() {
        let json = include_str!("../test_json/issue_comment.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let issue_comment =
            serde_path_to_error::deserialize::<_, IssueCommentEvent>(jd).unwrap();

        assert!(matches!(
            issue_comment,
            IssueCommentEvent {
                action: IssueCommentAction::Created,
                issue: Issue { state: IssueState::Open, number, .. },
                comment: Comment {
                    id,
                    body,
                    author_association: AuthorAssociation::Owner,
                    ..
                },
                repository: Repo { all_urls, .. },
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if body == Some("You are totally right! I'll get this fixed right away.")
                && id == uint!(492700400)
                && number == uint!(1)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()

        ))
    }

    #[test]
    fn pull_review() {
        let json = include_str!("../test_json/pull_review.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let pull_review =
            serde_path_to_error::deserialize::<_, PullRequestReviewEvent>(jd).unwrap();

        assert!(matches!(
            pull_review,
            PullRequestReviewEvent {
                action: PullRequestReviewAction::Submitted,
                review: PullRequestReview { state, .. },
                pull_request: PullRequest {
                    state: IssueState::Open, all_urls: pull_urls, number, ..
                },
                repository: Repo { name, all_urls, .. },
                sender: User { kind: Type::User, all_urls: sender_urls, .. },
                organization: None,
                ..
            } if name == "Hello-World"
                && state == "commented"
                && number == uint!(2)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
                && !pull_urls.is_empty()

        ))
    }
}
