use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio;
use unique_ip_counter::{DEFAULT_LOG_PORT, DEFAULT_METRICS_PORT};

mod ip_store;
mod metric_store;

use ip_store::{IpStore, IpStoreImpl};
use metric_store::{MetricsStore, MetricsStoreImpl};

#[derive(Debug, Deserialize)]
struct Log {
    ip: String,
}

struct AppState {
    ip_store: Arc<dyn IpStore>,
    metrics_store: Arc<dyn MetricsStore>,
}

async fn handle_post_request(
    State(state): State<Arc<AppState>>,
    Json(logs): Json<Vec<Log>>,
) {
    for log in logs {
        if let Ok(ip_addr) = log.ip.parse::<IpAddr>() {
            if state.ip_store.add(ip_addr) {
                let count = state.ip_store.count() as i64;
                state.metrics_store.update_unique_ip_count(count);
            }
        }
    }
}

async fn handle_metrics_request(State(state): State<Arc<AppState>>) -> String {
    state.metrics_store.get_metrics().unwrap_or_default()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = Arc::new(AppState {
        ip_store: Arc::new(IpStoreImpl::new()),
        metrics_store: Arc::new(MetricsStoreImpl::new()?),
    });

    let metrics_app = Router::new()
        .route("/metrics", get(handle_metrics_request))
        .with_state(state.clone());
    let metrics_addr = SocketAddr::from(([127, 0, 0, 1], DEFAULT_METRICS_PORT));

    tokio::spawn(async move {
        println!("Metrics server listening on {}", metrics_addr);
        let listener = tokio::net::TcpListener::bind(metrics_addr).await.unwrap();
        axum::serve(listener, metrics_app).await.unwrap();
    });

    let app = Router::new()
        .route("/logs", post(handle_post_request))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], DEFAULT_LOG_PORT));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
