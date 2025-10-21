// src/services/metrics.rs - Prometheus metrics service

use crate::error::Result;
use crate::storage::MetricsStore;
use std::sync::Arc;

pub struct PrometheusService {
    metrics_store: Arc<dyn MetricsStore>,
}

impl PrometheusService {
    pub fn new(metrics_store: Arc<dyn MetricsStore>) -> Self {
        Self { metrics_store }
    }

    /// Get metrics in Prometheus format
    pub fn get_metrics(&self) -> Result<String> {
        self.metrics_store
            .get_metrics()
            .map_err(crate::error::AppError::MetricsError)
    }

    /// Get the current unique IP count from metrics
    pub fn get_unique_ip_count(&self) -> i64 {
        self.metrics_store.get_unique_ip_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MockMetricsStore;

    #[test]
    fn test_get_metrics() {
        let metrics_store = Arc::new(MockMetricsStore::new());
        metrics_store.update_unique_ip_count(42);

        let service = PrometheusService::new(metrics_store);
        let metrics = service.get_metrics().unwrap();

        assert!(metrics.contains("unique_ip_addresses"));
        assert!(metrics.contains("42"));
    }

    #[test]
    fn test_get_unique_ip_count() {
        let metrics_store = Arc::new(MockMetricsStore::new());
        metrics_store.update_unique_ip_count(100);

        let service = PrometheusService::new(metrics_store);
        assert_eq!(service.get_unique_ip_count(), 100);
    }
}
