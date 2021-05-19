use std::{collections::BTreeMap, fmt};

use chrono::{DateTime, Utc};
use matrix_sdk::UInt;
use serde::Deserialize;
use url::Url;

pub type Dt = DateTime<Utc>;

/// Information about a specific commit.
#[derive(Clone, Debug, Deserialize)]
pub struct Commit {
    /// The sha of this commit.
    pub sha: String,

    /// The identifier of this commit.
    pub node_id: String,

    /// Information about this commit.
    pub commit: CommitInner,

    /// The api url of the commit referenced.
    pub url: String,

    /// The url to github webpage associated with this commit.
    pub html_url: Url,

    /// The api url to request information about comments.
    pub comments_url: String,

    /// The author of this commit.
    pub author: User,

    /// The user who committed the referenced commit.
    pub committer: User,

    /// A list of parents of this commit if any.
    pub parents: Vec<String>,
}

/// Further information about a commit.
#[derive(Clone, Debug, Deserialize)]
pub struct CommitInner {
    /// The url to this commit.
    pub url: Url,

    /// Information about author of this commit.
    pub author: ShortUser,

    /// Information about committer.
    pub committer: ShortUser,

    /// The commit message.
    pub message: String,

    /// SHA and url of the commit.
    pub tree: CommitTree,

    /// Number of comments associated with this commit.
    pub comment_count: UInt,

    /// Information about the verification of this commit.
    pub verification: Verification,
}

/// Information about a branch.
#[derive(Clone, Debug, Deserialize)]
pub struct Branch {
    /// The name of this branch.
    pub name: String,

    /// The last commit to this branch.
    pub commit: CommitTree,

    /// Is this branch protected.
    pub protected: bool,
}

/// Information about the verification of an object.
#[derive(Clone, Debug, Deserialize)]
pub struct Verification {
    /// Has this object been verified.
    pub verified: bool,

    /// Reason given about verification.
    ///
    /// "valid" on success, may give an error on failure.
    pub reason: String,

    /// The PGP signature of this commit.
    pub signature: Option<String>,

    /// The payload of this commit.
    ///
    /// Often source control specific information.
    pub payload: Option<String>,
}

/// Information any labels.
#[derive(Clone, Debug, Deserialize)]
pub struct Label {
    /// Numeric Id of this label.
    pub id: UInt,

    /// String identifier of the label.
    pub node_id: String,

    /// The name of this label.
    pub name: String,

    /// The short description of this label.
    pub description: String,

    /// Background color of the label box.
    pub color: String,

    /// Is this a default label.
    pub default: bool,
}

/// The links related to an issue or pull request.
#[derive(Clone, Debug)]
pub struct Links {
    pub self_link: Option<String>,
    pub html_link: Option<String>,
    pub issue_link: Option<String>,
    pub comments_link: Option<String>,
    pub review_comments_link: Option<String>,
    pub review_comment_link: Option<String>,
    pub commits_link: Option<String>,
    pub statuses_link: Option<String>,
    pub pull_request_link: Option<String>,
}

