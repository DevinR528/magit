use magit::{
    from_data::{bytes_to_hex, CONTENT_LEN, X_GITHUB_EVENT, X_HUB_SIGNATURE},
    routes, Config, RepoRoomMap, Store,
};
use rocket::{
    catchers,
    http::{ContentType, Header, Status},
    local::asynchronous::Client,
    routes, Build, Rocket,
};
use ruma::{room_id, RoomId};
use tokio::sync::mpsc::{channel, Sender};

fn app(to_matrix: Sender<(RoomId, String)>) -> Rocket<Build> {
    let mut config = Config::debug();
    config.github.repos.push(RepoRoomMap {
        repo: "DevinR528/cargo-sort".to_owned(),
        room: room_id!("!aaa:aaa.com"),
    });
    config.github.repos.push(RepoRoomMap {
        repo: "Codertocat/Hello-World".to_owned(),
        room: room_id!("!aaa:aaa.com"),
    });

    config.github.events.push("star".into());
    config.github.events.push("pull_request".into());
    config.github.events.push("issues".into());

    std::env::set_var(
        "__GITHUB_WEBHOOK_SECRET",
        &config.secret_key.as_deref().unwrap_or(""),
    );

    let store = Store { config, to_matrix };

    rocket::build()
        .manage(store)
        .mount("/", routes![routes::index])
        .register("/", catchers![routes::not_found])
}

fn make_signature(body: &str) -> String {
    use hmac::{Mac, NewMac};

    let secret = std::env::var("__GITHUB_WEBHOOK_SECRET").unwrap();
    let mut hmac = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret.as_bytes())
        .expect("failed to create Hmac digest");
    hmac.update(body.as_bytes());
    let end = hmac.finalize();
    let x = end.into_bytes();
    String::from_utf8_lossy(&bytes_to_hex(x.as_slice())).to_string()
}

#[tokio::test]
async fn stars() {
    let (to_matrix, mut from_gh) = channel(1024);
    let json = include_str!("../gitty-hub/test_json/star.json");

    let client = Client::debug(app(to_matrix)).await.expect("valid rocket instance");
    let response = client
        .post("/")
        .header(ContentType::JSON)
        .header(Header::new(CONTENT_LEN, json.len().to_string()))
        .header(Header::new(X_GITHUB_EVENT, "star"))
        .header(Header::new(X_HUB_SIGNATURE, format!("sha256={}", make_signature(json))))
        .body(json)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!("DevinR528", from_gh.recv().await.unwrap().1);
}

#[tokio::test]
async fn pull_request() {
    let (to_matrix, mut from_gh) = channel(1024);
    let json = include_str!("../gitty-hub/test_json/pull_request.json");

    let client = Client::debug(app(to_matrix)).await.expect("valid rocket instance");
    let response = client
        .post("/")
        .header(ContentType::JSON)
        .header(Header::new(CONTENT_LEN, json.len().to_string()))
        .header(Header::new(X_GITHUB_EVENT, "pull_request"))
        .header(Header::new(X_HUB_SIGNATURE, format!("sha256={}", make_signature(json))))
        .body(json)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    println!("{:?}", from_gh.recv().await.unwrap());
}

#[tokio::test]
async fn issue() {
    let (to_matrix, mut from_gh) = channel(1024);
    let json = include_str!("../gitty-hub/test_json/issue.json");

    let client = Client::debug(app(to_matrix)).await.expect("valid rocket instance");
    let response = client
        .post("/")
        .header(ContentType::JSON)
        .header(Header::new(CONTENT_LEN, json.len().to_string()))
        .header(Header::new(X_GITHUB_EVENT, "issues"))
        .header(Header::new(X_HUB_SIGNATURE, format!("sha256={}", make_signature(json))))
        .body(json)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    println!("{:?}", from_gh.recv().await.unwrap());
}
