use std::env;

use anyhow::{Ok, Result};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GlobalConfig {
    pub host: String,
    pub port: u16,
    pub cors_allowed_origins: Vec<String>,
    pub cat_fact_api: String,
    pub email: String,
    pub full_name: String,
    pub stack: String,
}

impl GlobalConfig {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        Ok(Self {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| String::from("0.0.0.0")),

            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| String::from("8000"))
                .parse()
                .unwrap_or(8000),

            cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_else(|_| String::from("http://localhost:8000"))
                .split(',')
                .map(|s| String::from(s.trim()))
                .collect(),

            cat_fact_api: env::var("CAT_FACT_API")
                .unwrap_or_else(|_| String::from("http://localhost:8000")),

            email: env::var("EMAIL").unwrap_or_else(|_| String::from("iamprecieee")),

            full_name: env::var("FULL_NAME").unwrap_or_else(|_| String::from("iamprecieee")),

            stack: env::var("STACK").unwrap_or_else(|_| String::from("Rust")),
        })
    }
}
