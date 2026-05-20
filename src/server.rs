use crate::auth::BearerToken;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info};

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

    match state.google_client.simulate_query(&project_id, &token.0, payload).await {
        Ok(bytes) => {
            info!(
                project_id = %project_id,
                bytes_escaneados = bytes,
                "Google Cloud has successfully returned the estimate"
            );
            (
                StatusCode::OK,
                format!(
                    "Simulation successful in Google Cloud, this query will scan {} bytes",
                    bytes
                ),
            )
        }
        Err(err) => {
            error!(
                project_id = %project_id,
                error = %err,
                "The simulation on Google Cloud has failed"
            );
            (
                StatusCode::BAD_REQUEST,
                format!("Error simulating the query in BigQuery: {}", err),
            )
        }
    }

}