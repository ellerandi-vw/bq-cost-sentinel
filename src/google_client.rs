use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BqDryRunResponse {
    pub total_bytes_processed: Option<String>, 
}

#[derive(Clone)]
pub struct BqClient {
    http_client: Client,
}

impl BqClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    pub async fn simulate_query(
        &self,
        project_id: &str,
        token: &str,
        mut payload: Value,
    ) -> Result<u64, String> {
        let url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/queries",
            project_id
        );

        if let Some(obj) = payload.as_object_mut() {
            obj.insert(
                "dryRun".to_string(),
                Value::Bool(true),
            );
        }

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error while connectin to Google Cloud: {}", e))?;

        if response.status() != StatusCode::OK {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown Google Cloud error".to_string());
            return Err(format!("Google Cloud rejected the simulation: {}", error_body));
        }

        let bq_response: BqDryRunResponse = response
            .json()
            .await
            .map_err(|e| format!("Error reading Google Cloud JSON: {}", e))?;

        let bytes_str = bq_response
            .total_bytes_processed
            .ok_or("Google Cloud did not return the totalBytesProcessed field")?;
            
        let bytes = bytes_str
            .parse::<u64>()
            .map_err(|_| "The totalBytesProcessed field is not a valid number")?;

        Ok(bytes)

    }
}