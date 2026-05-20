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

    }
}