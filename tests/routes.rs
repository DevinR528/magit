use magit::{
    app,
    from_data::{bytes_to_hex, CONTENT_LEN, X_GITHUB_EVENT, X_HUB_SIGNATURE},
};
use rocket::{
    http::{ContentType, Header, Status},
    local::asynchronous::Client,
};
use tokio::sync::mpsc::channel;

fn make_signature(body: &str) -> String {
    use hmac::{Mac, NewMac};

    let secret = std::env::var("__GITHUB_WEBHOOK_SECRET").unwrap();
    let mut hmac = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret.as_bytes())
        .expect("failed to create Hmac digest");

    // let canonical =
    //     serde_json::to_string(&serde_json::from_str::<serde_json::Value>(body).
    // unwrap())         .unwrap();
    // hmac.update(canonical.as_bytes());
    hmac.update(body.as_bytes());
    let end = hmac.finalize();
    let x = end.into_bytes();
    String::from_utf8_lossy(&bytes_to_hex(x.as_slice())).to_string()
}

#[tokio::test]
async fn stars() {
    let (to_matrix, mut from_gh) = channel(1024);
    let json = include_str!("../test_json/star.json");

    let client = Client::tracked(app(to_matrix)).await.expect("valid rocket instance");
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
    assert_eq!("DevinR528", from_gh.recv().await.unwrap());
}
