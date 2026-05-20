# BigQuery Cost Sentinel

A lightweight Reverse Proxy written in Rust that intercepts queries directed to BigQuery to determine their financial cost *before* they are executed.

It acts as a financial guardrail, allowing queries through or blocking them based on the estimated bytes processed and configured budget limits.

## Summary
1. **Async Web Proxy:** Built a highly concurrent server using Tokio and Axum.
2. **Zero-Trust Authentication:** Automatically extracts the user's/Service Account's OAuth2 `Bearer` token without validating it locally, delegating IAM and security enforcement entirely to Google Cloud.
3. **Payload Mutation:** Intercepts the original BigQuery JSON payload and injects the `"dryRun": true` flag on the fly.
4. **Pre-Flight Estimation:** Submits the mutated payload to the official BigQuery API using a native Rust HTTP client (`reqwest`), retrieving the exact number of bytes that *would* be processed, without incurring BigQuery compute costs.
5. **Financial Guardrail:** Evaluates the returned bytes against the configured mathematical formula and blocks the request with an HTTP 403 if it exceeds the project's financial limits.
6. **Transparent Pass-Through:** If the query is within the budget, the proxy forwards the original unaltered payload to BigQuery and streams the analytical data back to the client, acting as a fully transparent layer.

---

## Cost Formula

Google Cloud bills for BigQuery analysis per Tebibyte (TiB) scanned. Note that 1 TiB is equal to $1024^4$ bytes (1,099,511,627,776 bytes), not 1000 Gigabytes. The Sentinel calculates the exact cost using the following formula:

$$\text{Estimated Cost (USD/EUR)} = \left( \frac{\text{Processed Bytes}}{1099511627776} \right) \times \text{Price per TiB}$$

---

## Environment Configuration

The application is completely stateless and drives its behavior via environment variables (or a local `.env` file), making it highly compatible with Google Cloud Run and automated container platforms.

| Variable Name | Type | Default Value | Description |
| :--- | :--- | :--- | :--- |
| `PORT` | Integer | `8080` | The network port the proxy server binds to inside the container |
| `BQ_MAX_COST_PER_QUERY` | Float | `5.00` | The absolute maximum financial limit allowed per query |
| `BQ_PRICE_PER_TIB` | Float | `6.25` | Price per Tebibyte (TiB) scanned used to compute the cost |
| `ENFORCE_MODE` | Boolean | `true` | If `false`, violations are logged as warnings but queries are still permitted (Audit/Observer Mode) |

