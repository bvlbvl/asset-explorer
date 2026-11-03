use crate::domain::DailyClose;
use async_trait::async_trait;
use chrono::NaiveDate;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::time::Duration;
use thiserror::Error;

const API_BASE_URL: &str = "https://api.marketstack.com/v1";

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Marketstack-Anfrage hat das Zeitlimit ueberschritten.")]
    Timeout,
    #[error("Marketstack ist momentan nicht erreichbar.")]
    Unavailable,
    #[error("Der aktuelle Marketstack-Tarif erlaubt diesen Datenzugriff nicht.")]
    AccessRestricted,
    #[error("Das Marketstack-Anfragelimit wurde erreicht.")]
    RateLimited,
    #[error("Marketstack hat die Anfrage abgelehnt.")]
    Rejected,
    #[error("Marketstack hat eine ungueltige Antwort geliefert.")]
    InvalidResponse,
}

#[async_trait]
pub trait MarketDataProvider: Send + Sync {
    async fn daily_closes(
        &self,
        symbol: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<DailyClose>, ProviderError>;
}

pub struct MarketstackProvider {
    client: Client,
    api_key: String,
}

impl MarketstackProvider {
    pub fn new(api_key: String) -> Result<Self, reqwest::Error> {
        let client = Client::builder().timeout(Duration::from_secs(15)).build()?;

        Ok(Self { client, api_key })
    }
}

#[derive(Deserialize)]
struct MarketstackResponse {
    data: Option<Vec<MarketstackPoint>>,
    error: Option<MarketstackErrorBody>,
}

#[derive(Deserialize)]
struct MarketstackPoint {
    date: String,
    close: Option<f64>,
}

#[derive(Deserialize)]
struct MarketstackErrorBody {
    code: Option<String>,
}

#[async_trait]
impl MarketDataProvider for MarketstackProvider {
    async fn daily_closes(
        &self,
        symbol: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<DailyClose>, ProviderError> {
        let from = from.format("%Y-%m-%d").to_string();
        let to = to.format("%Y-%m-%d").to_string();
        let response = self
            .client
            .get(format!("{API_BASE_URL}/eod"))
            .query(&[
                ("access_key", self.api_key.as_str()),
                ("symbols", symbol),
                ("date_from", from.as_str()),
                ("date_to", to.as_str()),
            ])
            .send()
            .await
            .map_err(|error| {
                if error.is_timeout() {
                    ProviderError::Timeout
                } else {
                    ProviderError::Unavailable
                }
            })?;

        let status = response.status();
        let payload = response
            .json::<MarketstackResponse>()
            .await
            .map_err(|_| ProviderError::InvalidResponse)?;

        if let Some(error) = payload.error {
            return Err(match error.code.as_deref() {
                Some("function_access_restricted") => ProviderError::AccessRestricted,
                Some("rate_limit_reached") | Some("too_many_requests") => {
                    ProviderError::RateLimited
                }
                _ => ProviderError::Rejected,
            });
        }
        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(ProviderError::RateLimited);
        }
        if !status.is_success() {
            return Err(ProviderError::Rejected);
        }

        payload
            .data
            .ok_or(ProviderError::InvalidResponse)?
            .into_iter()
            .map(|point| {
                let date = point
                    .date
                    .get(..10)
                    .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
                    .ok_or(ProviderError::InvalidResponse)?;
                let close = point.close.ok_or(ProviderError::InvalidResponse)?;
                if !close.is_finite() {
                    return Err(ProviderError::InvalidResponse);
                }
                Ok(DailyClose { date, close })
            })
            .collect()
    }
}
