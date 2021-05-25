use std::fmt;

use matrix_sdk::async_trait;
use reqwest::{Client, Error as ReqError, Method};
use thiserror::Error;

pub mod repository;

#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON deserialization failed: {0}")]
    Client(#[from] ReqError),
    #[error("JSON deserialization failed: {0}")]
    Request(#[from] GithubFailure),
}

#[derive(Debug, Error)]
pub enum GithubFailure {
    #[error("failed")]
    Fail,
}

pub struct MetaData {
    pub description: &'static str,
    pub method: Method,
    pub path: &'static str,
    pub name: &'static str,
    pub authentication: bool,
}

pub trait GithubRequest {
    const METADATA: MetaData;
    type Response: GithubResponse;

    fn to_request(self) -> Result<reqwest::Request, GithubFailure>;
}

pub trait GithubResponse: Sized {
    fn from_response(resp: reqwest::Response) -> Result<Self, GithubFailure>;
}
pub struct GithubClient {
    tkn: String,
    cli: Client,
}

impl GithubClient {
    pub fn new(tkn: &str) -> Self { Self { tkn: tkn.to_owned(), cli: Client::new() } }

    pub async fn send_github_request<G: GithubRequest>(
        &self,
        req: G,
    ) -> Result<G::Response, Error> {
        let res = self.cli.execute(req.to_request()?).await?;
        G::Response::from_response(res).map_err(|e| e.into())
    }
}
