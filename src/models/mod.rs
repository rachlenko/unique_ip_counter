// src/models/mod.rs - Models module

mod log_entry;
mod responses;

pub use log_entry::LogEntry;
pub use responses::{HealthResponse, LogResponse, StatsResponse};
