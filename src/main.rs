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

    let debug_output = parse_debug_flag(
        env::var("APP_DEBUG").ok().as_deref(),
        cfg!(debug_assertions),
    );
    let provider = Arc::new(MarketstackProvider::new(api_key)?);
    let history_service = HistoryService::new(provider);
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await?;

    println!("Aktienkurs-Explorer: http://{LISTEN_ADDRESS}");
    println!(
        "Diagnoseausgabe: {}",
        if debug_output { "aktiv" } else { "aus" }
    );
    axum::serve(listener, api::router(history_service, debug_output)).await?;
    Ok(())
}

fn parse_debug_flag(value: Option<&str>, debug_build: bool) -> bool {
    match value.map(str::trim).map(str::to_ascii_lowercase).as_deref() {
        Some("1" | "true" | "yes" | "on") => true,
        Some("0" | "false" | "no" | "off") => false,
        _ => debug_build,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_debug_flag;

    #[test]
    fn debug_flag_can_override_build_default() {
        assert!(parse_debug_flag(Some("true"), false));
        assert!(!parse_debug_flag(Some("false"), true));
        assert!(parse_debug_flag(None, true));
        assert!(!parse_debug_flag(None, false));
    }
}