impl<'de> Deserialize<'de> for Links {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, MapAccess, Visitor};

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            #[serde(rename = "Self")]
            Self_,
            Html,
            Issue,
            Comments,
            ReviewComments,
            ReviewComment,
            Commits,
            Statuses,
            PullRequest,
        }

        struct LinksVisitor;
        impl<'a> Visitor<'a> for LinksVisitor {
            type Value = Links;

            // TODO: finish list
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("one of User, Owner, TODO")
            }

            fn visit_map<A: MapAccess<'a>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut self_link = None;
                let mut html_link = None;
                let mut issue_link = None;
                let mut comments_link = None;
                let mut review_comments_link = None;
                let mut review_comment_link = None;
                let mut commits_link = None;
                let mut statuses_link = None;
                let mut pull_request_link = None;
                // While there are entries remaining in the input, add them
                // into our map.
                while let Some((key, value)) =
                    map.next_entry::<Field, serde_json::Value>()?
                {
                    match key {
                        Field::Self_ => {
                            if self_link.is_some() {
                                return Err(Error::duplicate_field("self_link"));
                            }
                            self_link = value["href"].as_str().map(|s| s.to_owned());
                        }
                        Field::Html => {
                            if html_link.is_some() {
                                return Err(Error::duplicate_field("html_link"));
                            }
                            html_link = value["href"].as_str().map(|s| s.to_owned());
                        }
                        Field::Issue => {
                            if issue_link.is_some() {
                                return Err(Error::duplicate_field("issue_link"));
                            }
                            issue_link = value["href"].as_str().map(|s| s.to_owned());
                        }
                        Field::Comments => {
                            if comments_link.is_some() {
                                return Err(Error::duplicate_field("comments_link"));
                            }
                            comments_link = value["href"].as_str().map(|s| s.to_owned());
                        }
                        Field::ReviewComments => {
                            if review_comments_link.is_some() {
                                return Err(Error::duplicate_field(
                                    "review_comments_link",
                                ));
                            }
                            review_comments_link =
                                value["href"].as_str().map(|s| s.to_owned());
                        }
                        Field::ReviewComment => {
                            if review_comment_link.is_some() {
                                return Err(Error::duplicate_field(
                                    "review_comment_link",
                                ));
                            }
                            review_comment_link =
                                value["href"].as_str().map(|s| s.to_owned());
                        }
                        Field::Commits => {
                            if commits_link.is_some() {
                                return Err(Error::duplicate_field("commits_link"));
                            }
                            commits_link = value["href"].as_str().map(|s| s.to_owned());
                        }
                        Field::Statuses => {
                            if statuses_link.is_some() {
                                return Err(Error::duplicate_field("statuses_link"));
                            }
                            statuses_link = value["href"].as_str().map(|s| s.to_owned());
                        }
                        Field::PullRequest => {
                            if pull_request_link.is_some() {
                                return Err(Error::duplicate_field("pull_request_link"));
                            }
                            pull_request_link =
                                value["href"].as_str().map(|s| s.to_owned());
                        }
                    }
                }

                Ok(Links {
                    self_link,
                    html_link,
                    issue_link,
                    comments_link,
                    review_comments_link,
                    review_comment_link,
                    commits_link,
                    statuses_link,
                    pull_request_link,
                })
            }
        }
        const FIELDS: &[&str] = &[
            "self_link",
            "html_link",
            "issue_link",
            "comments_link",
            "review_comments_link",
            "review_comment_link",
            "commits_link",
            "statuses_link",
            "pull_request_link",
        ];
        d.deserialize_struct("Links", FIELDS, LinksVisitor)
    }
}

/// Simple information about a "user".
#[derive(Clone, Debug, Deserialize)]
pub struct ShortUser {
    /// Name of the user.
    pub name: String,

    /// Email of the user.
    pub email: String,

    /// The date of the event this user is related to happened.
    pub date: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CommitTree {
    /// SHA of the commit.
    pub sha: String,

    /// The url of this commit.
    pub url: Url,
}

/// Information about a repository.
#[derive(Clone, Debug, Deserialize)]
pub struct Repo {
    /// Numeric Id of this repository.
    pub id: UInt,

    /// String identifier of the repository.
    pub node_id: String,

    /// The name of this repository.
    pub name: String,

    /// The name including owner ie. `owner/repo-name`.
    pub full_name: String,

    /// The visibility of this repo.
    #[serde(default)]
    pub private: bool,

    /// Is this repo a fork.
    #[serde(default)]
    pub fork: bool,

    /// Information about the owner of this repository.
    pub owner: User,

    /// The public html web page url.
    pub html_url: Url,

    /// The url to the github api of this repo.
    pub url: String,

    /// Description of the repo.
    pub description: Option<String>,

    /// The time in UTC when the repo was created.
    pub created_at: Dt,

    /// The time in UTC when the repo was last updated.
    pub updated_at: Option<Dt>,

    /// The time in UTC when the repo was last pushed to.
    pub pushed_at: Option<Dt>,

    /// The url used when doing git operations.
    pub git_url: Option<String>,

    /// The url used when doing ssh operations.
    pub ssh_url: Option<String>,

    /// The url used to clone this repo.
    pub clone_url: Option<Url>,

    /// The url used for svn.
    pub svn_url: Option<Url>,

    /// The homepage of this repo, if set.
    pub homepage: Option<String>,

    /// Size of the repository.
    #[serde(default)]
    pub size: UInt,

    /// Number of stargazers (people who have starred the repo).
    #[serde(default)]
    pub stargazers_count: UInt,

    /// Number of people who watch this repo.
    #[serde(default)]
    pub watchers_count: UInt,

