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
