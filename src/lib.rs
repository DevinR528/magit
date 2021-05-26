#![allow(unused)]

use rocket::{
    catchers,
    figment::{
        providers::{Env, Format, Toml},
        Figment,
    },
    routes,
};
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

// Hack so github-derive can use import magit inside of magit.
extern crate self as magit;

pub mod api;
pub mod from_data;
pub mod routes;
pub mod strfmt;

use api::EventKind;

#[derive(Clone, Debug, Deserialize)]
pub struct GithubConfig {
    repos: Vec<String>,
    events: Vec<EventKind>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    secret_key: String,
    github: GithubConfig,
}

#[allow(unused)]
pub struct Store {
    config: Config,
    to_matrix: Sender<String>,
}

pub fn app(to_matrix: Sender<String>) -> rocket::Rocket<rocket::Build> {
    std::env::set_var("GITHUB_CONFIG", "./github.toml");

    let raw_config = Figment::from(rocket::Config::release_default())
        .merge(
            Toml::file(Env::var("GITHUB_CONFIG").expect(
                "The GITHUB_CONFIG env var needs to be set. Example: /etc/github.toml",
            ))
            .nested(),
        )
        .merge(Env::prefixed("GITHUB_").global());

    let config: Config = raw_config
        .extract()
        .expect("It looks like your config is invalid. Please take a look at the error");

    std::env::set_var("__GITHUB_WEBHOOK_SECRET", &config.secret_key);
    let store = Store { config, to_matrix };

    rocket::custom(raw_config)
        .manage(store)
        .mount("/", routes![routes::index])
        .register("/", catchers![not_found])
}

#[rocket::catch(404)]
fn not_found(r: &rocket::Request<'_>) -> String {
    println!("{:?}", r);
    println!("{:?}", r.uri());
    "not found".to_string()
}
