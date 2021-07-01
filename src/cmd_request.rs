use gitty_hub::{api::rest, GithubClient};
use matrix_sdk::{
    self, async_trait,
    events::{
        room::{
            member::MemberEventContent,
            message::{MessageEventContent, MessageType, TextMessageEventContent},
        },
        AnyMessageEventContent, StrippedStateEvent, SyncMessageEvent,
    },
    room::{Joined, Room},
    Client as MatrixClient, EventHandler,
};
use ruma::RoomId;
use tokio::{
    sync::mpsc::Sender,
    time::{sleep, Duration},
};

/// All the commands the github bot can do.
#[derive(Clone, Debug)]
pub enum Command {
    Octocat(String),
}

#[allow(unused)]
pub struct GithubBot {
    sender: Sender<(RoomId, Command)>,
    user: String,
}

impl GithubBot {
    pub fn new(sender: Sender<(RoomId, Command)>, user: String) -> Self {
        Self { sender, user }
    }

    async fn handle_incoming_message(&self, room: Joined, msg: &str) {
        if msg.starts_with("!github") {
            match msg.split_whitespace().skip(1).collect::<Vec<_>>().as_slice() {
                [""] => {}
                [] | [..] => {}
            }
            self.sender
                .send((
                    room.room_id().clone(),
                    Command::Octocat(msg.trim_start_matches("!party ").to_owned()),
                ))
                .await
                .unwrap();
            println!("message sent");
        } else {
            // TODO: check for exansions etc.
        }
    }
}

#[async_trait]
impl EventHandler for GithubBot {
    /// Respond to all github commands.
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
            self.handle_incoming_message(room, msg_body).await;
        }
    }

    /// This bot will auto join any room it is invited to.
    async fn on_stripped_state_member(
        &self,
        room: Room,
        room_member: &StrippedStateEvent<MemberEventContent>,
        _: Option<MemberEventContent>,
    ) {
        // TODO: this is a bit weak maybe just parse the username as a UserId
        // in `login_and_sync`?
        // If the invite isn't for use don't join
        if !room_member.state_key.contains(&self.user) {
            return;
        }

        if let Room::Invited(room) = room {
            println!("Autojoining room {}", room.room_id());
            let mut delay = 2;

            while let Err(err) = room.accept_invitation().await {
                // retry autojoin due to synapse sending invites, before the
                // invited user can join for more information see
                // https://github.com/matrix-org/synapse/issues/4345
                eprintln!(
                    "Failed to join room {} ({:?}), retrying in {}s",
                    room.room_id(),
                    err,
                    delay
                );

                sleep(Duration::from_secs(delay)).await;
                delay *= 2;

                if delay > 3600 {
                    eprintln!("Can't join room {} ({:?})", room.room_id(), err);
                    return;
                }
            }
            println!("Successfully joined room {}", room.room_id());
        }
    }
}

pub async fn octocat_request(
    github: &GithubClient,
    matrix: &MatrixClient,
    room: &RoomId,
    msg: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let res = github
        .send_github_request(rest::octocat::Request { accept: None, s: msg })
        .await?;
    let content = AnyMessageEventContent::RoomMessage(MessageEventContent::new(
        MessageType::Text(TextMessageEventContent::markdown(&res.cat)),
    ));
    matrix.room_send(room, content, None).await?;
    Ok(())
}
