

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client.get(format!("{}/health", app.address))
        .send().await.unwrap();

    assert!(response.status().is_success());
}
