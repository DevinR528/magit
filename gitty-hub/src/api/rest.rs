use std::fmt;

use github_derive::StringEnum;
use js_int::{uint, UInt};
use serde::Serialize;

pub mod actions;
pub mod issue;
pub mod octocat;
pub mod pull_request;
pub mod repository;

/// Specifies the types of repositories you want returned.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
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
}

impl Default for Type {
    fn default() -> Self { Self::All }
}

/// How the returned repositories are sorted.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
pub enum Direction {
    /// Ascending order, smallest to largest.
    Asc,

    /// Descending order, largest to smallest.
    Desc,
}

/// How the returned repositories are sorted.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "snake_case")]
pub enum Sort {
    /// Return all repository types the requester has access to.
    Created,

    /// Return only public repositories.
    Updated,

    /// Return only private repositories.
    Pushed,

    /// Return only repositories that are forks.
    FullName,
}

/// Filter issues by state.
#[derive(Clone, Debug, StringEnum)]
#[github_enum(rename_all = "lowercase")]
pub enum StateQuery {
    /// Return only open issues.
    Open,

    /// Return only closed issues.
    Close,

    /// Return all issues.
    All,
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
#[derive(Clone, Copy, Debug, Serialize)]
pub struct ApplicationV3Json;

impl fmt::Display for ApplicationV3Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "application/vnd.github.v3+json".fmt(f)
    }
}

/// Enables preview endpoint update_pull_request.
///
/// See https://docs.github.com/en/rest/reference/pulls#update-a-pull-request-branch-preview-notices
#[derive(Clone, Copy, Debug, Serialize)]
pub struct ApplicationLydian;

impl fmt::Display for ApplicationLydian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "application/vnd.github.lydian-preview+json".fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use js_int::uint;

    use super::{actions, issue, octocat, repository, Direction, Sort, Type};
    use crate::{
        api::{
            rest::{ApplicationV3Json, MilestoneQuery, StateQuery},
            Dt,
        },
        GithubClient,
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
                issue_number: uint!(1),
            })
            .await
            .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    #[ignore = "integration test"]
    async fn create_issue() {
        let cli = GithubClient::new(Some("token".to_owned())).unwrap();
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

        let cli = GithubClient::new(Some("token".to_owned())).unwrap();
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

    #[tokio::test]
    #[ignore = "integration test"]
    async fn octocat() {
        let cli = GithubClient::new(None).unwrap();
        let res = cli
            .send_github_request(octocat::Request { accept: None, s: "Hello All" })
            .await
            .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    #[ignore = "integration test"]
    async fn download_job_log() {
        let cli = GithubClient::new(Some("foo".to_owned())).unwrap();
        let res = cli
            .send_github_request(actions::download_job_log::Request {
                accept: None,
                owner: "DevinR528",
                repo: "magit",
                job_id: uint!(2_678_750_777),
            })
            .await
            .unwrap();
        println!("{}", res.logs);
    }

    #[tokio::test]
    #[ignore = "integration test"]
    async fn download_run_log() {
        let cli = GithubClient::new(Some("foo".to_owned())).unwrap();
        let res = cli
            .send_github_request(actions::download_run_log::Request {
                accept: None,
                owner: "DevinR528",
                repo: "magit",
                run_id: uint!(2_678_750_777),
            })
            .await
            .unwrap();
        println!("{}", res.logs);
    }

    #[tokio::test]
    #[ignore = "integration test"]
    async fn list_runs() {
        let cli = GithubClient::new(Some("foo".to_owned())).unwrap();
        let res = cli
            .send_github_request(actions::list_runs::Request {
                accept: None,
                owner: "DevinR528",
                repo: "magit",
                workflow_id: "stable.yml",
                actor: None,
                branch: None,
                event: None,
                per_page: None,
                page: None,
            })
            .await
            .unwrap();
        println!("{:?}", res);
    }
}
