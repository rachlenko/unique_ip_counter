// src/handlers/health.rs - Health check handler

use crate::models::HealthResponse;
use crate::services::IpCounterService;
use axum::{extract::State, Json};
use std::sync::Arc;

/// Handler for GET /health
pub async fn health_handler(State(service): State<Arc<IpCounterService>>) -> Json<HealthResponse> {
    Json(HealthResponse::healthy(
        service.get_unique_count(),
        service.get_uptime_seconds(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::IpCounterService;
    use crate::storage::{MockIpStore, MockMetricsStore};
    use axum::extract::State;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_health_handler() {
        let ip_store = Arc::new(MockIpStore::new());
        let metrics_store = Arc::new(MockMetricsStore::new());
        let service = Arc::new(IpCounterService::new(ip_store, metrics_store));

        let response = health_handler(State(service)).await.0;

        assert_eq!(response.status, "healthy");
        assert_eq!(response.unique_ip_count, 0);
        assert!(response.uptime.ends_with("s"));
        assert_eq!(response.version, crate::VERSION);
    }
}
