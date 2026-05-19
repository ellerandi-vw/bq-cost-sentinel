mod config;

use axum::{routing::get, Router};
use config::AppConfig;
use std::net::SocketAddr;
use tracing::{info, Level};
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
    info!(
        max_cost = config.max_cost_per_query,
        price_per_tib = config.price_per_tib,
        enforce_mode = config.enforce_mode,
        "Starting bq-cost-sentinel..."
    );

}