    /// The programming language used for this repo.
    pub language: Option<String>,

    /// Does this repo allow issues.
    #[serde(default)]
    pub has_issues: bool,

    /// Does this repo contain projects.
    #[serde(default)]
    pub has_projects: bool,

    /// Does this repo have downloadable resources.
    #[serde(default)]
    pub has_downloads: bool,

    /// Does this repo have a wiki.
    #[serde(default)]
    pub has_wiki: bool,

    /// Does this repo have github pages associated with it.
    #[serde(default)]
    pub has_pages: bool,

    /// How many times has this repo been forked.
    #[serde(default)]
    pub forks_count: UInt,

    /// The url to the repository this repo mirrors.
    pub mirror_url: Option<String>,

    /// Has this repo been archived.
    #[serde(default)]
    pub archived: bool,

    /// Has the repo been disabled.
    #[serde(default)]
    pub disabled: bool,

    /// How many open issues does this repo have.
    #[serde(default)]
    pub open_issues_count: UInt,

    /// License of this repo.
    pub license: Option<String>,

    /// Number of forks for the repo.
    #[serde(default)]
    pub forks: UInt,

    /// Number of open issues.
    #[serde(default)]
    pub open_issues: UInt,

    /// Number of watchers.
    #[serde(default)]
    pub watchers: UInt,

    /// This repositories default branch.
    pub default_branch: Option<String>,

    /// Allow squash and merge in web merge.
    #[serde(default = "true_fn")]
    pub allow_squash_merge: bool,

    /// Allow merge commit in web merge.
    #[serde(default)]
    pub allow_merge_commit: bool,

    /// Allow rebase and merge in web merge.
    #[serde(default = "true_fn")]
    pub allow_rebase_merge: bool,

    /// Allow branch to be deleted after merge.
    #[serde(default)]
    pub delete_branch_on_merge: bool,

    /// The topics this repo covers.
    #[serde(default)]
    pub topics: Vec<String>,

    /// The set permissions of this repo.
    #[serde(default)]
    pub permissions: Permissions,

    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}

/// Information about a github organization.
#[derive(Clone, Debug, Deserialize)]
pub struct Org {
    /// The name of the organization.
    pub login: String,

    /// Numeric identifier of the organization.
    #[serde(default)]
    pub id: UInt,

    /// String identifier of the organization.
    pub node_id: String,

    /// The url to the organizations github api.
    pub url: Url,

    /// Url to the organizations avatar image.
    pub avatar_url: Url,

    /// A description of the organization.
    #[serde(default)]
    pub description: String,

    /// The name of the organization.
    pub name: String,

    /// The name of the company associated with this organization.
    pub company: Option<String>,

    /// Url to a blog associated with this organization.
    pub blog: Option<String>,

    /// The location of this organization.
    pub location: Option<String>,

    /// An email address for this organization.
    pub email: String,

    /// The twitter user associated with this organization.
    pub twitter_username: Option<String>,

    /// Is this organization verified.
    pub is_verified: bool,

    /// Does this organization have projects.
    #[serde(default)]
    pub has_organization_projects: bool,
    /// Does this organization have repository projects.
    #[serde(default)]
    pub has_repository_projects: bool,

    /// Number of public repos.
    #[serde(default)]
    pub public_repos: UInt,

    /// Number of public gists.
    #[serde(default)]
    pub public_gists: UInt,

    /// Number of followers.
    #[serde(default)]
    pub followers: UInt,

    /// Number of users the organization is following.
    #[serde(default)]
    pub following: UInt,

    /// Url to the organizations github account.
    pub html_url: Url,

    /// Time in UTC this organization was created.
    pub created_at: Dt,

    /// Time in UTC this organization was last updated.
    pub updated_at: Dt,

    /// Type of resource this is.
    #[serde(rename = "type")]
    pub kind: Type,

    /// The number of private repos.
    #[serde(default)]
    pub total_private_repos: UInt,

    /// The number of owned private repos.
    #[serde(default)]
    pub owned_private_repos: UInt,

    /// The number of private gists.
    #[serde(default)]
    pub private_gists: UInt,

    /// How much space this organization takes up on disk.
    ///
    /// This measures the size of repos, gists, and any other resources an
    /// organization has.
    #[serde(default)]
    pub disk_usage: UInt,

    /// Number of collaborators.
    #[serde(default)]
    pub collaborators: UInt,

