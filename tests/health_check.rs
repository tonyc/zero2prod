use std::net::TcpListener;
use serde_json::Value;
use reqwest::Client;

#[tokio::test]
async fn health_check_succeeds() {
    let address = spawn_app();
    let url = format!("{}/health", address);
    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    let bytes = response.bytes().await.unwrap();
    println!("*** Text: {bytes:?}");

    let json: Value = serde_json::from_slice(&bytes).expect("looking for a json response");
    println!("*** JSON: {json:#?}");
}

#[tokio::test]
async fn health_check_json() {
    let address = spawn_app();
    let url = format!("{}/health", address);

    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    let bytes = response.bytes().await.unwrap();
    assert_eq!(15, bytes.len());

    // let json: Value = serde_json::from_slice(&bytes).expect("looking for a json response");
    // println!("*** JSON: {json:#?}");
}


fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

