// tests/common/mod.rs - Common test utilities

use ip_counter_service::{
    config::Settings,
    server::AppState,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Test configuration with random ports
pub fn test_settings() -> Settings {
    let mut settings = Settings::default();
    settings.server.log_port = 0; // Random port
    settings.server.metrics_port = 0; // Random port
    settings
}

/// Start a test server and return the addresses
pub async fn spawn_test_server() -> (SocketAddr, SocketAddr) {
    let settings = test_settings();
    let state = AppState::new(settings).unwrap();
    let (logs_app, metrics_app) = ip_counter_service::create_app(state);
    
    let logs_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let logs_addr = logs_listener.local_addr().unwrap();
    
    let metrics_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let metrics_addr = metrics_listener.local_addr().unwrap();
    
    tokio::spawn(async move {
        axum::serve(logs_listener, logs_app).await.unwrap();
    });
    
    tokio::spawn(async move {
        axum::serve(metrics_listener, metrics_app).await.unwrap();
    });
    
    // Give the servers time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    (logs_addr, metrics_addr)
}

/// Generate random IP addresses for testing
pub fn generate_test_ips(count: usize) -> Vec<String> {
    (1..=count)
        .map(|i| format!("192.168.{}.{}", i / 256, i % 256))
        .collect()
}
