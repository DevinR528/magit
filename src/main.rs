use gitty_hub::GithubClient;
use magit::{
    cmd_request::{octocat_request, Command, GithubBot},
    parse_config, routes, Store,
};
use matrix_sdk::{
    self,
    events::{
        room::message::{MessageEventContent, MessageType, NoticeMessageEventContent},
        AnyMessageEventContent,
    },
    Client as MatrixClient, ClientConfig, SyncSettings,
};
use ruma::RoomId;
use tokio::{
    sync::mpsc::{channel, Sender},
    task,
    time::{sleep, Duration},
};
use url::Url;

/// Used to increase the sleep duration between checking for github webhook messages.
const BACKOFF: usize = 20;

async fn login_and_sync(
    homeserver_url: &str,
    username: &str,
    password: &str,
    sender: Sender<(RoomId, Command)>,
) -> Result<MatrixClient, matrix_sdk::Error> {
    // the location for `SledStore` to save files to
    let mut home = dirs::home_dir().expect("no home directory found");
    home.push(".github_bot");

    let client_config = ClientConfig::new().store_path(home);

    println!("{} {} {}", homeserver_url, username, password);

    let homeserver_url =
        Url::parse(homeserver_url).expect("Couldn't parse the homeserver URL");
    // create a new Client with the given homeserver url and config
    let client = MatrixClient::new_with_config(homeserver_url, client_config).unwrap();

    client.login(username, password, Some("github bot"), Some("github bot")).await?;

    println!("logged in as {}", username);

    // An initial sync to set up state so our bot doesn't respond to old
    // messages. If the `StateStore` finds saved state in the location given the
    // initial sync will be skipped in favor of loading state from the store
    client.sync_once(SyncSettings::default()).await.unwrap();
    // add our CommandBot to be notified of incoming messages, we do this after the
    // initial sync to avoid responding to messages before the bot was running.
    client
        .set_event_handler(Box::new(GithubBot::new(sender, username.to_string())))
        .await;

    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (to_matrix, mut from_gh) = channel(1024);
    let (to_gh, mut from_matrix) = channel(1024);

    let (raw_config, config) = parse_config();

    let client = login_and_sync(
        &config.github.homeserver,
        &config.github.user_name,
        &config.github.password,
        to_gh,
    )
    .await?;
    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());

    let store = Store { config, to_matrix };

    let matrix_client = client.clone();
    let github_client = GithubClient::new(store.config.secret_key.clone())?;
    tokio::spawn(async move { client.sync(settings).await });
    tokio::spawn(async move {
        let mut durations = vec![];
        for i in 0..BACKOFF {
            durations.push(Duration::from_millis((50 * i) as u64));
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
                Some((room, msg)) = from_gh.recv() => {
                    println!("{}", msg);
                    let content = AnyMessageEventContent::RoomMessage(MessageEventContent::new(
                        MessageType::Notice(NoticeMessageEventContent::markdown(&msg.replace("\n", "<br>"))),
                    ));
                    matrix_client.room_send(&room, content, None).await.unwrap();
                }
                Some((room, cmd)) = from_matrix.recv() => {
                    println!("{:?}", cmd);
                    match cmd {
                        Command::Octocat(msg) => octocat_request(&github_client, &matrix_client, &room, &msg).await.unwrap(),
                    }
                }
            }
        }
    });

    rocket::custom(raw_config)
        .manage(store)
        .mount("/", rocket::routes![routes::index])
        .register("/", rocket::catchers![routes::not_found])
        .launch()
        .await
        .map_err(|e| e.into())
}
