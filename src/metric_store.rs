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
            "Total number of unique IP addresses seen since service start"
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
        
        encoder.encode(&metric_families, &mut buffer)
            .map_err(|e| format!("Failed to encode metrics: {}", e))?;
        
        String::from_utf8(buffer)
            .map_err(|e| format!("Failed to convert metrics to string: {}", e))
    }
    
    fn get_unique_ip_count(&self) -> i64 {
        self.unique_ip_gauge.get()
    }
}
