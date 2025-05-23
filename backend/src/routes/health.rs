// 🧩 TASK 2: Create the /health route
// Your health_check handler should:
// ✏️ Return a simple string: "OK" with status 200.
// 💡 Tip: Use Html<&'static str> from Axum if you want to return HTML/text.
use axum::response::IntoResponse;

pub async fn health_check() -> impl IntoResponse {
    "OK"
}
