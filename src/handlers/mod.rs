// src/handlers/mod.rs - Request handlers module

mod health;
mod logs;
mod metrics;

pub use health::health_handler;
pub use logs::{logs_handler, stats_handler};
pub use metrics::metrics_handler;
