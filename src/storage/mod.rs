// src/storage/mod.rs - Storage module

pub mod ip_store;
mod metrics_store;

pub use ip_store::{IpStore, IpStoreImpl};
pub use metrics_store::{MetricsStore, MetricsStoreImpl};

#[cfg(any(test, feature = "mocks"))]
pub use ip_store::MockIpStore;

#[cfg(any(test, feature = "mocks"))]
pub use metrics_store::MockMetricsStore;
