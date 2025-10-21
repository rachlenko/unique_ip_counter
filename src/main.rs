// src/main.rs - Application entry point

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use unique_ip_counter::{
    config::Settings,
    server::{create_app, AppState},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    init_tracing()?;

    // Load configuration
    let settings = Settings::from_env()?;
    settings
        .validate()
        .map_err(|e| anyhow::anyhow!("Invalid configuration: {}", e))?;

    info!(
        "Starting IP Counter Service v{}",
        unique_ip_counter::VERSION
    );
    info!("Configuration: {:?}", settings);

    // Create application state
    let state = AppState::new(settings.clone())?;

    // Create application routers
    let (logs_app, metrics_app) = create_app(state);

    // Create socket addresses
    let logs_addr = SocketAddr::from(([0, 0, 0, 0], settings.server.log_port));
    let metrics_addr = SocketAddr::from(([0, 0, 0, 0], settings.server.metrics_port));

    // Bind to ports
    let logs_listener = TcpListener::bind(logs_addr).await?;
    let metrics_listener = TcpListener::bind(metrics_addr).await?;

    info!("Logs service listening on {}", logs_addr);
    info!("Metrics service listening on {}", metrics_addr);

    // Start servers
    let logs_handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(logs_listener, logs_app).await {
            error!("Logs server error: {}", e);
        }
    });

    let metrics_handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(metrics_listener, metrics_app).await {
            error!("Metrics server error: {}", e);
        }
    });

    // Wait for servers
    tokio::try_join!(logs_handle, metrics_handle)?;

    Ok(())
}

fn init_tracing() -> anyhow::Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "unique_ip_counter=info,tower_http=debug".into());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}
