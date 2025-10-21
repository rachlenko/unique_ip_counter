// tests/integration/health_test.rs - Integration tests for health endpoint

use axum::http::StatusCode;
use http_body_util::BodyExt;
use ip_counter_service::{
    config::Settings,
    models::HealthResponse,
    server::{create_app, AppState},
};
use tower::ServiceExt;

async fn get_test_app() -> axum::Router {
    let settings = Settings::default();
    let state = AppState::new(settings).unwrap();
    let (logs_app, _) = create_app(state);
    logs_app
}

#[tokio::test]
async fn test_health_endpoint() {
    let app = get_test_app().await;
    
    let request = axum::http::Request::builder()
        .method("GET")
        .uri("/health")
        .body(String::new())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let health: HealthResponse = serde_json::from_slice(&bytes).unwrap();
    
    assert_eq!(health.status, "healthy");
    assert_eq!(health.unique_ip_count, 0);
    assert!(health.uptime.ends_with("s"));
    assert_eq!(health.version, ip_counter_service::VERSION);
}

#[tokio::test]
async fn test_health_with_ips() {
    let app = get_test_app().await;
    
    // Add some IPs
    for i in 1..=5 {
        let request_body = serde_json::json!({
            "timestamp": "2024-01-01T00:00:00Z",
            "ip": format!("172.16.0.{}", i)
        });
        
        let request = axum::http::Request::builder()
            .method("POST")
            .uri("/logs")
            .header("content-type", "application/json")
            .body(serde_json::to_string(&request_body).unwrap())
            .unwrap();
        
        app.clone().oneshot(request).await.unwrap();
    }
    
    // Check health
    let request = axum::http::Request::builder()
        .method("GET")
        .uri("/health")
        .body(String::new())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let health: HealthResponse = serde_json::from_slice(&bytes).unwrap();
    
    assert_eq!(health.status, "healthy");
    assert_eq!(health.unique_ip_count, 5);
}
