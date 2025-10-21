// src/server/app.rs - Application setup

use crate::config::Settings;
use crate::services::{IpCounterService, PrometheusService};
use crate::storage::{IpStoreImpl, MetricsStoreImpl};
use axum::Router;
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub ip_counter_service: Arc<IpCounterService>,
    pub prometheus_service: Arc<PrometheusService>,
    pub settings: Settings,
}

impl AppState {
    pub fn new(settings: Settings) -> anyhow::Result<Self> {
        // Create stores
        let ip_store = Arc::new(IpStoreImpl::new());
        let metrics_store = Arc::new(MetricsStoreImpl::new()?);

        // Create services
        let ip_counter_service = Arc::new(IpCounterService::new(
            ip_store.clone(),
            metrics_store.clone(),
        ));

        let prometheus_service = Arc::new(PrometheusService::new(metrics_store.clone()));

        Ok(Self {
            ip_counter_service,
            prometheus_service,
            settings,
        })
    }
}

/// Create the application with routes
pub fn create_app(state: AppState) -> (Router, Router) {
    let logs_app = super::routes::create_logs_routes(state.clone());
    let metrics_app = super::routes::create_metrics_routes(state);

    (logs_app, metrics_app)
}
