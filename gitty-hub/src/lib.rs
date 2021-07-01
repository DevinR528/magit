use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, ClientBuilder, Error as ReqError, Method, RequestBuilder, StatusCode,
};
use thiserror::Error;

// Hack so github-derive can use import gitty_hub inside of gitty_hub.
extern crate self as gitty_hub;

pub mod api;
pub mod utils;

pub const USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub const BASE_URL: &str = "https://api.github.com";

#[derive(Debug, Error)]
pub enum Error {
    #[error("A client request has failed: {0}")]
    Client(#[from] ReqError),

    #[error("Invalid header value: {0}")]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("JSON deserialization failed: {0}")]
    Request(#[from] GithubFailure),

    #[error("JSON deserialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("JSON deserialization failed: {0}")]
    JsonPath(#[from] serde_path_to_error::Error<serde_json::Error>),
}

#[derive(Debug, Error)]
pub enum GithubFailure {
    #[error("failed")]
    Fail,

    #[error("Request failed: {} CODE: {}",  .0.as_str(), .0.as_u16())]
    StatusError(StatusCode),
}

pub fn from_status(status: StatusCode) -> Result<(), Error> {
    Err(Error::Request(match status {
        s if s.is_success() => return Ok(()),
        s => GithubFailure::StatusError(s),
    }))
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

    fn to_request(self, client: &GithubClient) -> Result<reqwest::Request, Error>;
}

#[async_trait]
pub trait GithubResponse: Sized {
    async fn from_response(resp: reqwest::Response) -> Result<Self, Error>;
}
pub struct GithubClient {
    cli: Client,
    tkn: Option<String>,
}

impl GithubClient {
    pub fn new(tkn: Option<String>) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(Self {
            cli: Client::builder()
                .user_agent(USER_AGENT)
                .default_headers(headers)
                .build()?,
            tkn,
        })
    }

    pub fn request_builder(&self, method: Method, url: &str) -> RequestBuilder {
        self.cli.request(method, url)
    }

    pub async fn send_github_request<G: GithubRequest>(
        &self,
        req: G,
    ) -> Result<G::Response, Error> {
        let res = self.cli.execute(req.to_request(self)?).await?;
        G::Response::from_response(res).await
    }
}
