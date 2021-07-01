pub mod check_run;
pub mod check_suite;
pub mod commit_comment;
pub mod common;
pub mod create;
pub mod delete;
pub mod installation;
pub mod issue;
pub mod issue_comment;
pub mod milestone;
pub mod pull;
pub mod pull_review;
pub mod pull_review_comment;
pub mod push;
pub mod release;
pub mod star;
pub mod status;
pub mod watch;

use check_run::CheckRunEvent;
use check_suite::CheckSuiteEvent;
use commit_comment::CommitCommentEvent;
use common::EventKind;
use create::CreateEvent;
use delete::DeleteEvent;
use installation::InstallationEvent;
use issue::IssueEvent;
use issue_comment::IssueCommentEvent;
use milestone::MilestoneEvent;
use pull::PullRequestEvent;
use pull_review::PullRequestReviewEvent;
use pull_review_comment::PullRequestReviewCommentEvent;
use push::PushEvent;
use release::ReleaseEvent;
use star::StarEvent;
use status::StatusEvent;
use watch::WatchEvent;

/// Describes the GitHub webhook event that triggered this request.
///
/// `GithubEvent` does not implement `Deserialize` because there is no information in
/// the payload to find which variant, that information is in the request header.
#[allow(clippy::large_enum_variant)]
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
    Create(CreateEvent<'req>),

    /// The create payload of a github webhook.
    // TODO: make the struct
    Delete(DeleteEvent<'req>),

    /// The installation payload of a github webhook.
    Installation(InstallationEvent<'req>),

    /// The issue payload of a github webhook.
    Issue(IssueEvent<'req>),

    /// The issue comment payload of a github webhook.
    IssueComment(IssueCommentEvent<'req>),

    /// The milestone payload of a github webhook.
    Milestone(MilestoneEvent<'req>),

    /// The milestone payload of a github webhook.
    Ping(serde_json::Value),

    /// The pull request payload of a github webhook.
    PullRequest(PullRequestEvent<'req>),

    /// The pull request review payload of a github webhook.
    PullRequestReview(PullRequestReviewEvent<'req>),

    /// The pull request review comment payload of a github webhook.
    PullRequestReviewComment(PullRequestReviewCommentEvent<'req>),

    /// The push payload of a github webhook.
    Push(PushEvent<'req>),

    /// The release comment payload of a github webhook.
    Release(ReleaseEvent<'req>),

    /// The stared payload of a github webhook.
    Star(StarEvent<'req>),

    /// The status payload of a github webhook.
    Status(StatusEvent<'req>),

    /// The watch payload of a github webhook.
    Watch(WatchEvent<'req>),
}

impl<'a> GitHubEvent<'a> {
    pub fn as_kind(&self) -> EventKind {
        match self {
            GitHubEvent::CheckRun(_) => EventKind::CheckRun,
            GitHubEvent::CheckSuite(_) => EventKind::CheckSuite,
            GitHubEvent::CommitComment(_) => EventKind::CommitComment,
            GitHubEvent::Create(_) => EventKind::Create,
            GitHubEvent::Delete(_) => EventKind::Delete,
            GitHubEvent::Installation(_) => EventKind::Installation,
            GitHubEvent::Issue(_) => EventKind::Issues,
            GitHubEvent::IssueComment(_) => EventKind::IssueComment,
            GitHubEvent::Milestone(_) => EventKind::Milestone,
            GitHubEvent::Ping(_) => EventKind::Ping,
            GitHubEvent::PullRequest(_) => EventKind::PullRequest,
            GitHubEvent::PullRequestReview(_) => EventKind::PullRequestReview,
            GitHubEvent::PullRequestReviewComment(_) => {
                EventKind::PullRequestReviewComment
            }
            GitHubEvent::Push(_) => EventKind::Push,
            GitHubEvent::Release(_) => EventKind::Release,
            GitHubEvent::Star(_) => EventKind::Star,
            GitHubEvent::Status(_) => EventKind::Status,
            GitHubEvent::Watch(_) => EventKind::Watch,
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use js_int::uint;

    use crate::api::webhooks::{
        check_run::{CheckRun, CheckRunEvent, Deployment, Output},
        check_suite::{CheckAction, CheckSuite, CheckSuiteEvent},
        commit_comment::{CommitComment, CommitCommentAction, CommitCommentEvent},
        common::{
            App, AuthorAssociation, Changes, Comment, Commit, CommitInner, Committer,
            Installation, Issue, IssueState, Milestone, PullRequest, Repository, Type,
            User,
        },
        create::{CreateEvent, RefType},
        delete::DeleteEvent,
        installation::{InstallationAction, InstallationEvent},
        issue::{IssueAction, IssueEvent},
        issue_comment::{IssueCommentAction, IssueCommentEvent},
        milestone::{MilestoneAction, MilestoneEvent},
        pull::{PullRequestAction, PullRequestEvent},
        pull_review::{
            PullRequestReview, PullRequestReviewAction, PullRequestReviewEvent,
        },
        pull_review_comment::{
            PullRequestReviewComment, PullRequestReviewCommentAction,
            PullRequestReviewCommentEvent,
        },
        push::PushEvent,
        release::{Release, ReleaseAction, ReleaseEvent},
        star::{StarAction, StarEvent},
        status::{StatusEvent, StatusState},
        watch::{WatchAction, WatchEvent},
    };

    #[test]
    fn stared() {
        let json = include_str!("../../test_json/star.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let star = serde_path_to_error::deserialize::<_, StarEvent<'_>>(jd).unwrap();

        assert!(matches!(
            star,
            StarEvent {
                action: StarAction::Created,
                repository: Repository { name, all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                ..
            } if name == "cargo-sort"
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
        ))
    }

    #[test]
    fn status() {
        let json = include_str!("../../test_json/status.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let status = serde_path_to_error::deserialize::<_, StatusEvent<'_>>(jd).unwrap();

        assert!(matches!(
            status,
            StatusEvent {
                state: StatusState::Success,
                branches,
                commit: Commit { commit: CommitInner { message, .. }, .. },
                repository: Repository { name, all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
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
        let json = include_str!("../../test_json/pull_request.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let pull =
            serde_path_to_error::deserialize::<_, PullRequestEvent<'_>>(jd).unwrap();

        assert!(matches!(
            pull,
            PullRequestEvent {
                action: PullRequestAction::Opened,
                number,
                pull_request: PullRequest {
                    state: IssueState::Open, all_urls: pull_urls, ..
                },
                repository: Repository { name, all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
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
        let json = include_str!("../../test_json/issue.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let issue = serde_path_to_error::deserialize::<_, IssueEvent<'_>>(jd).unwrap();

        assert!(matches!(
            issue,
            IssueEvent {
                action: IssueAction::Edited,
                issue: Issue { number, .. },
                repository: Repository { name, all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
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
        let json = include_str!("../../test_json/installation.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let installation =
            serde_path_to_error::deserialize::<_, InstallationEvent<'_>>(jd).unwrap();

        assert!(matches!(
            installation,
            InstallationEvent {
                action: InstallationAction::Deleted,
                installation: Installation {
                    events, app_id, account: User { login, ..}, ..
                },
                repositories,
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
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
        let json = include_str!("../../test_json/check_suite.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let check_suite =
            serde_path_to_error::deserialize::<_, CheckSuiteEvent<'_>>(jd).unwrap();

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
                repository: Repository { all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if head_branch == "changes"
                && id == uint!(118_578_147)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
                && !pull_requests.is_empty()
                && events.is_empty()
        ))
    }

    #[test]
    fn check_run() {
        let json = include_str!("../../test_json/check_run.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let check_run =
            serde_path_to_error::deserialize::<_, CheckRunEvent<'_>>(jd).unwrap();

        assert!(matches!(
            check_run,
            CheckRunEvent {
                action: CheckAction::Created,
                check_run: CheckRun {
                    id,
                    pull_requests,
                    deployment: Some(Deployment { environment, .. }),
                    output: Output { annotations_count, .. },
                    ..
                },
                repository: Repository { all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if environment == "lab"
                && id == uint!(128_620_228)
                && annotations_count == uint!(0)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
                && !pull_requests.is_empty()
        ))
    }

    #[test]
    fn commit_comment() {
        let json = include_str!("../../test_json/commit_comment.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let commit_comment =
            serde_path_to_error::deserialize::<_, CommitCommentEvent<'_>>(jd).unwrap();

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
                repository: Repository { all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if path == Some(Path::new("hello/world.rs"))
                && id == uint!(33_548_674)
                && position == Some(uint!(10))
                && line == Some(uint!(10))
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
        ))
    }

    #[test]
    fn issue_comment() {
        let json = include_str!("../../test_json/issue_comment.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let issue_comment =
            serde_path_to_error::deserialize::<_, IssueCommentEvent<'_>>(jd).unwrap();

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
                repository: Repository { all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if body == Some("You are totally right! I'll get this fixed right away.")
                && id == uint!(492_700_400)
                && number == uint!(1)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
        ))
    }

    #[test]
    fn pull_review() {
        let json = include_str!("../../test_json/pull_review.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let pull_review =
            serde_path_to_error::deserialize::<_, PullRequestReviewEvent<'_>>(jd)
                .unwrap();

        assert!(matches!(
            pull_review,
            PullRequestReviewEvent {
                action: PullRequestReviewAction::Submitted,
                review: PullRequestReview { state, .. },
                pull_request: PullRequest {
                    state: IssueState::Open, all_urls: pull_urls, number, ..
                },
                repository: Repository { name, all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
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

    #[test]
    fn push() {
        let json = include_str!("../../test_json/push.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let push = serde_path_to_error::deserialize::<_, PushEvent<'_>>(jd).unwrap();

        assert!(matches!(
            push,
            PushEvent {
                created, deleted,
                pusher: Committer { name: committer_name, .. },
                commits,
                repository: Repository { name, all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                ..
            } if name == "Hello-World"
                && committer_name == Some("Codertocat")
                && !created && deleted
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
                && commits.is_empty()
        ))
    }

    #[test]
    fn release() {
        let json = include_str!("../../test_json/release.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let release =
            serde_path_to_error::deserialize::<_, ReleaseEvent<'_>>(jd).unwrap();

        assert!(matches!(
            release,
            ReleaseEvent {
                action: ReleaseAction::Published,
                release: Release { target_commitish, assets, .. },
                repository: Repository { name, all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if name == "Hello-World"
                && target_commitish == "master"
                && assets.is_empty()
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
        ))
    }

    #[test]
    fn create() {
        let json = include_str!("../../test_json/create.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let create = serde_path_to_error::deserialize::<_, CreateEvent<'_>>(jd).unwrap();

        assert!(matches!(
            create,
            CreateEvent {
                ref_,
                ref_type: RefType::Tag,
                pusher_type,
                description: None,
                master_branch,
                repository: Repository { all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if ref_ == "simple-tag"
                && pusher_type == "user"
                && master_branch == "master"
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
        ))
    }

    #[test]
    fn delete() {
        let json = include_str!("../../test_json/delete.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let delete = serde_path_to_error::deserialize::<_, DeleteEvent<'_>>(jd).unwrap();

        assert!(matches!(
            delete,
            DeleteEvent {
                ref_,
                ref_type: RefType::Tag,
                pusher_type,
                repository: Repository { all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if ref_ == "simple-tag"
                && pusher_type == "user"
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
        ))
    }

    #[test]
    fn milestone() {
        let json = include_str!("../../test_json/milestone.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let milestone =
            serde_path_to_error::deserialize::<_, MilestoneEvent<'_>>(jd).unwrap();

        assert!(matches!(
            milestone,
            MilestoneEvent {
                action: MilestoneAction::Created,
                milestone: Milestone { title, number, .. },
                changes: Some(Changes { .. }),
                repository: Repository { all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if title == "v1.0"
                && number == uint!(1)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
        ))
    }

    #[test]
    fn watch() {
        let json = include_str!("../../test_json/watch.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let watch = serde_path_to_error::deserialize::<_, WatchEvent<'_>>(jd).unwrap();

        assert!(matches!(
            watch,
            WatchEvent {
                action: WatchAction::Started,
                repository: Repository { .. },
                sender: User { .. },
                organization: None,
                installation: None,
                ..
            }
        ))
    }

    #[test]
    fn pull_request_review_comment() {
        let json = include_str!("../../test_json/pull_review_comment.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let pull =
            serde_path_to_error::deserialize::<_, PullRequestReviewCommentEvent<'_>>(jd)
                .unwrap();

        assert!(matches!(
            pull,
            PullRequestReviewCommentEvent {
                action: PullRequestReviewCommentAction::Created,
                comment: PullRequestReviewComment { diff_hunk, commit_id, .. },
                repository: Repository { name, all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                ..
            } if diff_hunk == "@@ -1 +1 @@\n-# Hello-World"
                && commit_id == "ec26c3e57ca3a959ca5aad62de7213c562f8c821"
                && name == "Hello-World"
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
        ))
    }

    #[test]
    fn real_push_event() {
        let json = include_str!("../../test_json/real_push.json");

        let jd = &mut serde_json::Deserializer::from_str(json);
        let push = serde_path_to_error::deserialize::<_, PushEvent<'_>>(jd).unwrap();

        assert!(matches!(
            push,
            PushEvent {
                created, deleted, forced,
                pusher: Committer { name: committer_name, .. },
                commits,
                repository: Repository { name, all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                ..
            } if name == "magit"
                && committer_name == Some("DevinR528")
                && !created && !deleted && !forced
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
                && commits.len() == 4
        ))
    }

    #[test]
    fn real_check_run() {
        let json = include_str!("../../test_json/real_check_run.json");
        // Some Deserializer.
        let jd = &mut serde_json::Deserializer::from_str(json);
        let check_run =
            serde_path_to_error::deserialize::<_, CheckRunEvent<'_>>(jd).unwrap();

        assert!(matches!(
            check_run,
            CheckRunEvent {
                action: CheckAction::Created,
                check_run: CheckRun {
                    id,
                    pull_requests,
                    deployment: None,
                    output: Output { annotations_count, .. },
                    ..
                },
                repository: Repository { all_urls, .. },
                sender: User { kind: Some(Type::User), all_urls: sender_urls, .. },
                organization: None,
                installation: None,
                ..
            } if id == uint!(2_786_954_999)
                && annotations_count == uint!(0)
                && !sender_urls.is_empty()
                && !all_urls.is_empty()
                && !pull_requests.is_empty()
        ))
    }
}
