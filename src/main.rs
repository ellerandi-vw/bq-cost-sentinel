mod config;

use axum::{routing::get, Router};
use config::AppConfig;
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
