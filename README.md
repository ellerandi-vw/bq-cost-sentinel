# BigQuery Cost Sentinel

A lightweight Reverse Proxy written in Rust that intercepts queries directed to BigQuery to determine their financial cost *before* they are executed.

It acts as a financial guardrail, allowing queries through or blocking them based on the estimated bytes processed and configured budget limits.

## Summary
1. **Async Web Proxy:** Built a highly concurrent server using Tokio and Axum.
2. **Zero-Trust Authentication:** Automatically extracts the user's/Service Account's OAuth2 `Bearer` token without validating it locally, delegating IAM and security enforcement entirely to Google Cloud.
3. **Payload Mutation:** Intercepts the original BigQuery JSON payload and injects the `"dryRun": true` flag on the fly.
4. **Pre-Flight Estimation:** Submits the mutated payload to the official BigQuery API using a native Rust HTTP client (`reqwest`), retrieving the exact number of bytes that *would* be processed, completely free of charge.
5. **Financial Guardrail:** Evaluates the returned bytes against the configured mathematical formula and blocks the request with an HTTP 403 if it exceeds the project's financial limits.

---

## Cost Formula

Google Cloud bills BigQuery analysis by the Tebibyte (TiB) scanned. Note that 1 TiB is equal to $1024^4$ bytes (1,099,511,627,776 bytes), not 1000 Gigabytes. The Sentinel calculates the exact cost using the following formula:

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
* Google Cloud SDK (`gcloud` CLI) authenticated to a valid Google Cloud target environment

### Running locally
Clone the repository, create a `.env` file in the root directory, and run the application using Cargo:

```bash
# 1. Create your local .env file
echo "PORT=8080" > .env
echo "BQ_MAX_COST_PER_QUERY=12.50" >> .env
echo "BQ_PRICE_PER_TIB=6.25" >> .env
echo "ENFORCE_MODE=false" >> .env

# 2. Run the application
cargo run
```
