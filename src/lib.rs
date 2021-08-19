use std::{collections::BTreeMap, sync::Arc};

use gitty_hub::{api, api::EventKind};
use rocket::figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use ruma::RoomId;
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

pub mod cmd_request;
pub mod from_data;
pub mod routes;
pub mod strfmt;

#[derive(Clone, Debug, Deserialize)]
pub struct RepoName {
    pub owner: String,
    pub repo: String,
}

impl RepoName {
    pub fn as_full_name(&self) -> String { format!("{}/{}", self.owner, self.repo) }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RepoRoomMap {
    #[serde(deserialize_with = "repo_owner")]
    pub repo_name: RepoName,
    pub room: RoomId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GithubConfig {
    pub repos: Vec<RepoRoomMap>,
    pub events: Vec<EventKind>,
    pub homeserver: String,
    pub user_name: String,
    pub password: String,
    pub user_token: Option<String>,
    #[serde(flatten)]
    pub format_strings: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub secret_key: Option<String>,
    pub github: GithubConfig,
}

impl Config {
    #[doc(hidden)]
    pub fn debug() -> Self {
        Self {
            secret_key: Some("test".to_owned()),
            github: GithubConfig {
                repos: vec![],
                events: vec![],
                homeserver: "foobar.com".to_owned(),
                user_name: "foobar.com".to_owned(),
                password: "foobar.com".to_owned(),
                user_token: None,
                format_strings: BTreeMap::default(),
            },
        }
    }
}

fn repo_owner<'de, D>(deser: D) -> Result<RepoName, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    let string = String::deserialize(deser)?;
    let mut iter = string.split('/');

    let owner =
        iter.next().ok_or_else(|| D::Error::missing_field("owner not found"))?.to_owned();
    let repo =
        iter.next().ok_or_else(|| D::Error::missing_field("repo not found"))?.to_owned();

    Ok(RepoName { owner, repo })
}

#[allow(unused)]
pub struct Store {
    pub config: Arc<Config>,
    pub to_matrix: Sender<(RoomId, String)>,
}

pub fn parse_config() -> (Figment, Config) {
    std::env::set_var("GITHUB_CONFIG", "./github.toml");
    let raw_config = Figment::from(rocket::Config::release_default())
        .merge(
            Toml::file(Env::var("GITHUB_CONFIG").expect(
                "the GITHUB_CONFIG env var needs to be set Example: /etc/github.toml",
            ))
            .nested(),
        )
        .merge(Env::prefixed("GITHUB_").global());

    let config: Config =
        raw_config.extract().expect("it looks like your config is invalid");

    std::env::set_var(
        "__GITHUB_WEBHOOK_SECRET",
        &config.secret_key.as_deref().unwrap_or(""),
    );

    (raw_config, config)
}
