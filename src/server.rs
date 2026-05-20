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

#[allow(dead_code)]
pub struct AppState {
    pub config: crate::config::AppConfig,
    pub google_client: crate::google_client::BqClient
}

pub async fn proxy_query(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
    token: BearerToken,
    Json(payload): Json<Value>, 
) -> impl IntoResponse {
    
    info!(
        project_id = %project_id,
        "Request for a query sent to BigQuery was intercepted"
    );

    let sql_query = payload
        .get("query")
        .and_then(|q| q.as_str())
        .unwrap_or("Query not found in the payload");

    warn!(
        query_text = %sql_query,
        "Simulating evaluation (not yet connected to Google Cloud)"
    );

    (
        StatusCode::OK,
        format!("Simulation OK. Request for project {} intercepted", project_id),
    )
}