// src/storage/ip_store.rs - IP storage implementation

use ahash::AHasher;
use dashmap::DashSet;
use std::hash::BuildHasherDefault;
use std::net::IpAddr;
use std::sync::Arc;

/// Trait for IP storage
pub trait IpStore: Send + Sync {
    /// Add an IP address, returns true if it's new
    fn add(&self, ip: IpAddr) -> bool;

    /// Check if an IP exists
    fn contains(&self, ip: &IpAddr) -> bool;

    /// Get the count of unique IPs
    fn count(&self) -> usize;

    /// Clear all IPs
    fn clear(&self);

    /// Get all IPs (for testing/debugging)
    fn get_all(&self) -> Vec<IpAddr>;
}

/// Thread-safe implementation using DashSet
pub struct IpStoreImpl {
    ips: Arc<DashSet<IpAddr, BuildHasherDefault<AHasher>>>,
}

impl IpStoreImpl {
    pub fn new() -> Self {
        Self::with_capacity(10000)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ips: Arc::new(DashSet::with_capacity_and_hasher(
                capacity,
                BuildHasherDefault::<AHasher>::default(),
            )),
        }
    }
}

impl Default for IpStoreImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl IpStore for IpStoreImpl {
    fn add(&self, ip: IpAddr) -> bool {
        self.ips.insert(ip)
    }

    fn contains(&self, ip: &IpAddr) -> bool {
        self.ips.contains(ip)
    }

    fn count(&self) -> usize {
        self.ips.len()
    }

    fn clear(&self) {
        self.ips.clear()
    }

    fn get_all(&self) -> Vec<IpAddr> {
        self.ips.iter().map(|entry| *entry.key()).collect()
    }
}

/// Mock implementation for testing
#[cfg(any(test, feature = "mocks"))]
pub struct MockIpStore {
    ips: parking_lot::RwLock<std::collections::HashSet<IpAddr>>,
}

#[cfg(any(test, feature = "mocks"))]
impl MockIpStore {
    pub fn new() -> Self {
        Self {
            ips: parking_lot::RwLock::new(std::collections::HashSet::new()),
        }
    }
}

#[cfg(any(test, feature = "mocks"))]
impl Default for MockIpStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(test, feature = "mocks"))]
impl IpStore for MockIpStore {
    fn add(&self, ip: IpAddr) -> bool {
        self.ips.write().insert(ip)
    }

    fn contains(&self, ip: &IpAddr) -> bool {
        self.ips.read().contains(ip)
    }

    fn count(&self) -> usize {
        self.ips.read().len()
    }

    fn clear(&self) {
        self.ips.write().clear()
    }

    fn get_all(&self) -> Vec<IpAddr> {
        self.ips.read().iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_mock_store() {
        let store = MockIpStore::new();
        let ip = IpAddr::from_str("10.0.0.1").unwrap();

        assert!(store.add(ip));
        assert!(!store.add(ip));
        assert!(store.contains(&ip));
        assert_eq!(store.count(), 1);

        store.clear();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_get_all() {
        let store = IpStoreImpl::new();
        let ip1 = IpAddr::from_str("192.168.1.1").unwrap();
        let ip2 = IpAddr::from_str("192.168.1.2").unwrap();

        store.add(ip1);
        store.add(ip2);

        let all_ips = store.get_all();
        assert_eq!(all_ips.len(), 2);
        assert!(all_ips.contains(&ip1));
        assert!(all_ips.contains(&ip2));
    }

    #[test]
    fn test_ipv6_support() {
        let store = IpStoreImpl::new();
        let ipv6 = IpAddr::from_str("2001:0db8:85a3::8a2e:0370:7334").unwrap();

        assert!(store.add(ipv6));
        assert!(store.contains(&ipv6));
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn test_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let store = Arc::new(IpStoreImpl::new());
        let mut handles = vec![];

        // Spawn multiple threads adding IPs
        for i in 0..10 {
            let store_clone = Arc::clone(&store);
            let handle = thread::spawn(move || {
                for j in 0..100 {
                    let ip = IpAddr::from_str(&format!("192.168.{}.{}", i, j)).unwrap();
                    store_clone.add(ip);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Should have 1000 unique IPs
        assert_eq!(store.count(), 1000);
    }

    #[test]
    fn test_add_new_ip() {
        let store = IpStoreImpl::new();
        let ip = IpAddr::from_str("192.168.1.1").unwrap();
        assert!(store.add(ip));
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn test_add_duplicate_ip() {
        let store = IpStoreImpl::new();
        let ip = IpAddr::from_str("192.168.1.1").unwrap();
        assert!(store.add(ip));
        assert!(!store.add(ip)); // Should return false for duplicate
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn test_contains() {
        let store = IpStoreImpl::new();
        let ip1 = IpAddr::from_str("10.0.0.1").unwrap();
        let ip2 = IpAddr::from_str("10.0.0.2").unwrap();

        store.add(ip1);
        assert!(store.contains(&ip1));
        assert!(!store.contains(&ip2));
    }

    #[test]
    fn test_count_multiple_ips() {
        let store = IpStoreImpl::new();
        let ips = vec![
            "192.168.1.1",
            "192.168.1.2",
            "192.168.1.3",
            "10.0.0.1",
            "::1",
        ];

        for ip_str in ips {
            let ip = IpAddr::from_str(ip_str).unwrap();
            store.add(ip);
        }

        assert_eq!(store.count(), 5);
    }

    #[test]
    fn test_clear() {
        let store = IpStoreImpl::new();
        let ip = IpAddr::from_str("192.168.1.1").unwrap();

        store.add(ip);
        assert_eq!(store.count(), 1);

        store.clear();
        assert_eq!(store.count(), 0);
        assert!(!store.contains(&ip));
    }
}
