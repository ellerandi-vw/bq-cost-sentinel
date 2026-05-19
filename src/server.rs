use crate::auth::BearerToken;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::{info, warn};

pub struct AppState {
    pub config: crate::config::AppConfig,
}

pub async fn proxy_query(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
    token: BearerToken,
    Json(payload): Json<Value>, 
) -> impl IntoResponse {
    
}