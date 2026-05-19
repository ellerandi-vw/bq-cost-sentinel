use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub port: u16,
    pub max_cost_per_query: f64,
    pub price_per_tib: f64,
    pub enforce_mode: bool,
}

impl AppConfig {
    pub fn load_from_env() -> Self {

    }
}