    /// The email of the person who pays.
    pub billing_email: Option<String>,

    /// The plan this organization is using.
    pub plan: Option<Plan>,

    /// The default permissions of a repository.
    #[serde(default)]
    pub default_repository_permission: RepoPermission,

    /// Can organization members create new repos.
    ///
    /// Note: defaults to true.
    #[serde(default = "true_fn")]
    pub members_can_create_repositories: bool,

    /// Does this organization require 2fa.
    #[serde(default)]
    pub two_factor_requirement_enabled: bool,

    /// The creation type for repositories in this organization.
    pub members_allowed_repository_creation_type: RepoCreationType,

    /// Note: defaults to true.
    #[serde(default = "true_fn")]
    pub members_can_create_public_repositories: bool,
    /// Note: defaults to true.
    #[serde(default = "true_fn")]
    pub members_can_create_private_repositories: bool,
    /// Note: defaults to true.
    #[serde(default = "true_fn")]
    pub members_can_create_internal_repositories: bool,
    /// Note: defaults to true.
    #[serde(default = "true_fn")]
    pub members_can_create_pages: bool,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}

/// Information about a user/organizations plan.
#[derive(Clone, Debug, Deserialize)]
pub struct Plan {
    /// The name of this plan.
    pub name: String,

    /// How much space does the organization have.
    #[serde(default)]
    pub space: UInt,

    /// The number of private repositories this org has.
    #[serde(default)]
    pub private_repos: UInt,

    /// Number of members.
    #[serde(default)]
    pub filled_seats: UInt,

    /// Number of allowed members.
    #[serde(default)]
    pub seats: UInt,
}

/// Information about a user.
///
/// This can be used for identifying an organization, owner, or
/// sender.
#[derive(Clone, Debug, Deserialize)]
pub struct User {
    /// The name of the user.
    pub login: String,

    /// The numeric identifier of this user.
    pub id: UInt,

    /// String identifier of the user.
    pub node_id: String,

    /// The users avatar url.
    pub avatar_url: Url,

    pub gravatar_id: String,

    /// Url to the github api for this user.
    pub url: String,

    /// Url to the github webpage of this user.
    pub html_url: String,

    /// The type of user.
    #[serde(rename = "type")]
    pub kind: Type,

    /// Is this the administrator of this resource.
    pub site_admin: bool,

    /// A map of all the github api urls.
    #[serde(flatten, default)]
    pub all_urls: UrlMap,
}

/// The permissions a repository has.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Permissions {
    /// Administrative privileges.
    #[serde(default)]
    admin: bool,

    /// Are pushes enabled.
    #[serde(default)]
    push: bool,

    /// Is pulling permitted.
    #[serde(default)]
    pull: bool,
}

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
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepoPermission {
    Read,
    Write,
    Admin,
    None,
}

impl Default for RepoPermission {
    fn default() -> Self { Self::Read }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RepoCreationType {
    All,
    Private,
    None,
}

impl Default for RepoCreationType {
    fn default() -> Self { Self::All }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueState {
    Open,
    Closed,
    Unknown,
}

impl Default for IssueState {
    fn default() -> Self { Self::Open }
}

#[derive(Clone, Debug)]
pub struct UrlMap {
    urls: BTreeMap<String, String>,
}

#[allow(unused)]
impl UrlMap {
    pub fn get(&self, k: &str) -> Option<&str> { self.urls.get(k).map(|s| s.as_str()) }
    pub fn len(&self) -> usize { self.urls.len() }
    pub fn is_empty(&self) -> bool { self.urls.is_empty() }
}

impl<'de> Deserialize<'de> for UrlMap {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};

        struct UrlMapVisitor;
        impl<'a> Visitor<'a> for UrlMapVisitor {
            type Value = UrlMap;

            // TODO: finish list
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("one of User, Owner, TODO")
            }

            fn visit_map<A: MapAccess<'a>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut urls = BTreeMap::new();
                // While there are entries remaining in the input, add them
                // into our map.
                while let Some((key, value)) = map.next_entry::<String, String>()? {
                    if !key.ends_with("s_url") {
                        continue;
                    }
                    urls.insert(key, value);
                }
                Ok(UrlMap { urls })
            }
        }

        d.deserialize_any(UrlMapVisitor)
    }
}

pub(crate) const fn true_fn() -> bool { true }
