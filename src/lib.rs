// src/lib.rs - Library root, exports public API

pub mod config;
pub mod error;
pub mod handlers;
pub mod models;
pub mod server;
pub mod services;
pub mod storage;

// Re-export commonly used types
pub use config::Settings;
pub use error::{AppError, Result};
pub use server::create_app;
pub use services::ip_counter::IpCounterService;
pub use storage::ip_store::IpStore;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default log port
pub const DEFAULT_LOG_PORT: u16 = 5000;

/// Default metrics port
pub const DEFAULT_METRICS_PORT: u16 = 9102;
