use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use gitty_hub::GithubClient;
use magit::{
    cmd_request::{issue_request, octocat_request, Command, GithubBot},
    parse_config, routes, Store,
};
use matrix_sdk::{
    self,
    events::{
        room::message::{MessageEventContent, MessageType, NoticeMessageEventContent},
        AnyMessageEventContent, AnySyncMessageEvent, AnySyncRoomEvent, AnyToDeviceEvent,
    },
    verification::{SasVerification, Verification},
    Client as MatrixClient, ClientConfig, SyncSettings,
};
use ruma::{RoomId, UserId};
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

    client.login(username, password, None, None).await?;

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

async fn wait_for_confirmation(client: MatrixClient, sas: SasVerification) {
    println!("Does the emoji match: {:?}", sas.emoji());

    sas.confirm().await.unwrap();

    if sas.is_done() {
        print_result(&sas);
        print_devices(sas.other_device().user_id(), &client).await;
    }
}

fn print_result(sas: &SasVerification) {
    let device = sas.other_device();

    println!(
        "Successfully verified device {} {} {:?}",
        device.user_id(),
        device.device_id(),
        device.local_trust_state()
    );
}

async fn print_devices(user_id: &UserId, client: &MatrixClient) {
    println!("Devices of user {}", user_id);

    for device in client.get_user_devices(user_id).await.unwrap().devices() {
        println!(
            "   {:<10} {:<30} {:<}",
            device.device_id(),
            device.display_name().as_deref().unwrap_or_default(),
            device.is_trusted()
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

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
    let github_client = GithubClient::new(store.config.github.user_token.clone())?;

    tokio::spawn(async move {
        let client = &client;
        let initial_sync = Arc::new(AtomicBool::from(true));
        let initial_ref = &initial_sync;
        client
            .sync_with_callback(settings, |res| async move {
                let client = &client;
                let initial = &initial_ref;

                for event in
                    res.to_device.events.iter().filter_map(|e| e.deserialize().ok())
                {
                    match event {
                        AnyToDeviceEvent::KeyVerificationStart(e) => {
                            if let Some(Verification::SasV1(sas)) = client
                                .get_verification(&e.sender, &e.content.transaction_id)
                                .await
                            {
                                println!(
                                    "Starting verification with {} {}",
                                    &sas.other_device().user_id(),
                                    &sas.other_device().device_id()
                                );
                                print_devices(&e.sender, client).await;
                                sas.accept().await.unwrap();
                            }
                        }

                        AnyToDeviceEvent::KeyVerificationKey(e) => {
                            if let Some(Verification::SasV1(sas)) = client
                                .get_verification(&e.sender, &e.content.transaction_id)
                                .await
                            {
                                tokio::spawn(wait_for_confirmation(
                                    (*client).clone(),
                                    sas,
                                ));
                            }
                        }

                        AnyToDeviceEvent::KeyVerificationMac(e) => {
                            if let Some(Verification::SasV1(sas)) = client
                                .get_verification(&e.sender, &e.content.transaction_id)
                                .await
                            {
                                if sas.is_done() {
                                    print_result(&sas);
                                    print_devices(&e.sender, client).await;
                                }
                            }
                        }

                        _ => (),
                    }
                }

                if !initial.load(Ordering::SeqCst) {
                    for (_room_id, room_info) in res.rooms.join {
                        for event in room_info
                            .timeline
                            .events
                            .iter()
                            .filter_map(|e| e.event.deserialize().ok())
                        {
                            if let AnySyncRoomEvent::Message(event) = event {
                                match event {
                                    AnySyncMessageEvent::RoomMessage(m) => {
                                        if let MessageType::VerificationRequest(_) =
                                            &m.content.msgtype
                                        {
                                            let request = client
                                                .get_verification_request(
                                                    &m.sender,
                                                    &m.event_id,
                                                )
                                                .await
                                                .expect("Request object wasn't created");

                                            request.accept().await.expect(
                                                "Can't accept verification request",
                                            );
                                        }
                                    }
                                    AnySyncMessageEvent::KeyVerificationKey(e) => {
                                        if let Some(Verification::SasV1(sas)) = client
                                            .get_verification(
                                                &e.sender,
                                                e.content.relates_to.event_id.as_str(),
                                            )
                                            .await
                                        {
                                            tokio::spawn(wait_for_confirmation(
                                                (*client).clone(),
                                                sas,
                                            ));
                                        }
                                    }
                                    AnySyncMessageEvent::KeyVerificationMac(e) => {
                                        if let Some(Verification::SasV1(sas)) = client
                                            .get_verification(
                                                &e.sender,
                                                e.content.relates_to.event_id.as_str(),
                                            )
                                            .await
                                        {
                                            if sas.is_done() {
                                                print_result(&sas);
                                                print_devices(&e.sender, client).await;
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }

                initial.store(false, Ordering::SeqCst);
                matrix_sdk::LoopCtrl::Continue
            })
            .await;
    });
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
                        Command::Octocat(msg) => {
                            octocat_request(
                                &github_client,
                                &matrix_client,
                                &room,
                                &msg
                            ).await.unwrap();
                        },
                        Command::Create(title, body) => {
                            issue_request(
                                &github_client,
                                &matrix_client,
                                &room,
                                &title,
                                body.as_deref()
                            ).await.unwrap();
                        },
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
