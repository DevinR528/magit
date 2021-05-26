use std::fmt;

use matrix_sdk::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, ClientBuilder, Error as ReqError, Method, RequestBuilder, StatusCode,
};
use ruma::{serde::StringEnum, uint, UInt};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod action;
pub mod issue;
pub mod pull_request;
pub mod repository;

/// Specifies the types of repositories you want returned.
#[derive(Clone, Debug, StringEnum)]
#[ruma_enum(rename_all = "lowercase")]
pub enum Type {
    /// Return all repository types the requester has access to.
    All,

    /// Return only public repositories.
    Public,

    /// Return only private repositories.
    Private,

    /// Return only repositories that are forks.
    Forks,

    /// Return only soruce repositories.
    Sources,

    /// Return only repositories where requester is a member.
    Member,

    /// Internal repositories.
    ///
    /// Only supported when a Github app calls this endpoint.
    Internal,

    #[doc(hidden)]
    _Custom(String),
}

impl Default for Type {
    fn default() -> Self { Self::All }
}

/// How the returned repositories are sorted.
#[derive(Clone, Debug, StringEnum)]
#[ruma_enum(rename_all = "lowercase")]
pub enum Direction {
    /// Ascending order, smallest to largest.
    Asc,

    /// Descending order, largest to smallest.
    Desc,

    #[doc(hidden)]
    _Custom(String),
}

/// How the returned repositories are sorted.
#[derive(Clone, Debug, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
pub enum Sort {
    /// Return all repository types the requester has access to.
    Created,

    /// Return only public repositories.
    Updated,

    /// Return only private repositories.
    Pushed,

    /// Return only repositories that are forks.
    FullName,

    #[doc(hidden)]
    _Custom(String),
}

/// Filter issues by state.
#[derive(Clone, Debug, StringEnum)]
#[ruma_enum(rename_all = "lowercase")]
pub enum StateQuery {
    /// Return only open issues.
    Open,

    /// Return only closed issues.
    Close,

    /// Return all issues.
    All,

    #[doc(hidden)]
    _Custom(String),
}

impl Default for StateQuery {
    fn default() -> Self { Self::Open }
}

#[derive(Clone, Debug)]
pub enum MilestoneQuery {
    /// Refers to a milestone by its number.
    Int(UInt),

    /// All issues are returned.
    Wildcard,

    /// Issues with out milestones are returned.
    None,
}

impl Serialize for MilestoneQuery {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Self::Int(int) => int.serialize(ser),
            Self::Wildcard => "*".serialize(ser),
            Self::None => "none".serialize(ser),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AssigneeQuery {
    /// All issues are returned.
    Wildcard,

    /// Issues with out assigned users are returned.
    None,
}

impl Serialize for AssigneeQuery {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Self::Wildcard => "*".serialize(ser),
            Self::None => "none".serialize(ser),
        }
    }
}

pub fn comma_list<S, T>(list: &[T], ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
    T: Serialize,
{
    list.serialize(ser)
}

pub fn opt_default<S, T>(opt: &Option<T>, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
    T: Serialize + Default,
{
    if let Some(it) = opt { it.serialize(ser) } else { T::default().serialize(ser) }
}

pub fn sort<S>(sort: &Option<Sort>, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    if let Some(sort) = sort {
        sort.serialize(ser)
    } else {
        let sort = Sort::Created;
        sort.serialize(ser)
    }
}

pub fn direction<S>(dir: &Option<Direction>, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    if let Some(dir) = dir {
        dir.serialize(ser)
    } else {
        let dir = Direction::Desc;
        dir.serialize(ser)
    }
}

pub fn per_page<S>(page: &Option<UInt>, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    use serde::ser::Error;
    if let Some(page) = page {
        if page > &uint!(100) {
            return Err(S::Error::custom("per_page must be <= 100"));
        }
        page.serialize(ser)
    } else {
        let page = uint!(30);
        page.serialize(ser)
    }
}

pub fn page<S>(page: &Option<UInt>, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    if let Some(page) = page {
        page.serialize(ser)
    } else {
        let page = uint!(1);
        page.serialize(ser)
    }
}

/// Enables preview notices.
///
/// See https://docs.github.com/en/rest/reference/repos#get-a-repository-preview-notices.
#[derive(Clone, Copy, Debug, serde::Serialize)]
pub struct ApplicationV3Json;

impl fmt::Display for ApplicationV3Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "application/vnd.github.v3+json".fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{issue, repository, Direction, Sort, Type};
    use crate::api::{
        rest::{ApplicationV3Json, MilestoneQuery, StateQuery},
        Dt, GithubClient,
    };

    #[tokio::test]
    #[ignore = "integration test"]
    async fn get_repository_integration() {
        let cli = GithubClient::new(Some("foo".to_owned())).unwrap();
        let res = cli
            .send_github_request(repository::get_repository::Request {
                accept: None,
                repo: "magit",
                owner: "DevinR528",
                r#type: Type::All,
            })
            .await
            .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    #[ignore = "integration test"]
    async fn list_org_repositories() {
        let cli = GithubClient::new(Some("foo".to_owned())).unwrap();
        let res = cli
            .send_github_request(repository::list_org_repositories::Request {
                accept: None,
                org: "ruma",
                sort: None,
                direction: None,
                page: None,
                per_page: None,
                r#type: Type::All,
            })
            .await
            .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    #[ignore = "integration test"]
    async fn get_issue() {
        let cli = GithubClient::new(Some("foo".to_owned())).unwrap();
        let res = cli
            .send_github_request(issue::get_issue::Request {
                accept: None,
                owner: "DevinR528",
                repo: "rumatui",
                issue_number: ruma::uint!(1),
            })
            .await
            .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    #[ignore = "integration test"]
    async fn create_issue() {
        let cli = GithubClient::new(Some(
            include_str!("/home/devin/Desktop/github/tkns.txt").trim().to_owned(),
        ))
        .unwrap();
        let res = cli
            .send_github_request(issue::create_issue::Request {
                accept: Some(ApplicationV3Json),
                owner: "DevinR528",
                repo: "magit",
                title: "Test issue to be closed by bot",
                body: Some("Test issue to be closed by bot"),
                milestone: None,
                assignees: vec![],
                labels: vec![],
            })
            .await
            .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    #[ignore = "integration test"]
    async fn list_issues() {
        use std::convert::TryFrom;

        let cli = GithubClient::new(Some(
            include_str!("/home/devin/Desktop/github/tkns.txt").trim().to_owned(),
        ))
        .unwrap();
        let res = cli
            .send_github_request(issue::list_repo_issues::Request {
                accept: Some(ApplicationV3Json),
                owner: "DevinR528",
                repo: "magit",
                creator: None,
                labels: vec!["foo", "bar", "more"],
                mentioned: None,
                since: Some(Dt::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(
                        i64::try_from(
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis(),
                        )
                        .unwrap(),
                        0,
                    )
                    .unwrap(),
                    chrono::Utc,
                )),
                sort: None,
                direction: None,
                page: None,
                per_page: None,
                milestone: MilestoneQuery::None,
                state: None,
            })
            .await
            .unwrap();
        println!("{:?}", res);
    }
}