> [!IMPORTANT]  
> Check the current BigQuery price per TiB in your region at [BigQuery pricing](https://cloud.google.com/bigquery/pricing?hl=en) (e.g., $7.8125 at Madrid)

---

## Local Development

### Prerequisites
* Rust Toolchain (Stable channel)
* Docker
* Google Cloud SDK (`gcloud` CLI) authenticated to a valid Google Cloud target environment

### Running locally
You have two options to configure the application locally:

**Option A**: Using a `.env` file

Clone the template:
```bash
cp template.env .env
```
Run the application:
```bash
cargo run
```

**Option B**: Using terminal exports

Alternatively, you can export the variables directly into your terminal session:
```bash
export PORT=8080
export BQ_MAX_COST_PER_QUERY=12.50
export BQ_PRICE_PER_TIB=6.25
export ENFORCE_MODE=false
```
Run the application
```bash
cargo run
```

---

### Containerization & Deployment
To deploy the solution as a standardized platform building block, use the Docker configuration provided in the root directory, which produces a minimal footprint runtime image (using Debian `bookworm-slim`).

```bash
docker build -t bq-cost-sentinel:local .
```

---

### Testing the Container Locally
You can spin up the compiled container locally by passing the required environment variables:

```bash
docker run -p 8080:8080 \
  -e PORT=8080 \
  -e BQ_MAX_COST_PER_QUERY=12.50 \
  -e BQ_PRICE_PER_TIB=6.25 \
  -e ENFORCE_MODE=false \
  bq-cost-sentinel:local
```

### Testing the Proxy
Once the Sentinel is running on port 8080, you can simulate a client (like Looker, dbt, or a data analyst) sending a standard BigQuery REST API payload. 

Open a new terminal session and run the following `curl` command. It uses your active Google Cloud credentials to authenticate:
```bash
export TOKEN=$(gcloud auth print-access-token)
export PROJECT_ID="<project_id>"
```
Send the query to the local Sentinel proxy instead of Google's endpoint (remember to **edit the query**):
```bash
curl -X POST http://localhost:8080/bigquery/v2/projects/$PROJECT_ID/queries \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "SELECT * FROM `<table>`",
    "useLegacySql": false
  }'
```

* If within budget: The proxy will silently forward the query to Google Cloud and return the actual JSON data rows.
* If over budget (and ENFORCE_MODE=true): The proxy will return an `HTTP 403 Forbidden` with a JSON payload explaining the financial violation.

---

## Security Compliance & Audit Logs
When a query is forcefully blocked by the Sentinel, a structured JSON entry is written directly to standard output, which is natively caught and indexed by Google Cloud Logging.

Example of a blocked query log:

```json
{
  "timestamp": "2026-05-20T09:47:22.822010Z",
  "level": "WARN",
  "fields": {
    "message": "The query has exceeded the budget",
    "project_id": "my-google-project",
    "estimated_cost": 85.22,
    "limit": 12.50
  },
  "target": "bq_cost_sentinel::server"
}
```
This structure provides internal auditors with real-time, TISAX-compliant signals to easily identify projects running unoptimized queries.

---

## Customer Integration Guide
Once the platform team has published the core `bq-cost-sentinel` image to the central Artifact Registry, consumer teams (e.g., Data Engineering, Analytics) must follow these two simple steps to protect their BigQuery projects:

### Deploy Your Own Instance
Deploy a new Google Cloud Run service in your Google Cloud Project using the centralized Docker image provided by the Foundation. 

During deployment, configure your specific financial boundaries using environment variables:
* `BQ_MAX_COST_PER_QUERY`: Set your team's budget threshold (e.g., `10.00` USD).
* `ENFORCE_MODE`: Set to `true` to actively block expensive queries, or `false` to just log warnings to Cloud Logging without interrupting workflows.

### Override the BigQuery API Endpoint
By default, all data tools connect to the public `https://bigquery.googleapis.com` endpoint. To route your queries through the Sentinel, you simply need to override this endpoint with your newly deployed Cloud Run URL. Authentication (OAuth tokens) is handled transparently.

Here is how to configure the most common tools:

#### Python (Google Cloud Client Library)
Pass the `ClientOptions` object when instantiating your BigQuery client:

```python
from google.cloud import bigquery
from google.api_core.client_options import ClientOptions

# Replace with your actual Cloud Run URL
sentinel_url = "https://bq-cost-sentinel-xxx-ew.a.run.app"

options = ClientOptions(api_endpoint=sentinel_url)
client = bigquery.Client(client_options=options)

# Queries will now be intercepted and financially validated automatically
query_job = client.query("SELECT * FROM `my_project.my_dataset.my_table`")
results = query_job.result()
```

#### dbt (Data Build Tool)
Add the `endpoint` parameter to your target configuration in your `profiles.yml` file:
```yaml
my_bigquery_project:
  target: dev
  outputs:
    dev:
      type: bigquery
      method: oauth
      project: my_project
      dataset: my_dataset
      # Add this single line pointing to your Cloud Run service
      endpoint: https://bq-cost-sentinel-xxx-ew.a.run.app
```

#### BI Tools (Looker, Tableau, etc.)
Most enterprise Business Intelligence tools allow you to configure custom API endpoints or hostnames in their Advanced Connection Settings. Replace the default Google API hostname with your Cloud Run service URL.

---

## License
Distributed under the MIT License. See `LICENSE` for more information.
