use hmac::{Hmac, Mac, NewMac};
use rocket::{
    data,
    data::FromData,
    http::{ContentType, Status},
    outcome::Outcome,
    request::Request,
    Data,
};
use sha2::Sha256;
use tokio::io::AsyncReadExt;

use crate::api::{common::EventKind, GitHubEvent};

// TODO: accept: 'application/vnd.github.v3+json'
pub const X_GITHUB_EVENT: &str = "x-github-event";
pub const X_HUB_SIGNATURE: &str = "x-hub-signature-256";
pub const CONTENT_LEN: &str = "content-length";
pub const CONTENT_TYPE: &str = "content-type";

#[rocket::async_trait]
impl<'r> FromData<'r> for GitHubEvent<'r> {
    type Error = String;

    async fn from_data(
        request: &'r Request<'_>,
        data: Data,
    ) -> data::Outcome<GitHubEvent<'r>, Self::Error> {
        let keys = request.headers().get(X_HUB_SIGNATURE).collect::<Vec<_>>();
        let content_len = request.headers().get(CONTENT_LEN).collect::<Vec<_>>();
        let content_type = request.headers().get(CONTENT_TYPE).collect::<Vec<_>>();
        if content_type.len() != 1 {
            return Outcome::Failure((
                Status::BadRequest,
                "Multiple content length headers".to_owned(),
            ));
        }
        if content_len.len() != 1 {
            return Outcome::Failure((
                Status::BadRequest,
                "Multiple content length headers".to_owned(),
            ));
        }
        if keys.len() != 1 {
            return Outcome::Failure((
                Status::BadRequest,
                "Multiple signature keys headers".to_owned(),
            ));
        }

        let signature = keys[0];
        let content_len = if let Ok(content) = content_len[0].parse() {
            content
        } else {
            return Outcome::Failure((
                Status::InternalServerError,
                "Content length headers failed to parse".to_owned(),
            ));
        };

        let mut string = String::new();
        if data.open(content_len).read_to_string(&mut string).await.is_err() {
            return Outcome::Failure((
                Status::InternalServerError,
                "Content too large".to_owned(),
            ));
        }

        let secret = match std::env::var("__GITHUB_WEBHOOK_SECRET") {
            Ok(s) => s,
            Err(_) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    "No secret key found".to_owned(),
                ));
            }
        };

        if !validate(secret.as_str(), signature, &string) {
            return Outcome::Failure((
                Status::BadRequest,
                "Validation failed".to_owned(),
            ));
        }

        let keys = request.headers().get(X_GITHUB_EVENT).collect::<Vec<_>>();
        if keys.len() != 1 {
            return Outcome::Failure((
                Status::BadRequest,
                "No payload type specified in header".to_owned(),
            ));
        }

        let decoded = if Ok(ContentType::Form) == content_type[0].parse() {
            // Form data has a "payload=..." prefix so remove it
            rocket::http::uri::Uri::percent_decode_lossy(&string[8..]).to_string()
        } else {
            string
        };

        // We store `string` in request-local cache for long-lived borrows.
        let body = rocket::request::local_cache!(request, decoded);

        Outcome::Success(match keys[0].into() {
            EventKind::CheckSuite => {
                GitHubEvent::CheckSuite(
                    match serde_json::from_str(body).map_err(|e| e.to_string()) {
                        Ok(ev) => ev,
                        Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                    },
                )
            }
            EventKind::CheckRun => GitHubEvent::CheckRun(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::CommitComment => GitHubEvent::CommitComment(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Create => GitHubEvent::Create(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Delete => GitHubEvent::Delete(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Installation => GitHubEvent::Installation(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Issues => GitHubEvent::Issue(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::IssueComment => GitHubEvent::IssueComment(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Milestone => GitHubEvent::Milestone(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Ping => GitHubEvent::Ping(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::PullRequest => GitHubEvent::PullRequest(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::PullRequestReview => GitHubEvent::PullRequestReview(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::PullRequestReviewComment => GitHubEvent::PullRequestReviewComment(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Push => GitHubEvent::Push(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Release => GitHubEvent::Release(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Star => GitHubEvent::Star(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Star => GitHubEvent::Status(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            EventKind::Watch => GitHubEvent::Watch(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            ev => {
                return Outcome::Failure((
                    Status::BadRequest,
                    format!("Found unknown event `{}`", ev),
                ));
            }
        })
    }
}

fn validate<B: AsRef<[u8]>>(secret: B, signature: B, message: B) -> bool {
    let signature = &signature.as_ref()[7..];

    let mut hmac = Hmac::<Sha256>::new_from_slice(secret.as_ref())
        .expect("failed to create Hmac digest");
    hmac.update(message.as_ref());
    let end = hmac.finalize();
    let x = end.into_bytes();
    let s = bytes_to_hex(x.as_slice());
    s.eq(signature.as_ref())
}

pub fn bytes_to_hex(bytes: &[u8]) -> Vec<u8> {
    const CHARS: &[u8] = b"0123456789abcdef";
    let mut v = Vec::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        v.push(CHARS[(byte >> 4) as usize]);
        v.push(CHARS[(byte & 0xf) as usize]);
    }
    v
}
