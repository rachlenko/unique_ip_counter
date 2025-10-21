// src/models/responses.rs - API response models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogResponse {
    pub status: String,
    pub message: String,
    pub unique_ips: usize,
}

impl LogResponse {
    pub fn success(message: impl Into<String>, unique_ips: usize) -> Self {
        Self {
            status: "success".to_string(),
            message: message.into(),
            unique_ips,
        }
    }

    pub fn error(message: impl Into<String>, unique_ips: usize) -> Self {
        Self {
            status: "error".to_string(),
            message: message.into(),
            unique_ips,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatsResponse {
    pub unique_ip_addresses: usize,
    pub estimated_memory_usage_bytes: usize,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
    pub unique_ip_count: usize,
    pub uptime: String,
    pub version: String,
}

impl HealthResponse {
    pub fn healthy(unique_ip_count: usize, uptime_seconds: u64) -> Self {
        Self {
            status: "healthy".to_string(),
            unique_ip_count,
            uptime: format!("{}s", uptime_seconds),
            version: crate::VERSION.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_response_success() {
        let response = LogResponse::success("IP logged", 42);
        assert_eq!(response.status, "success");
        assert_eq!(response.message, "IP logged");
        assert_eq!(response.unique_ips, 42);
    }

    #[test]
    fn test_log_response_error() {
        let response = LogResponse::error("Invalid IP", 10);
        assert_eq!(response.status, "error");
        assert_eq!(response.message, "Invalid IP");
        assert_eq!(response.unique_ips, 10);
    }

    #[test]
    fn test_health_response() {
        let response = HealthResponse::healthy(100, 3600);
        assert_eq!(response.status, "healthy");
        assert_eq!(response.unique_ip_count, 100);
        assert_eq!(response.uptime, "3600s");
    }

    #[test]
    fn test_serialize_stats_response() {
        let stats = StatsResponse {
            unique_ip_addresses: 1000,
            estimated_memory_usage_bytes: 32000,
            uptime_seconds: 7200,
        };

        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("\"unique_ip_addresses\":1000"));
        assert!(json.contains("\"estimated_memory_usage_bytes\":32000"));
        assert!(json.contains("\"uptime_seconds\":7200"));
    }
}
