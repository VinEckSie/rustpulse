use axum::http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health", app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn store_event_returns_201() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let payload = json!({
        "title": "RustConf",
        "date": "2025-09-10"
    });

    let response = client
        .post(format!("{}/events", app.address))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn fetch_nonexistent_event_returns_404() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/events/unknown-id", app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
#[tokio::test]
async fn test_health_check() {
    let app = crate::startup::app(); // Use your actual router setup
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
