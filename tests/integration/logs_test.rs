// tests/integration/logs_test.rs - Integration tests for logs endpoint

use axum::http::StatusCode;
use axum::response::Response;
use http_body_util::BodyExt;
use ip_counter_service::{
    config::Settings,
    models::{LogEntry, LogResponse},
    server::{create_app, AppState},
};
use serde_json::json;
use tower::ServiceExt;

async fn get_test_app() -> axum::Router {
    let settings = Settings::default();
    let state = AppState::new(settings).unwrap();
    let (logs_app, _) = create_app(state);
    logs_app
}

async fn read_body_json<T: serde::de::DeserializeOwned>(response: Response) -> T {
    let body = response.into_body();
    let bytes = body
        .collect()
        .await
        .expect("Failed to read body")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("Failed to parse JSON")
}

#[tokio::test]
async fn test_post_valid_log_entry() {
    let app = get_test_app().await;
    
    let request_body = json!({
        "timestamp": "2024-01-01T00:00:00Z",
        "ip": "192.168.1.1",
        "url": "/test"
    });
    
    let request = axum::http::Request::builder()
        .method("POST")
        .uri("/logs")
        .header("content-type", "application/json")
        .body(serde_json::to_string(&request_body).unwrap())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body: LogResponse = read_body_json(response).await;
    assert_eq!(body.status, "success");
    assert_eq!(body.unique_ips, 1);
}

#[tokio::test]
async fn test_post_duplicate_ip() {
    let app = get_test_app().await;
    
    let request_body = json!({
        "timestamp": "2024-01-01T00:00:00Z",
        "ip": "10.0.0.1"
    });
    
    // First request
    let request1 = axum::http::Request::builder()
        .method("POST")
        .uri("/logs")
        .header("content-type", "application/json")
        .body(serde_json::to_string(&request_body).unwrap())
        .unwrap();
    
    let response1 = app.clone().oneshot(request1).await.unwrap();
    assert_eq!(response1.status(), StatusCode::OK);
    
    // Duplicate request
    let request2 = axum::http::Request::builder()
        .method("POST")
        .uri("/logs")
        .header("content-type", "application/json")
        .body(serde_json::to_string(&request_body).unwrap())
        .unwrap();
    
    let response2 = app.oneshot(request2).await.unwrap();
    assert_eq!(response2.status(), StatusCode::OK);
    
    let body: LogResponse = read_body_json(response2).await;
    assert_eq!(body.message, "IP already seen");
    assert_eq!(body.unique_ips, 1);
}

#[tokio::test]
async fn test_post_invalid_ip() {
    let app = get_test_app().await;
    
    let request_body = json!({
        "timestamp": "2024-01-01T00:00:00Z",
        "ip": "not.an.ip"
    });
    
    let request = axum::http::Request::builder()
        .method("POST")
        .uri("/logs")
        .header("content-type", "application/json")
        .body(serde_json::to_string(&request_body).unwrap())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body: LogResponse = read_body_json(response).await;
    assert_eq!(body.status, "error");
    assert!(body.message.contains("Invalid IP"));
}

#[tokio::test]
async fn test_post_ipv6() {
    let app = get_test_app().await;
    
    let request_body = json!({
        "timestamp": "2024-01-01T00:00:00Z",
        "ip": "2001:0db8:85a3::8a2e:0370:7334"
    });
    
    let request = axum::http::Request::builder()
        .method("POST")
        .uri("/logs")
        .header("content-type", "application/json")
        .body(serde_json::to_string(&request_body).unwrap())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body: LogResponse = read_body_json(response).await;
    assert_eq!(body.status, "success");
    assert_eq!(body.unique_ips, 1);
}

#[tokio::test]
async fn test_multiple_unique_ips() {
    let app = get_test_app().await;
    
    let ips = vec![
        "192.168.1.1",
        "192.168.1.2",
        "10.0.0.1",
        "172.16.0.1",
    ];
    
    for (i, ip) in ips.iter().enumerate() {
        let request_body = json!({
            "timestamp": "2024-01-01T00:00:00Z",
            "ip": ip
        });
        
        let request = axum::http::Request::builder()
            .method("POST")
            .uri("/logs")
            .header("content-type", "application/json")
            .body(serde_json::to_string(&request_body).unwrap())
            .unwrap();
        
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        
        let body: LogResponse = read_body_json(response).await;
        assert_eq!(body.unique_ips, i + 1);
    }
}

#[tokio::test]
async fn test_get_stats() {
    let app = get_test_app().await;
    
    // Add some IPs first
    for i in 1..=5 {
        let request_body = json!({
            "timestamp": "2024-01-01T00:00:00Z",
            "ip": format!("192.168.1.{}", i)
        });
        
        let request = axum::http::Request::builder()
            .method("POST")
            .uri("/logs")
            .header("content-type", "application/json")
            .body(serde_json::to_string(&request_body).unwrap())
            .unwrap();
        
        app.clone().oneshot(request).await.unwrap();
    }
    
    // Get stats
    let request = axum::http::Request::builder()
        .method("GET")
        .uri("/stats")
        .body(String::new())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body: serde_json::Value = read_body_json(response).await;
    assert_eq!(body["unique_ip_addresses"], 5);
    assert_eq!(body["estimated_memory_usage_bytes"], 5 * 32);
}
