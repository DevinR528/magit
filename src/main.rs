use magit::app;
use ruma::{
    api::{
        client::{
            r0::{
                message::send_message_event::Request as MessageRequest,
                sync::sync_events::Request as SyncRequest,
            },
            Error as ApiError,
        },
        error::MatrixError,
        SendAccessToken,
    },
    client::{
        http_client::Isahc, Client, Error as ClientError, HttpClient, HttpClientExt,
    },
    events::{
        room::message::{MessageEventContent, MessageType, TextMessageEventContent},
        AnyMessageEventContent, AnySyncMessageEvent, AnySyncRoomEvent, SyncMessageEvent,
    },
    presence::PresenceState,
    uint, RoomId,
};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task,
    time::{sleep, Duration},
};

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

async fn on_room_message<C: HttpClientExt>(
    client: C,
    mut since: String,
    room_id: &RoomId,
) -> Result<(), ClientError<C::Error, ApiError>> {
    loop {
        let response = client
            .send_matrix_request(
                "",
                SendAccessToken::IfRequired(""),
                assign::assign!(SyncRequest::new(), {
                    filter: None,
                    since: Some(&since),
                    set_presence: &PresenceState::Online,
                    timeout: Some(Duration::from_millis(500)),
                }),
            )
            .await?;

        since = response.next_batch.clone();

        for event in response
            .rooms
            .join
            .get(room_id)
            .into_iter()
            .flat_map(|room| &room.timeline.events)
            .filter_map(|ev| ev.deserialize().ok())
        {
            let msg_body = if let AnySyncRoomEvent::Message(
                AnySyncMessageEvent::RoomMessage(SyncMessageEvent {
                    content:
                        MessageEventContent {
                            msgtype:
                                MessageType::Text(TextMessageEventContent {
                                    body: msg_body,
                                    ..
                                }),
                            ..
                        },
                    ..
                }),
            ) = event
            {
                msg_body
            } else {
                continue;
            };

            if msg_body.contains("!party") {
                let content = AnyMessageEventContent::RoomMessage(
                    MessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉"),
                );

                println!("sending");

                // send our message to the room we found the "!party" command in
                // the last parameter is an optional Uuid which we don't care about.
                client
                    .send_matrix_request(
                        "",
                        SendAccessToken::IfRequired(""),
                        MessageRequest::new(room_id, "", &content),
                    )
                    .await?;

                println!("message sent");
            }
        }
    }
    Ok(())
}

#[allow(unused)]
async fn login_and_sync(
    homeserver_url: &str,
    username: String,
    password: String,
    listener: Receiver<String>,
    sender: Sender<String>,
) -> Result<Client<Isahc>, ClientError<isahc::Error, ApiError>> {
    let client =
        Client::with_http_client(Isahc::new().unwrap(), homeserver_url.to_owned(), None);

    client.log_in(&username, &password, None, Some("github bot")).await?;

    println!("logged in as {}", username);

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
