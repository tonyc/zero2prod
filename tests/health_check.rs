use serde_json::{Value};
use reqwest::{Client};

#[tokio::test]
async fn health_check_succeeds() {
    spawn_app();

    let client = Client::new();

    let response = client
        .get("http://localhost:8000/health")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    let text = response.bytes().await.unwrap();
    println!("*** Text: {:?}", text);

    let json: Value = serde_json::from_slice(&text).expect("looking for a json response");
    println!("*** JSON: {:#?}", json);
}


fn spawn_app() {
    let server = zero2prod::run().expect("failed to bind address");

    let _ = tokio::spawn(server);
}

