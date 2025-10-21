// src/models/log_entry.rs - LogEntry model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub ip: String,
    #[serde(default)]
    pub url: Option<String>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(ip: String, url: Option<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            ip,
            url,
        }
    }

    /// Validate the log entry
    pub fn validate(&self) -> Result<(), String> {
        // Validate IP address
        IpAddr::from_str(&self.ip).map_err(|e| format!("Invalid IP address: {}", e))?;

        // Validate URL if present
        if let Some(url) = &self.url {
            if url.is_empty() {
                return Err("URL cannot be empty".to_string());
            }
        }

        Ok(())
    }

    /// Parse IP address
    pub fn parse_ip(&self) -> Result<IpAddr, String> {
        IpAddr::from_str(&self.ip).map_err(|e| format!("Failed to parse IP: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_log_entry() {
        let entry = LogEntry::new("192.168.1.1".to_string(), Some("/test".to_string()));
        assert_eq!(entry.ip, "192.168.1.1");
        assert_eq!(entry.url, Some("/test".to_string()));
    }

    #[test]
    fn test_validate_valid_ipv4() {
        let entry = LogEntry::new("192.168.1.1".to_string(), None);
        assert!(entry.validate().is_ok());
    }

    #[test]
    fn test_validate_valid_ipv6() {
        let entry = LogEntry::new("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(), None);
        assert!(entry.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_ip() {
        let entry = LogEntry::new("not.an.ip".to_string(), None);
        assert!(entry.validate().is_err());
    }

    #[test]
    fn test_validate_empty_url() {
        let entry = LogEntry::new("192.168.1.1".to_string(), Some("".to_string()));
        assert!(entry.validate().is_err());
    }

    #[test]
    fn test_parse_ip_success() {
        let entry = LogEntry::new("8.8.8.8".to_string(), None);
        let ip = entry.parse_ip().unwrap();
        assert_eq!(ip, IpAddr::from_str("8.8.8.8").unwrap());
    }

    #[test]
    fn test_parse_ip_failure() {
        let entry = LogEntry::new("invalid".to_string(), None);
        assert!(entry.parse_ip().is_err());
    }

    #[test]
    fn test_deserialize_json() {
        let json = r#"{
            "timestamp": "2024-01-01T00:00:00Z",
            "ip": "10.0.0.1",
            "url": "/api/test"
        }"#;

        let entry: LogEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.ip, "10.0.0.1");
        assert_eq!(entry.url, Some("/api/test".to_string()));
    }

    #[test]
    fn test_deserialize_json_without_url() {
        let json = r#"{
            "timestamp": "2024-01-01T00:00:00Z",
            "ip": "10.0.0.1"
        }"#;

        let entry: LogEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.ip, "10.0.0.1");
        assert_eq!(entry.url, None);
    }
}
