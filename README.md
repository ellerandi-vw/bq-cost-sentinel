# BigQuery Cost Sentinel
Queries to BigQuery are sent to a Reverse Proxy to determine their cost before being sent, so that they can be allowed through or blocked based on the number of bytes processed

## Component Logic
1. **Interception:** The data client targets the Cloud Run URL of the deployed `bq-cost-sentinel` instead of `bigquery.googleapis.com`.
2. **Parsing & Cloning:** The Sentinel reads the incoming HTTP request payload, clones it asynchronously, and modifies the copy by setting `"dryRun": true`.
3. **Pre-Flight Estimation:** The Sentinel submits the dry-run payload to the official BigQuery API. BigQuery returns the exact number of bytes that *would* be processed by the query, free of charge.
4. **Policy Valuation:** The internal math engine processes the byte volume against configured pricing configurations.
   * **Pass Scenario:** The computed financial cost falls within the configured boundary. The Sentinel drops the dry-run flag, passes the original query unaltered to BigQuery, and transparently streams the rows back to the client.
   * **Block Scenario:** The query violates the financial limit. The Sentinel short-circuits the connection immediately, returning an `HTTP 400 Bad Request` with an explicit JSON payload detailing the violation, preventing any data scan from taking place.

---

## Environment Configuration

The application is completely stateless and drives its behavior via environment variables, making it highly compatible with Secret Manager and automated container platforms.

| Variable Name | Type | Default Value | Description |
| :--- | :--- | :--- | :--- |
| `PORT` | Integer | `8080` | The network port the proxy server binds to inside the container |
| `BQ_MAX_COST_PER_QUERY` | Float | `5.00` | The absolute maximum financial limit allowed per query in USD |
| `BQ_PRICE_PER_TIB` | Float | `6.25` | Price per Tebibyte (TiB) scanned used to compute limits |
| `ENFORCE_MODE` | Boolean | `true` | If `false`, violations are logged as warnings but queries are still permitted (Audit Mode) |
| `LOG_LEVEL` | String | `info` | Logging granularity. Supported: `error`, `warn`, `info`, `debug`, `trace` |
| `EXEMPT_USERS_SECRET_ID` | String | `""` | Optional ID of a GCP Secret containing a JSON array of accounts exempt from restrictions |

---

## Local Development and Installation

### Prerequisites
* Rust Toolchain (Stable channel, 1.75+ recommended)
* Docker Desktop
* Google Cloud SDK (`gcloud` CLI) authenticated to a valid Google Cloud target environment

### Building From Source
Clone the repository and run an optimized release build via Cargo:

```bash
git clone https://github.com/llerandi/bq-cost-sentinel
cd bq-cost-sentinel
cargo build --release
```
The resulting optimized native binary will be generated at `./target/release/bq-cost-sentinel`.

---

## Execution
Set up local mock configurations and execute the application:

```bash
export BQ_MAX_COST_PER_QUERY=2.50
export BQ_PRICE_PER_TIB=6.25
export LOG_LEVEL=debug
export ENFORCE_MODE=true

./target/release/bq-cost-sentinel
```

---

## Containerization & Deployment
To deploy the solution to Google Cloud Run as a standardized platform building block, use the multi-stage Docker configuration provided in the root directory.

### Building the Secure Container Image
```bash
docker build -t gcr.io/google-foundation/bq-cost-sentinel:v1.0.0 .
```

---

## Marketplace Deployment Interface (Terraform Example)
Below is an example of how consumer brands invoke this building block within their isolated infrastructure landing zones using the standardized module wrapper:

```bash
module "bigquery_budget_guardrail" {
  source  = "app.terraform.io/google-foundation/sentinel/google"
  version = "1.0.0"

  project_id         = "my-fancy-google-project-prod"
  region             = "europe-west3"
  service_name       = "bq-cost-sentinel"
  
  max_cost_per_query = 5.00
  price_per_tib      = 6.25
  enforce_mode       = true

  labels = {
    brand       = "example"
    cost-center = "1234"
    managed-by  = "myself"
  }
}
```

---

## Security Compliance & Audit Logs
When a query is forcefully blocked by the Sentinel, a structured JSON entry is written directly to standard output, which is natively caught by Cloud Logging:

```json
{
  "timestamp": "2026-05-19T11:42:01.082Z",
  "level": "WARN",
  "fields": {
    "message": "Query execution blocked due to financial guardrail breach",
    "principal_email": "junior-analyst@example.es",
    "project_id": "my-fancy-google-project-prod",
    "estimated_bytes_processed": 14298053222400,
    "calculated_cost_usd": 85.22,
    "max_allowed_cost_usd": 5.00,
    "enforced": true
  }
}
```

This strict layout provides your internal auditors with real-time TISAX-compliant signals indicating precisely which identity attempted an unoptimized query structure.

---

## License
Distributed under the MIT License. See `LICENSE` for more information.
