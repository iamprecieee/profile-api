use std::net::SocketAddr;

use anyhow::{anyhow, Ok, Result};
use dotenvy::dotenv;
use hng_stage_0::{
    api::{build_app, AppContext},
    config::GlobalConfig,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt().init();

    let config = GlobalConfig::from_env().map_err(|e| {
        tracing::error!("Failed to complete configuration: {}", e);
        anyhow!("Configuration error")
    })?;

    let addr = format!("{}:{}", config.host, config.port);

    let context = AppContext {
        config: config.clone(),
    };

    let app = build_app(context).await;

    let listener = TcpListener::bind(&addr).await.map_err(|e| {
        tracing::error!("Failed to bind to {}: {}", addr, e);
        anyhow!("Server startup error")
    })?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .map_err(|e| anyhow!("Server error {}", e))?;

    Ok(())
}
