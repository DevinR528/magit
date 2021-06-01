use std::fmt;

use github_derive::StringEnum;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub enum Type {
    User,
    Organization,
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected, Visitor};

        struct StrVisitor;

        impl<'a> Visitor<'a> for StrVisitor {
            type Value = &'a str;

            // TODO: finish list
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("one of User, Organization")
            }

            fn visit_borrowed_str<E: Error>(self, v: &'a str) -> Result<Self::Value, E> {
                Ok(v) // so easy
            }

            fn visit_borrowed_bytes<E: Error>(
                self,
                v: &'a [u8],
            ) -> Result<Self::Value, E> {
                std::str::from_utf8(v)
                    .map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
            }
        }
        let s = d.deserialize_str(StrVisitor)?;

        match s {
            "User" => Ok(Self::User),
            "Organization" => Ok(Self::Organization),
            _ => Err(Error::invalid_value(Unexpected::Str(s), &StrVisitor)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum LockReason {
    /// Locked this because it was resolved.
    Resolved,

    /// Locked this because it was off topic.
    OffTopic,

    /// Locked this because it was too heated.
    TooHeated,

    /// Locked this because of spam.
    Spam,
}

impl<'de> Deserialize<'de> for LockReason {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected, Visitor};

        struct StrVisitor;

        impl<'a> Visitor<'a> for StrVisitor {
            type Value = &'a str;

            // TODO: finish list
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("one of User, Organization")
            }

            fn visit_borrowed_str<E: Error>(self, v: &'a str) -> Result<Self::Value, E> {
                Ok(v) // so easy
            }

            fn visit_borrowed_bytes<E: Error>(
                self,
                v: &'a [u8],
            ) -> Result<Self::Value, E> {
                std::str::from_utf8(v)
                    .map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
            }
        }
        let s = d.deserialize_str(StrVisitor)?;

        match s {
            "resolved" => Ok(Self::Resolved),
            "off-topic" => Ok(Self::OffTopic),
            "too heated" => Ok(Self::TooHeated),
            "spam" => Ok(Self::Spam),
            _ => Err(Error::invalid_value(Unexpected::Str(s), &StrVisitor)),
        }
    }
}

#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "snake_case")]
pub enum RepoSelection {
    All,
    Selected,
}

#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "snake_case")]
pub enum RepoPermission {
    /// Read only access.
    Read,

    /// Write access.
    Write,

    /// Complete administrative access.
    Admin,

    /// No access.
    ///
    /// This is the equivalent of `null` or not being present in the JSON.
    None,
}

impl Default for RepoPermission {
    fn default() -> Self { Self::Read }
}

#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
pub enum RepoCreationType {
    All,
    Private,
    None,
}

impl Default for RepoCreationType {
    fn default() -> Self { Self::All }
}

#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
pub enum IssueState {
    Open,
    Closed,
    Unknown,
}

impl Default for IssueState {
    fn default() -> Self { Self::Open }
}

/// An enum representing all the different payload event types within the Github webhooks
/// API.
#[derive(Clone, Debug, PartialEq, Eq, StringEnum)]
#[github_enum(rename_all = "snake_case")]
pub enum EventKind {
    CheckRun,
    CheckSuite,
    CodeScanningAlert,
    CommitComment,
    ContentReference,
    Create,
    Delete,
    Deployment,
    DeploymentReview,
    DeploymentStatus,
    DeployKey,
    Discussion,
    DiscussionComment,
    Fork,
    Gollum,
    Installation,
    Issues,
    IssueComment,
    Label,
    Member,
    Membership,
    Milestone,
    Organization,
    OrgBlock,
    PageBuild,
    Ping,
    Project,
    ProjectCard,
    ProjectColumn,
    Public,
    PullRequest,
    PullRequestReview,
    PullRequestReviewComment,
    Push,
    RegistryPackage,
    Release,
    Repository,
    RepositoryDispatch,
    SecretScanningAlert,
    Star,
    Status,
    Team,
    TeamAdd,
    Watch,
    WorkflowDispatch,
    WorkflowRun,
}

#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorAssociation {
    Collaborator,
    Contributor,
    FirstTime,
    FirstTimeContributor,
    Mannequin,
    Member,
    Owner,
    None,
}

impl Default for AuthorAssociation {
    fn default() -> Self { Self::None }
}

#[derive(Debug, Copy, Clone, StringEnum)]
#[github_enum(rename_all = "snake_case")]
pub enum CheckStatus {
    /// There are checks queued to run.
    Queued,

    /// This check has finished.
    Completed,

    /// The check is in progress.
    InProgress,

    /// The check has been requested.
    Requested,

    /// `None` is that same as not present or null.
    None,
}

impl Default for CheckStatus {
    fn default() -> Self { Self::None }
}

#[derive(Debug, Copy, Clone, StringEnum)]
#[github_enum(rename_all = "snake_case")]
pub enum ConclusionStatus {
    /// The check has succeeded.
    Success,

    /// The check has failed.
    Failure,

    /// The check has finished with a neutral result.
    Neutral,

    /// The check has been canceled.
    Cancelled,

    /// The check has timed out.
    TimeOut,

    /// The check needs attention.
    ActionRequired,

    /// The check has gone stale.
    ///
    /// Something has changed while the check was running.
    Stale,

    /// `None` is that same as not present or null.
    None,
}

impl Default for ConclusionStatus {
    fn default() -> Self { Self::None }
}

/// The event that triggered a workflow run.
#[derive(Clone, Debug, PartialEq, Eq, StringEnum)]
#[github_enum(rename_all = "snake_case")]
pub enum WorkflowEvent {
    /// A push from a branch that has a workflow enabled.
    Push,

    /// A pull request against a branch that has workflows enabled.
    PullRequest,

    /// An issue opened that has a workflow associated with it.
    Issue,
}

/// The status of a file tracked by github.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
pub enum FileStatus {
    /// The file has been added / created.
    Added,

    /// The file has been added.
    Modified,

    /// The file has been removed.
    Removed,

    /// The file has been renamed.
    Renamed,

    // TODO confirm
    Changed,
}

/// The status of a file tracked by github.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "snake_case")]
pub enum MergeStateStatus {
    /// The head ref is out of date.
    Behind,

    /// The merge is blocked.
    Blocked,

    /// Mergeable and passing commit status.
    Clean,

    /// The merge commit cannot be cleanly created..
    Dirty,

    // The merge is blocked due to the pull request being a draft.
    Draft,

    /// Mergeable with passing commit status and pre-receive hooks.
    HasHook,

    /// The state cannot currently be determined.
    Unknown,

    /// Mergeable with non-passing commit status.
    Unstable,
}
