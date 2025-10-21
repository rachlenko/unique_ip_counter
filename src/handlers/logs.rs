// src/handlers/logs.rs - Logs endpoint handlers

use crate::error::Result;
use crate::models::{LogEntry, LogResponse, StatsResponse};
use crate::services::IpCounterService;
use axum::{extract::State, Json};
use std::sync::Arc;
use tracing::warn;

/// Handler for POST /logs
pub async fn logs_handler(
    State(service): State<Arc<IpCounterService>>,
    Json(entry): Json<LogEntry>,
) -> Result<Json<LogResponse>> {
    match service.process_log_entry(&entry) {
        Ok(is_new) => {
            let unique_count = service.get_unique_count();
            let message = if is_new {
                "New IP logged"
            } else {
                "IP already seen"
            };

            Ok(Json(LogResponse::success(message, unique_count)))
        }
        Err(e) => {
            warn!("Failed to process log entry: {}", e);
            Ok(Json(LogResponse::error(
                e.to_string(),
                service.get_unique_count(),
            )))
        }
    }
}

/// Handler for GET /stats
pub async fn stats_handler(State(service): State<Arc<IpCounterService>>) -> Json<StatsResponse> {
    Json(StatsResponse {
        unique_ip_addresses: service.get_unique_count(),
        estimated_memory_usage_bytes: service.get_estimated_memory_usage(),
        uptime_seconds: service.get_uptime_seconds(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::IpCounterService;
    use crate::storage::{MockIpStore, MockMetricsStore};
    use axum::extract::State;

    async fn create_test_handler() -> Arc<IpCounterService> {
        let ip_store = Arc::new(MockIpStore::new());
        let metrics_store = Arc::new(MockMetricsStore::new());
        Arc::new(IpCounterService::new(ip_store, metrics_store))
    }

    #[tokio::test]
    async fn test_logs_handler_valid() {
        let service = create_test_handler().await;
        let entry = LogEntry::new("192.168.1.1".to_string(), None);

        let result = logs_handler(State(service.clone()), Json(entry)).await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert_eq!(response.status, "success");
        assert_eq!(response.unique_ips, 1);
    }

    #[tokio::test]
    async fn test_logs_handler_duplicate() {
        let service = create_test_handler().await;
        let entry = LogEntry::new("192.168.1.1".to_string(), None);

        // First request
        let _ = logs_handler(State(service.clone()), Json(entry.clone()))
            .await
            .unwrap();

        // Duplicate request
        let result = logs_handler(State(service.clone()), Json(entry)).await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert_eq!(response.status, "success");
        assert_eq!(response.message, "IP already seen");
        assert_eq!(response.unique_ips, 1);
    }

    #[tokio::test]
    async fn test_stats_handler() {
        let service = create_test_handler().await;

        // Add some IPs
        for i in 1..=5 {
            let entry = LogEntry::new(format!("192.168.1.{}", i), None);
            service.process_log_entry(&entry).unwrap();
        }

        let response = stats_handler(State(service)).await.0;

        assert_eq!(response.unique_ip_addresses, 5);
        assert_eq!(response.estimated_memory_usage_bytes, 5 * 32);
    }
}
