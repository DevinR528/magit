use matrix_sdk::{
    self, async_trait,
    events::{
        room::message::{MessageEventContent, MessageType, TextMessageEventContent},
        AnyMessageEventContent, SyncMessageEvent,
    },
    room::Room,
    Client, ClientConfig, EventHandler, SyncSettings,
};
use rocket::{
    catchers,
    figment::{
        providers::{Env, Format, Toml},
        Figment,
    },
    routes,
};
use serde::Deserialize;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use url::Url;

mod api;
mod response;
mod routes;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    secret_key: String,
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
    std::env::set_var("GITHUB_WEBHOOK_SECRET", &config.secret_key);
    let store = Store { config, to_matrix };
    rocket::custom(raw_config)
        .manage(store)
        .mount("/", routes![routes::index])
        .register("/", catchers![not_found])
}

#[allow(unused)]
struct CommandBot {
    listener: Receiver<String>,
    sender: Sender<String>,
}

impl CommandBot {
    pub fn new(listener: Receiver<String>, sender: Sender<String>) -> Self {
        Self { listener, sender }
    }
}

#[async_trait]
impl EventHandler for CommandBot {
    async fn on_room_message(
        &self,
        room: Room,
        event: &SyncMessageEvent<MessageEventContent>,
    ) {
        if let Room::Joined(room) = room {
            let msg_body = if let SyncMessageEvent {
                content:
                    MessageEventContent {
                        msgtype:
                            MessageType::Text(TextMessageEventContent {
                                body: msg_body, ..
                            }),
                        ..
                    },
                ..
            } = event
            {
                msg_body
            } else {
                return;
            };

            if msg_body.contains("!party") {
                let content = AnyMessageEventContent::RoomMessage(
                    MessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉"),
                );

                println!("sending");

                // send our message to the room we found the "!party" command in
                // the last parameter is an optional Uuid which we don't care about.
                room.send(content, None).await.unwrap();

                println!("message sent");
            }
        }
    }
}

#[allow(unused)]
async fn login_and_sync(
    homeserver_url: &str,
    username: String,
    password: String,
    listener: Receiver<String>,
    sender: Sender<String>,
) -> Result<matrix_sdk::Client, matrix_sdk::Error> {
    // the location for `JsonStore` to save files to
    let mut home = dirs::home_dir().expect("no home directory found");
    home.push("github_bot");

    let client_config = ClientConfig::new().store_path(home);

    let homeserver_url =
        Url::parse(&homeserver_url).expect("Couldn't parse the homeserver URL");
    // create a new Client with the given homeserver url and config
    let client = Client::new_with_config(homeserver_url, client_config).unwrap();

    client.login(&username, &password, None, Some("github bot")).await?;

    println!("logged in as {}", username);

    // An initial sync to set up state and so our bot doesn't respond to old
    // messages. If the `StateStore` finds saved state in the location given the
    // initial sync will be skipped in favor of loading state from the store
    client.sync_once(SyncSettings::default()).await.unwrap();
    // add our CommandBot to be notified of incoming messages, we do this after the
    // initial sync to avoid responding to messages before the bot was running.
    client.set_event_handler(Box::new(CommandBot::new(listener, sender))).await;

    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[allow(unused)]
    let (to_matrix, from_gh) = channel(1024);
    // let (to_gh, mut from_matrix) = channel(1024);

    // let (homeserver_url, username, password) =
    //     match (env::args().nth(1), env::args().nth(2), env::args().nth(3)) {
    //         (Some(a), Some(b), Some(c)) => (a, b, c),
    //         _ => {
    //             eprintln!(
    //                 "Usage: {} <homeserver_url> <username> <password>",
    //                 env::args().next().unwrap()
    //             );
    //             exit(1)
    //         }
    //     };

    // let client =
    //     login_and_sync(&homeserver_url, username, password, from_gh, to_gh).await?;

    // let settings = SyncSettings::default().token(client.sync_token().await.unwrap());

    // let sender = client.clone();
    // tokio::spawn(async move { client.sync(settings).await });
    // tokio::spawn(async move {
    //     tokio::select! {
    //         Some(msg) = from_matrix.recv() => {
    //             sender.rooms();
    //         }
    //     }
    // });
    app(to_matrix).launch().await?;
    Ok(())
}

#[rocket::catch(404)]
fn not_found(r: &rocket::Request<'_>) -> String {
    println!("{:?}", r);
    println!("{:?}", r.uri());
    "not found".to_string()
}
