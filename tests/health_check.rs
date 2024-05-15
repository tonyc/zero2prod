#[tokio::test]
async fn health_check_succeeds() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("failed to execute request");


    assert!(response.status().is_success());
    let text = response.text().await.expect("no response text");

    println!("**** response.text: {text:?}");

    assert_eq!(Some(19), response.content_length());
}


fn spawn_app() {
    let server = zero2prod::run().expect("failed to bind address");

    let _ = tokio::spawn(server);
}
