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

use crate::RepoName;

/// All the commands the github bot can do.
#[derive(Clone, Debug)]
pub enum Command {
    Octocat(String),
    Create(String, Option<String>),
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
        println!("{}", msg);
        let send = |cmd| async {
            self.sender.send((room.room_id().clone(), cmd)).await.unwrap()
        };
        if let Some(msg) = msg.strip_prefix("!github ") {
            let cmds = split_cmd_words(msg).unwrap_or_default();
            println!("{:?}", cmds);

            match cmds.iter().map(|s| s.as_str()).collect::<Vec<&str>>().as_slice() {
                ["party", msg] => send(Command::Octocat((*msg).to_owned())).await,
                ["create", "issue", title] => {
                    send(Command::Create((*title).to_owned(), None)).await
                }
                ["create", "issue", title, body] => {
                    send(Command::Create((*title).to_owned(), Some((*body).to_owned())))
                        .await
                }
                [] | [..] => {}
            }

            println!("message sent");
        } else {
            // TODO: check for expansions etc.
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

    async fn on_custom_event(&self, _: Room, msg: &matrix_sdk::CustomEvent<'_>) {
        println!("{:?}", msg)
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

pub async fn issue_request(
    github: &GithubClient,
    matrix: &MatrixClient,
    room: &RoomId,
    repo: &RepoName,
    title: &str,
    body: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    github
        .send_github_request(rest::issue::create_issue::Request {
            accept: None,
            owner: &repo.owner,
            repo: &repo.repo,
            title,
            body,
            labels: vec![],
            assignees: vec![],
            milestone: None,
        })
        .await?;
    let content = AnyMessageEventContent::RoomMessage(MessageEventContent::new(
        MessageType::Text(TextMessageEventContent::plain("Filed issue")),
    ));
    matrix.room_send(room, content, None).await?;
    Ok(())
}

fn split_cmd_words(cmd: &str) -> Result<Vec<String>, &'static str> {
    let mut out = vec![];
    let mut buff = String::new();

    let mut cmd = cmd.chars();

    let mut in_quotes = false;
    let mut quote_kind = None;
    while let Some(c) = cmd.next() {
        if c.is_whitespace() && !in_quotes {
            out.push(buff.to_owned());
            buff.clear();
            continue;
        }

        // are these a problem ?
        // `‘` | `’` | `“` | `”`
        match c {
            q @ '\'' | q @ '"' => {
                if in_quotes && quote_kind == Some(q) {
                    in_quotes = false;
                    quote_kind = None;
                } else if in_quotes && (quote_kind.is_none() || quote_kind != Some(q)) {
                    buff.push(q)
                } else {
                    in_quotes = true;
                    quote_kind = Some(q);
                }
            }
            '\\' => match cmd.next() {
                Some(c2) => buff.push(match c2 {
                    'a' => '\u{07}',
                    'b' => '\u{08}',
                    'v' => '\u{0B}',
                    'f' => '\u{0C}',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    'e' | 'E' => '\u{1B}',
                    '\\' => '\\',
                    '\'' => '\'',
                    '"' => '"',
                    '$' => '$',
                    '`' => '`',
                    ' ' => ' ',
                    _ => return Err("Invalid escape char"),
                }),
                None => return Err("Invalid escape char"),
            },
            c => buff.push(c),
        }
    }

    if in_quotes {
        return Err("Mismatched quotes oops");
    }

    if !buff.is_empty() {
        out.push(buff);
    }

    Ok(out)
}

#[test]
fn nothing_special() {
    assert_eq!(split_cmd_words("a b c d").unwrap(), ["a", "b", "c", "d"]);
}

#[test]
fn quoted_strings() {
    assert_eq!(split_cmd_words("a \"b b\" a").unwrap(), ["a", "b b", "a"]);
}

#[test]
fn escaped_double_quotes() {
    assert_eq!(split_cmd_words("a \"\\\"b\\\" c\" d").unwrap(), ["a", "\"b\" c", "d"]);
}

#[test]
fn escaped_single_quotes() {
    assert_eq!(split_cmd_words("a \"'b' c\" d").unwrap(), ["a", "'b' c", "d"]);
}

#[test]
fn escaped_spaces() {
    assert_eq!(split_cmd_words("a b\\ c d").unwrap(), ["a", "b c", "d"]);
}

#[test]
fn bad_double_quotes() {
    assert_eq!(split_cmd_words("a \"b c d e").unwrap_err(), "Mismatched quotes oops");
}

#[test]
fn bad_single_quotes() {
    assert_eq!(split_cmd_words("a 'b c d e").unwrap_err(), "Mismatched quotes oops");
}

#[test]
fn bad_quotes() {
    assert_eq!(split_cmd_words("one '\"\"\"").unwrap_err(), "Mismatched quotes oops");
}

#[test]
fn trailing_whitespace() {
    assert_eq!(split_cmd_words("a b c d ").unwrap(), ["a", "b", "c", "d"]);
}

#[test]
fn percent_signs() {
    assert_eq!(split_cmd_words("abc '%foo bar%'").unwrap(), ["abc", "%foo bar%"]);
}
