//! Test helpers for spinning up the HTTP server.

/// Handle to a spawned test instance.
pub struct TestApp {
    /// Base address of the running server, e.g. `http://127.0.0.1:12345`.
    pub address: String,
}

/// Spawns the application on an ephemeral port.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo() {
/// // Typically used from async tests:
/// // let app = spawn_app().await;
/// # }
/// ```
pub async fn spawn_app() -> TestApp {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = rustpulse::start(listener); // your axum app setup
    tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
    }
}
