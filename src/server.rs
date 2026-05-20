use crate::auth::BearerToken;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info, warn};

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

    let payload_for_dryrun = payload.clone();

    match state.google_client.simulate_query(&project_id, &token.0, payload).await {
        Ok(bytes) => {
            let bytes_f64 = bytes as f64;
            let tibs = bytes_f64 / 1_099_511_627_776.0;

            let query_cost = tibs * state.config.price_per_tib;
            let is_too_expensive = query_cost > state.config.max_cost_per_query;

            if is_too_expensive {
                warn!(
                    project_id = %project_id,
                    estimated_cost = query_cost,
                    limite = state.config.max_cost_per_query,
                    "The query has exceeded the budget"
                );

                if state.config.enforce_mode {
                    return (
                        StatusCode::FORBIDDEN,
                        format!(
                            "Blocked by BigQuery Sentinel: The query costs ${:.2}, exceeding your limit of ${:.2}",
                            query_cost, state.config.max_cost_per_query
                        ),
                    ).into_response();
                }
            }

            info!(
                project_id = %project_id,
                estimated_cost = query_cost,
                "Query validated... Within safe limits"
            );

            // cheap query or enforce_mode == false
            (
                StatusCode::OK,
                format!("Approved, estimated cost: ${:.2} ({} bytes)", query_cost, bytes),
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