use chrono::{Datelike, Duration as ChronoDuration, Utc};
use reqwest::blocking::Client;
use std::env;
use std::time::Duration;

const API_BASE_URL: &str = "https://api.marketstack.com/v1";
const ISIN: &str = "IE00B0M62Q58";
const MARKETSTACK_SYMBOL: &str = "IQQW.XETRA";

fn main() {
    if let Err(message) = run() {
        eprintln!("Fehler: {message}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let _ = dotenvy::dotenv();
    let api_key = env::var("MARKETSTACK_API_KEY")
        .map_err(|_| "MARKETSTACK_API_KEY fehlt in .env oder der Umgebung.".to_owned())?;
    if api_key.trim().is_empty() {
        return Err("MARKETSTACK_API_KEY ist leer.".to_owned());
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|_| "Der HTTP-Client konnte nicht erstellt werden.".to_owned())?;

    println!("Marketstack-Symbol fuer {ISIN}: {MARKETSTACK_SYMBOL} (Xetra-Listing)");

    println!("Intraday-Abfrage (1-Minuten-Intervall, maximal 10 Datensaetze)");
    print_response(
        &client,
        "intraday",
        &api_key,
        &[
            ("symbols", MARKETSTACK_SYMBOL),
            ("interval", "1min"),
            ("limit", "10"),
        ],
    );

    let first_day_this_month = Utc::now()
        .date_naive()
        .with_day(1)
        .expect("Der erste Monatstag ist gueltig.");
    let last_day_previous_month = first_day_this_month - ChronoDuration::days(1);
    let first_day_previous_month = last_day_previous_month
        .with_day(1)
        .expect("Der erste Tag des Vormonats ist gueltig.");
    let date_from = first_day_previous_month.to_string();
    let date_to = last_day_previous_month.to_string();

    println!("\nTageskurse fuer {MARKETSTACK_SYMBOL} vom {date_from} bis {date_to}");
    print_response(
        &client,
        "eod",
        &api_key,
        &[
            ("symbols", MARKETSTACK_SYMBOL),
            ("date_from", &date_from),
            ("date_to", &date_to),
        ],
    );

    Ok(())
}

fn print_response(
    client: &Client,
    endpoint: &str,
    api_key: &str,
    parameters: &[(&str, &str)],
) {
    let mut request = client
        .get(format!("{API_BASE_URL}/{endpoint}"))
        .query(&[("access_key", api_key)]);
    for (name, value) in parameters {
        request = request.query(&[(*name, *value)]);
    }

    match request.send() {
        Ok(response) => {
            let status = response.status();
            match response.text() {
                Ok(body) => {
                    println!("HTTP {status}");
                    println!("{}", body.replace(api_key, "[REDACTED]"));
                }
                Err(_) => eprintln!("Antwort konnte nicht gelesen werden."),
            }
        }
        Err(_) => eprintln!("Anfrage fehlgeschlagen (Netzwerkfehler oder Timeout)."),
    }
}
