// src/handlers/metrics.rs - Metrics endpoint handler

use crate::error::Result;
use crate::services::PrometheusService;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;

/// Handler for GET /metrics
pub async fn metrics_handler(
    State(service): State<Arc<PrometheusService>>,
) -> Result<impl IntoResponse> {
    let metrics = service.get_metrics()?;
    Ok(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::PrometheusService;
    use crate::storage::{MetricsStore, MockMetricsStore};
    use axum::extract::State;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_metrics_handler() {
        let metrics_store = Arc::new(MockMetricsStore::new());
        metrics_store.update_unique_ip_count(25);

        let service = Arc::new(PrometheusService::new(metrics_store));
        let result = metrics_handler(State(service)).await;

        assert!(result.is_ok());
        let _metrics = result.unwrap().into_response();
        // The actual response body testing would be more complex
        // as we'd need to extract the body from the Response
    }
}
