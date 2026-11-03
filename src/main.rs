mod api;
mod domain;
mod marketstack;
mod service;

use crate::marketstack::MarketstackProvider;
use crate::service::HistoryService;
use std::env;
use std::error::Error;
use std::sync::Arc;

const LISTEN_ADDRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("Server konnte nicht gestartet werden: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let api_key = env::var("MARKETSTACK_API_KEY")?;
    if api_key.trim().is_empty() {
        return Err("MARKETSTACK_API_KEY ist leer.".into());
    }

    let provider = Arc::new(MarketstackProvider::new(api_key)?);
    let history_service = HistoryService::new(provider);
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await?;

    println!("Aktienkurs-Explorer: http://{LISTEN_ADDRESS}");
    axum::serve(listener, api::router(history_service)).await?;
    Ok(())
}
