mod config;
mod auth;
mod server;

use axum::{
    routing::{get, post},
    Router
};
use config::AppConfig;
use std::{
    net::SocketAddr, 
    sync::Arc
};
use tracing::{
    info,
    Level
};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Error initializing the logging (tracing) component");

    let config = AppConfig::load_from_env();

    let shared_state = Arc::new(server::AppState {
        config: config.clone(),
    });

    info!(
        max_cost = config.max_cost_per_query,
        price_per_tib = config.price_per_tib,
        enforce_mode = config.enforce_mode,
        "Starting BigQuery Cost Sentinel in proxy mode..."
    );

    let app = Router::new()
        .route("/health", get(|| async { "OK" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}