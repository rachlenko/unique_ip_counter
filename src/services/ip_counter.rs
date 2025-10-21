// src/services/ip_counter.rs - IP counting service

use crate::error::{AppError, Result};
use crate::models::LogEntry;
use crate::storage::{IpStore, MetricsStore};
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, info};

pub struct IpCounterService {
    ip_store: Arc<dyn IpStore>,
    metrics_store: Arc<dyn MetricsStore>,
    start_time: Instant,
}

impl IpCounterService {
    pub fn new(ip_store: Arc<dyn IpStore>, metrics_store: Arc<dyn MetricsStore>) -> Self {
        Self {
            ip_store,
            metrics_store,
            start_time: Instant::now(),
        }
    }

    /// Process a log entry and return whether the IP is new
    pub fn process_log_entry(&self, entry: &LogEntry) -> Result<bool> {
        // Validate the entry
        entry.validate().map_err(AppError::BadRequest)?;

        // Parse the IP
        let ip = entry.parse_ip().map_err(AppError::InvalidIpAddress)?;

        // Add to store
        let is_new = self.ip_store.add(ip);

        if is_new {
            // Update metrics
            let count = self.ip_store.count() as i64;
            self.metrics_store.update_unique_ip_count(count);

            info!(
                ip = %ip,
                timestamp = %entry.timestamp,
                total_unique = count,
                "New unique IP address"
            );
        } else {
            debug!(
                ip = %ip,
                timestamp = %entry.timestamp,
                "Duplicate IP address"
            );
        }

        Ok(is_new)
    }

    /// Get the count of unique IPs
    pub fn get_unique_count(&self) -> usize {
        self.ip_store.count()
    }

    /// Check if an IP exists
    pub fn contains_ip(&self, ip: &IpAddr) -> bool {
        self.ip_store.contains(ip)
    }

    /// Get uptime in seconds
    pub fn get_uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Get estimated memory usage
    pub fn get_estimated_memory_usage(&self) -> usize {
        // Rough estimate: 32 bytes per IP address
        self.ip_store.count() * 32
    }

    /// Clear all IPs (for testing)
    #[cfg(test)]
    pub fn clear(&self) {
        self.ip_store.clear();
        self.metrics_store.update_unique_ip_count(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{MockIpStore, MockMetricsStore};
    use std::str::FromStr;

    fn create_test_service() -> IpCounterService {
        let ip_store = Arc::new(MockIpStore::new());
        let metrics_store = Arc::new(MockMetricsStore::new());
        IpCounterService::new(ip_store, metrics_store)
    }

    #[test]
    fn test_process_valid_entry() {
        let service = create_test_service();
        let entry = LogEntry::new("192.168.1.1".to_string(), Some("/test".to_string()));

        let result = service.process_log_entry(&entry);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should be new
        assert_eq!(service.get_unique_count(), 1);
    }

    #[test]
    fn test_process_duplicate_entry() {
        let service = create_test_service();
        let entry = LogEntry::new("192.168.1.1".to_string(), None);

        // First entry
        let result1 = service.process_log_entry(&entry).unwrap();
        assert!(result1);

        // Duplicate entry
        let result2 = service.process_log_entry(&entry).unwrap();
        assert!(!result2);

        assert_eq!(service.get_unique_count(), 1);
    }

    #[test]
    fn test_process_invalid_ip() {
        let service = create_test_service();
        let entry = LogEntry::new("not.an.ip".to_string(), None);

        let result = service.process_log_entry(&entry);
        assert!(result.is_err());
        assert_eq!(service.get_unique_count(), 0);
    }

    #[test]
    fn test_multiple_unique_ips() {
        let service = create_test_service();

        let ips = vec![
            "192.168.1.1",
            "192.168.1.2",
            "10.0.0.1",
            "::1",
            "2001:db8::1",
        ];

        for ip in ips {
            let entry = LogEntry::new(ip.to_string(), None);
            service.process_log_entry(&entry).unwrap();
        }

        assert_eq!(service.get_unique_count(), 5);
    }

    #[test]
    fn test_contains_ip() {
        let service = create_test_service();
        let entry = LogEntry::new("192.168.1.1".to_string(), None);

        service.process_log_entry(&entry).unwrap();

        let ip = IpAddr::from_str("192.168.1.1").unwrap();
        assert!(service.contains_ip(&ip));

        let other_ip = IpAddr::from_str("192.168.1.2").unwrap();
        assert!(!service.contains_ip(&other_ip));
    }

    #[test]
    fn test_estimated_memory_usage() {
        let service = create_test_service();

        for i in 1..=10 {
            let entry = LogEntry::new(format!("192.168.1.{}", i), None);
            service.process_log_entry(&entry).unwrap();
        }

        let memory = service.get_estimated_memory_usage();
        assert_eq!(memory, 10 * 32);
    }

    #[test]
    fn test_clear() {
        let service = create_test_service();
        let entry = LogEntry::new("192.168.1.1".to_string(), None);

        service.process_log_entry(&entry).unwrap();
        assert_eq!(service.get_unique_count(), 1);

        service.clear();
        assert_eq!(service.get_unique_count(), 0);
    }

    #[test]
    fn test_uptime() {
        let service = create_test_service();
        std::thread::sleep(std::time::Duration::from_millis(100));

        let _uptime = service.get_uptime_seconds();
    }
}
