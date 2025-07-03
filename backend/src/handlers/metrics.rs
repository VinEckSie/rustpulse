
use axum::{Json, routing::get, Router};
use crate::adapters::postgres_metrics_repo::PostgresMetricsRepository;
use crate::app::metrics_service::MetricsService;

// pub fn router() -> Router {
//     Router::new().route("/metrics", get(get_metrics))
// }

// pub async fn get_metrics() -> Json<String> {
//     let repo = PostgresMetricsRepository;
//     let service = MetricsService::new(&repo);
//     let metrics = service.get_status();
// 
//     Json(format!("Id: {}, Server Id: {}, CPU: {}, RAM: {}", metrics.id, metrics.server_id,metrics.cpu,metrics.ram))
// }
