// src/config/settings.rs - Application settings

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub metrics: MetricsConfig,
    pub log_level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub log_port: u16,
    pub metrics_port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetricsConfig {
    pub job_name: String,
    pub scrape_interval_secs: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            metrics: MetricsConfig::default(),
            log_level: "info".to_string(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            log_port: 5000,
            metrics_port: 9102,
            host: "0.0.0.0".to_string(),
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            job_name: "ip_counter_service".to_string(),
            scrape_interval_secs: 10,
        }
    }
}

impl Settings {
    /// Load settings from environment variables
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut settings = Settings::default();

        if let Ok(port) = env::var("LOG_PORT") {
            settings.server.log_port = port.parse().unwrap_or(5000);
        }

        if let Ok(port) = env::var("METRICS_PORT") {
            settings.server.metrics_port = port.parse().unwrap_or(9102);
        }

        if let Ok(host) = env::var("SERVER_HOST") {
            settings.server.host = host;
        }

        if let Ok(level) = env::var("LOG_LEVEL") {
            settings.log_level = level;
        }

        Ok(settings)
    }

    /// Validate settings
    pub fn validate(&self) -> Result<(), String> {
        if self.server.log_port == self.server.metrics_port {
            return Err("Log port and metrics port cannot be the same".to_string());
        }

        if self.server.log_port == 0 || self.server.metrics_port == 0 {
            return Err("Ports must be non-zero".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.server.log_port, 5000);
        assert_eq!(settings.server.metrics_port, 9102);
        assert_eq!(settings.server.host, "0.0.0.0");
        assert_eq!(settings.log_level, "info");
    }

    #[test]
    fn test_validate_settings_success() {
        let settings = Settings::default();
        assert!(settings.validate().is_ok());
    }

    #[test]
    fn test_validate_settings_same_ports() {
        let mut settings = Settings::default();
        settings.server.log_port = 5000;
        settings.server.metrics_port = 5000;
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_validate_settings_zero_port() {
        let mut settings = Settings::default();
        settings.server.log_port = 0;
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_from_env() {
        env::set_var("LOG_PORT", "6000");
        env::set_var("METRICS_PORT", "9103");
        env::set_var("SERVER_HOST", "127.0.0.1");
        env::set_var("LOG_LEVEL", "debug");

        let settings = Settings::from_env().unwrap();
        assert_eq!(settings.server.log_port, 6000);
        assert_eq!(settings.server.metrics_port, 9103);
        assert_eq!(settings.server.host, "127.0.0.1");
        assert_eq!(settings.log_level, "debug");

        // Clean up
        env::remove_var("LOG_PORT");
        env::remove_var("METRICS_PORT");
        env::remove_var("SERVER_HOST");
        env::remove_var("LOG_LEVEL");
    }
}
