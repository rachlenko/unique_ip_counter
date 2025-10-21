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
