use std::env;
use dotenvy::dotenv;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub port: u16,
    pub max_cost_per_query: f64,
    pub price_per_tib: f64,
    pub enforce_mode: bool,
}

impl AppConfig {
    pub fn load_from_env() -> Self {
        dotenv.ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid number");

        let max_cost_per_query = env::var("BQ_MAX_COST_PER_QUERY")
            .unwrap_or_else(|_| "5.00".to_string())
            .parse::<f64>()
            .expect("BQ_MAX_COST_PER_QUERY must be a valid decimal");

        let price_per_tib = env::var("BQ_PRICE_PER_TIB")
            .unwrap_or_else(|_| "6.25".to_string())
            .parse::<f64>()
            .expect("BQ_PRICE_PER_TIB must be a valid decimal");

        let enforce_mode = env::var("ENFORCE_MODE")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        AppConfig {
            port,
            max_cost_per_query,
            price_per_tib,
            enforce_mode,
        }
    }
}