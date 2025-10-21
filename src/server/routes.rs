// src/server/routes.rs - Route configuration

use crate::handlers;
use crate::server::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::Level;

/// Create routes for the logs service
pub fn create_logs_routes(state: AppState) -> Router {
    Router::new()
        .route("/logs", post(handlers::logs_handler))
        .route("/health", get(handlers::health_handler))
        .route("/stats", get(handlers::stats_handler))
        .with_state(state.ip_counter_service)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
                )
                .layer(CorsLayer::permissive()),
        )
}

/// Create routes for the metrics service
pub fn create_metrics_routes(state: AppState) -> Router {
    Router::new()
        .route("/metrics", get(handlers::metrics_handler))
        .with_state(state.prometheus_service)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Settings;

    #[test]
    fn test_create_routes() {
        let settings = Settings::default();
        let state = AppState::new(settings).unwrap();

        let _logs_app = create_logs_routes(state.clone());
        let _metrics_app = create_metrics_routes(state);

        // Routes are created without panic
    }
}
