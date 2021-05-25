use std::borrow::Cow;

use github_derive::github_rest_api;
use reqwest::{header::HeaderMap, Method};

use crate::api::common::Repo;

// github_rest_api! {
//     metadata: {
//         description: "",
//         method: GET,
//         path: "/repos/:user/:repo",
//         name: "get_repository",
//         authentication: true,
//     }

//     request: {
//         #[github(path)]
//         pub user: &'a str,

//         #[github(path)]
//         pub repo: &'a str,

//         #[github(header = ACCEPT)]
//         pub accept: Option<ApplicationV3Json>,
//     }

//     response: {
//         #[serde(flatten, borrow)]
//         pub repository: IncomingRepo,
//     }
// }

#[derive(Clone, Debug, serde::Deserialize, github_derive::Incoming)]
pub struct Other<'a> {
    pub fielda: &'a str,
    pub fieldb: Option<&'a str>,
}

#[derive(Clone, Debug, serde::Deserialize, github_derive::Incoming)]
pub struct Test<'a> {
    pub fielda: &'a str,
    pub fieldb: Option<&'a str>,
    pub fieldc: Other<'a>,
    #[serde(borrow)]
    pub fieldd: Cow<'a, str>,
}
/// Enables preview notices.
///
/// See https://docs.github.com/en/rest/reference/repos#get-a-repository-preview-notices.
#[derive(Clone, Copy, Debug, serde::Serialize)]
pub struct ApplicationV3Json;

fn f() {
    format_args!("{}", 1);
}
