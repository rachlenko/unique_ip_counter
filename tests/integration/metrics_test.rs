// tests/integration/metrics_test.rs - Integration tests for metrics endpoint

use axum::http::StatusCode;
use http_body_util::BodyExt;
use ip_counter_service::{
    config::Settings,
    server::{create_app, AppState},
};
use tower::ServiceExt;

async fn get_test_app() -> (axum::Router, axum::Router) {
    let settings = Settings::default();
    let state = AppState::new(settings).unwrap();
    create_app(state)
}

#[tokio::test]
async fn test_metrics_endpoint() {
    let (_, metrics_app) = get_test_app().await;
    
    let request = axum::http::Request::builder()
        .method("GET")
        .uri("/metrics")
        .body(String::new())
        .unwrap();
    
    let response = metrics_app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(bytes.to_vec()).unwrap();
    
    // Check for Prometheus format
    assert!(body_str.contains("# HELP unique_ip_addresses"));
    assert!(body_str.contains("# TYPE unique_ip_addresses gauge"));
    assert!(body_str.contains("unique_ip_addresses"));
}

#[tokio::test]
async fn test_metrics_update_after_logs() {
    let (logs_app, metrics_app) = get_test_app().await;
    
    // Add some IPs through logs endpoint
    for i in 1..=3 {
        let request_body = serde_json::json!({
            "timestamp": "2024-01-01T00:00:00Z",
            "ip": format!("10.0.0.{}", i)
        });
        
        let request = axum::http::Request::builder()
            .method("POST")
            .uri("/logs")
            .header("content-type", "application/json")
            .body(serde_json::to_string(&request_body).unwrap())
            .unwrap();
        
        logs_app.clone().oneshot(request).await.unwrap();
    }
    
    // Get metrics
    let request = axum::http::Request::builder()
        .method("GET")
        .uri("/metrics")
        .body(String::new())
        .unwrap();
    
    let response = metrics_app.oneshot(request).await.unwrap();
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(bytes.to_vec()).unwrap();
    
    // Should contain the count
    assert!(body_str.contains("unique_ip_addresses 3"));
}
