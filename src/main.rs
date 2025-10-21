use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio;
use unique_ip_counter::{DEFAULT_LOG_PORT, DEFAULT_METRICS_PORT};

#[derive(Debug, Deserialize)]
struct Log {
    ip: String,
}

async fn handle_post_request(Json(logs): Json<Vec<Log>>) {
    for log in logs {
        println!("Received log for IP: {}", log.ip);
    }
}

async fn handle_metrics_request() -> String {
    "Metrics placeholder".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let metrics_app = Router::new().route("/metrics", get(handle_metrics_request));
    let metrics_addr = SocketAddr::from(([127, 0, 0, 1], DEFAULT_METRICS_PORT));

    tokio::spawn(async move {
        println!("Metrics server listening on {}", metrics_addr);
        let listener = tokio::net::TcpListener::bind(metrics_addr).await.unwrap();
        axum::serve(listener, metrics_app).await.unwrap();
    });

    let app = Router::new().route("/logs", post(handle_post_request));

    let addr = SocketAddr::from(([127, 0, 0, 1], DEFAULT_LOG_PORT));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}