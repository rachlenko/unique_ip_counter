// src/server/mod.rs - Server module

mod app;
mod routes;

pub use app::{create_app, AppState};
