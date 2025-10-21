// src/storage/metrics_store.rs - Metrics storage

use lazy_static::lazy_static;
use prometheus::{Encoder, IntGauge, Registry, TextEncoder};
use std::sync::Arc;

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();
}

/// Trait for metrics storage
pub trait MetricsStore: Send + Sync {
    /// Update the unique IP count
    fn update_unique_ip_count(&self, count: i64);

    /// Get metrics in Prometheus format
    fn get_metrics(&self) -> Result<String, String>;

    /// Get the current unique IP count
    fn get_unique_ip_count(&self) -> i64;
}

/// Prometheus-based metrics store
pub struct MetricsStoreImpl {
    unique_ip_gauge: IntGauge,
    registry: Arc<Registry>,
}

impl MetricsStoreImpl {
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Registry::new();

        let unique_ip_gauge = IntGauge::new(
            "unique_ip_addresses",
            "Total number of unique IP addresses seen since service start",
        )?;

        registry.register(Box::new(unique_ip_gauge.clone()))?;

        Ok(Self {
            unique_ip_gauge,
            registry: Arc::new(registry),
        })
    }
}

impl Default for MetricsStoreImpl {
    fn default() -> Self {
        Self::new().expect("Failed to create metrics store")
    }
}

impl MetricsStore for MetricsStoreImpl {
    fn update_unique_ip_count(&self, count: i64) {
        self.unique_ip_gauge.set(count);
    }

    fn get_metrics(&self) -> Result<String, String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();

        encoder
            .encode(&metric_families, &mut buffer)
            .map_err(|e| format!("Failed to encode metrics: {}", e))?;

        String::from_utf8(buffer).map_err(|e| format!("Failed to convert metrics to string: {}", e))
    }

    fn get_unique_ip_count(&self) -> i64 {
        self.unique_ip_gauge.get()
    }
}

#[cfg(any(test, feature = "mocks"))]
pub struct MockMetricsStore {
    count: parking_lot::RwLock<i64>,
}

#[cfg(any(test, feature = "mocks"))]
impl MockMetricsStore {
    pub fn new() -> Self {
        Self {
            count: parking_lot::RwLock::new(0),
        }
    }
}

#[cfg(any(test, feature = "mocks"))]
impl Default for MockMetricsStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(test, feature = "mocks"))]
impl MetricsStore for MockMetricsStore {
    fn update_unique_ip_count(&self, count: i64) {
        *self.count.write() = count;
    }

    fn get_metrics(&self) -> Result<String, String> {
        let count = *self.count.read();
        Ok(format!(
            "# HELP unique_ip_addresses Total number of unique IP addresses\n\
             # TYPE unique_ip_addresses gauge\n\
             unique_ip_addresses {}\n",
            count
        ))
    }

    fn get_unique_ip_count(&self) -> i64 {
        *self.count.read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_and_get_count() {
        let store = MetricsStoreImpl::new().unwrap();

        store.update_unique_ip_count(42);
        assert_eq!(store.get_unique_ip_count(), 42);

        store.update_unique_ip_count(100);
        assert_eq!(store.get_unique_ip_count(), 100);
    }

    #[test]
    fn test_get_metrics_format() {
        let store = MetricsStoreImpl::new().unwrap();
        store.update_unique_ip_count(123);

        let metrics = store.get_metrics().unwrap();
        assert!(metrics.contains("unique_ip_addresses"));
        assert!(metrics.contains("123"));
        assert!(metrics.contains("# HELP"));
        assert!(metrics.contains("# TYPE"));
    }

    #[test]
    fn test_initial_value() {
        let store = MetricsStoreImpl::new().unwrap();
        assert_eq!(store.get_unique_ip_count(), 0);
    }

    #[test]
    fn test_mock_store() {
        let store = MockMetricsStore::new();

        assert_eq!(store.get_unique_ip_count(), 0);

        store.update_unique_ip_count(50);
        assert_eq!(store.get_unique_ip_count(), 50);

        let metrics = store.get_metrics().unwrap();
        assert!(metrics.contains("unique_ip_addresses 50"));
    }
}
