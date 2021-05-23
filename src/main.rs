use magit::app;
use matrix_sdk::{
    self, async_trait,
    events::{
        room::message::{MessageEventContent, MessageType, TextMessageEventContent},
        AnyMessageEventContent, SyncMessageEvent,
    },
    room::Room,
    Client, ClientConfig, EventHandler, SyncSettings,
};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task,
    time::{sleep, Duration},
};
use url::Url;

/// Used to increase the sleep duration between checking for github webhook messages.
const BACKOFF: usize = 20;

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
) -> Result<Client, matrix_sdk::Error> {
    // the location for `JsonStore` to save files to
    let mut home = dirs::home_dir().expect("no home directory found");
    home.push("github_bot");

    let client_config = ClientConfig::new().store_path(home);

    let homeserver_url =
        Url::parse(homeserver_url).expect("Couldn't parse the homeserver URL");
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
    let (to_matrix, mut from_gh) = channel(1024);
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
    tokio::spawn(async move {
        let mut durations = vec![];
        for i in 0..BACKOFF {
            let time = Duration::from_millis((50 * i) as u64);
            durations.push(time);
        }
        let mut next_sleep = 1;
        loop {
            let idx = next_sleep % BACKOFF;
            let sleep = sleep(durations[idx]);
            tokio::pin!(sleep);

            tokio::select! {
                _ = &mut sleep => {
                    next_sleep += 1;
                    task::yield_now().await;
                }
                Some(msg) = from_gh.recv() => {
                    println!("{}", msg);
                    // sender.rooms();
                }
            }
        }
    });

    app(to_matrix).launch().await?;
    Ok(())
}
