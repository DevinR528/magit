use hmac::{Hmac, Mac, NewMac};
use rocket::{
    data, data::FromData, http::Status, outcome::Outcome, request::Request, Data,
};
use sha2::Sha256;
use tokio::io::AsyncReadExt;

use crate::api::GitHubEvent;

// TODO: accept: 'application/vnd.github.v3+json'
pub const X_GITHUB_EVENT: &str = "x-github-event";
pub const X_HUB_SIGNATURE: &str = "x-hub-signature-256";
pub const CONTENT_LEN: &str = "content-length";

pub const CHECK_SUITE_EVENT: &str = "check_suite";
pub const CHECK_RUN_EVENT: &str = "check_run";
pub const COMMIT_COMMENT_EVENT: &str = "commit_comment";
pub const CREATE_EVENT: &str = "create";
pub const INSTALLATION_EVENT: &str = "installation";
pub const ISSUE_EVENT: &str = "issue";
pub const ISSUE_COMMENT_EVENT: &str = "issue_comment";
pub const PULL_REQUEST_EVENT: &str = "pull_request";
pub const PULL_REQUEST_REVIEW_EVENT: &str = "pull_request_review";
pub const PULL_REQUEST_REVIEW_COMMENT_EVENT: &str = "pull_request_review_comment";
pub const PUSH_EVENT: &str = "push_review";
pub const RELEASE_EVENT: &str = "release";
pub const STARS_EVENT: &str = "star";
pub const STATUS_EVENT: &str = "status";

#[rocket::async_trait]
impl<'r> FromData<'r> for GitHubEvent<'r> {
    type Error = String;

    async fn from_data(
        request: &'r Request<'_>,
        data: Data,
    ) -> data::Outcome<GitHubEvent<'r>, Self::Error> {
        let keys = request.headers().get(X_HUB_SIGNATURE).collect::<Vec<_>>();
        let content_len = request.headers().get(CONTENT_LEN).collect::<Vec<_>>();
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

        let string = match data.open(content_len).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => {
                return Outcome::Failure((
                    Status::PayloadTooLarge,
                    "Content length was wrong".to_owned(),
                ));
            }
            Err(e) => {
                return Outcome::Failure((Status::InternalServerError, e.to_string()));
            }
        };

        // We store `string` in request-local cache for long-lived borrows.
        let body = rocket::request::local_cache!(request, string);

        let secret = match std::env::var("GITHUB_WEBHOOK_SECRET") {
            Ok(s) => s,
            Err(_) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    "No secret key found".to_owned(),
                ));
            }
        };

        if !validate(secret.as_str(), signature, body) {
            return Outcome::Failure((
                Status::BadRequest,
                "Validation failed".to_owned(),
            ));
        }

        let keys = request.headers().get(X_GITHUB_EVENT).collect::<Vec<_>>();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, "".to_owned()));
        }

        Outcome::Success(match keys[0] {
            CHECK_SUITE_EVENT => GitHubEvent::CheckSuite(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            CHECK_RUN_EVENT => GitHubEvent::CheckRun(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            COMMIT_COMMENT_EVENT => GitHubEvent::CommitComment(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            CREATE_EVENT => GitHubEvent::Create(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            INSTALLATION_EVENT => {
                GitHubEvent::Installation(
                    match serde_json::from_str(body).map_err(|e| e.to_string()) {
                        Ok(ev) => ev,
                        Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                    },
                )
            }
            ISSUE_EVENT => GitHubEvent::Issue(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            ISSUE_COMMENT_EVENT => {
                GitHubEvent::IssueComment(
                    match serde_json::from_str(body).map_err(|e| e.to_string()) {
                        Ok(ev) => ev,
                        Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                    },
                )
            }
            PULL_REQUEST_EVENT => GitHubEvent::PullRequest(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            PULL_REQUEST_REVIEW_EVENT => GitHubEvent::PullRequestReview(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            PULL_REQUEST_REVIEW_COMMENT_EVENT => GitHubEvent::PullRequestReviewComment(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            PUSH_EVENT => GitHubEvent::Push(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            RELEASE_EVENT => GitHubEvent::Release(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            STARS_EVENT => GitHubEvent::Star(
                match serde_json::from_str(body).map_err(|e| e.to_string()) {
                    Ok(ev) => ev,
                    Err(err) => return Outcome::Failure((Status::BadRequest, err)),
                },
            ),
            STATUS_EVENT => GitHubEvent::Status(
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
