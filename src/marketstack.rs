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
    #[error("{provider} hat die Anfrage mit HTTP {status} abgelehnt.")]
    DetailedRejected {
        provider: &'static str,
        status: u16,
        code: Option<String>,
    },
    #[error("{provider}: {source}")]
    AtProvider {
        provider: &'static str,
        #[source]
        source: Box<ProviderError>,
    },
    #[error("Marketstack hat eine ungueltige Antwort geliefert.")]
    InvalidResponse,
}

impl ProviderError {
    pub fn diagnostic(&self) -> String {
        match self {
            Self::Timeout => "Zeitlimit von 15 Sekunden ueberschritten.".to_owned(),
            Self::Unavailable => "Verbindung zum Anbieter fehlgeschlagen.".to_owned(),
            Self::AccessRestricted => "Marketstack meldet function_access_restricted.".to_owned(),
            Self::RateLimited => "Anbieter-Anfragelimit erreicht.".to_owned(),
            Self::DetailedRejected {
                provider,
                status,
                code,
            } => format!(
                "{provider} antwortete mit HTTP {status}{}.",
                code.as_deref()
                    .map(|code| format!(" (Fehlercode: {code})"))
                    .unwrap_or_default()
            ),
            Self::AtProvider { provider, source } => {
                format!("{provider}: {}", source.diagnostic())
            }
            Self::InvalidResponse => "Antwort des Anbieters hat ein ungueltiges Format.".to_owned(),
        }
    }

    pub fn provider_name(&self) -> Option<&'static str> {
        match self {
            Self::AtProvider { provider, .. } | Self::DetailedRejected { provider, .. } => {
                Some(provider)
            }
            _ => None,
        }
    }

    pub fn root_cause(&self) -> &Self {
        match self {
            Self::AtProvider { source, .. } => source.root_cause(),
            _ => self,
        }
    }
}

fn at_provider(provider: &'static str, source: ProviderError) -> ProviderError {
    ProviderError::AtProvider {
        provider,
        source: Box::new(source),
    }
}

#[derive(Clone, Debug)]
pub struct ResolvedInstrument {
    pub symbol: String,
    pub name: Option<String>,
    pub exchange: Option<String>,
}

#[derive(Clone, Debug)]
pub enum IsinResolution {
    Found(Vec<ResolvedInstrument>),
    NotFound { stage: &'static str, detail: String },
}

#[async_trait]
pub trait MarketDataProvider: Send + Sync {
    async fn daily_closes(
        &self,
        symbol: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<DailyClose>, ProviderError>;

    async fn resolve_instrument_for_isin(
        &self,
        _isin: &str,
    ) -> Result<IsinResolution, ProviderError> {
        Ok(IsinResolution::NotFound {
            stage: "OpenFIGI",
            detail: "Kein ISIN-Resolver konfiguriert.".to_owned(),
        })
    }
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
struct OpenfigiMappingResponse {
    data: Option<Vec<OpenfigiMappingResult>>,
    warning: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct OpenfigiMappingResult {
    ticker: Option<String>,
    name: Option<String>,
    #[serde(rename = "exchCode")]
    exch_code: Option<String>,
}

#[derive(Deserialize)]
struct MarketstackTickerResponse {
    data: Option<Vec<MarketstackTicker>>,
    error: Option<MarketstackErrorBody>,
}

#[derive(Clone, Deserialize)]
struct MarketstackTicker {
    symbol: Option<String>,
    name: Option<String>,
    has_eod: Option<bool>,
    stock_exchange: Option<MarketstackTickerExchange>,
}

#[derive(Clone, Deserialize)]
struct MarketstackTickerExchange {
    mic: Option<String>,
}

#[derive(Deserialize)]
struct MarketstackErrorBody {
    code: Option<String>,
}

fn select_marketstack_listings(
    ticker: &str,
    openfigi_exchange: Option<&str>,
    openfigi_name: Option<&str>,
    listings: Vec<MarketstackTicker>,
) -> Result<Vec<ResolvedInstrument>, String> {
    let eod_listings: Vec<_> = listings
        .into_iter()
        .filter(|listing| {
            listing.has_eod == Some(true)
                && listing.symbol.as_deref().is_some_and(|symbol| {
                    symbol
                        .split_once('.')
                        .map_or(symbol, |(base, _)| base)
                        .eq_ignore_ascii_case(ticker)
                })
        })
        .collect();

    if eod_listings.is_empty() {
        return Err(format!(
            "Marketstack /tickers lieferte kein EOD-faehiges Listing fuer {ticker}."
        ));
    }

    let target_mic = openfigi_exchange.and_then(marketstack_mic_for_openfigi);
    let primary_index = if let Some(target_mic) = target_mic {
        eod_listings
            .iter()
            .position(|listing| {
                listing
                    .stock_exchange
                    .as_ref()
                    .and_then(|exchange| exchange.mic.as_deref())
                    .is_some_and(|mic| mic.eq_ignore_ascii_case(target_mic))
            })
            .ok_or_else(|| {
                format!(
                    "OpenFIGI exchCode {} entspricht Marketstack-MIC {target_mic}, aber dieses Listing wurde nicht gefunden. EOD-Listings: {}.",
                    openfigi_exchange.unwrap_or_default(),
                    format_listing_symbols(&eod_listings)
                )
            })?
    } else if eod_listings.len() == 1 {
        0
    } else {
        return Err(format!(
            "OpenFIGI exchCode {} ist keinem Marketstack-MIC zugeordnet und mehrere EOD-Listings sind verfuegbar: {}.",
            openfigi_exchange.unwrap_or("nicht geliefert"),
            format_listing_symbols(&eod_listings)
        ));
    };

    let mut ordered_listings = eod_listings;
    let primary_listing = ordered_listings.remove(primary_index);
    ordered_listings.insert(0, primary_listing);

    Ok(ordered_listings
        .into_iter()
        .map(|listing| {
            let symbol = listing.symbol.unwrap_or_default();
            let exchange = listing
                .stock_exchange
                .and_then(|exchange| exchange.mic)
                .or_else(|| openfigi_exchange.map(str::to_owned));

            ResolvedInstrument {
                symbol,
                name: listing.name.or_else(|| openfigi_name.map(str::to_owned)),
                exchange,
            }
        })
        .collect())
}

fn marketstack_mic_for_openfigi(exchange: &str) -> Option<&'static str> {
    match exchange {
        "LN" => Some("XLON"),
        _ => None,
    }
}

fn format_listing_symbols(listings: &[MarketstackTicker]) -> String {
    listings
        .iter()
        .filter_map(|listing| listing.symbol.as_deref())
        .collect::<Vec<_>>()
        .join(", ")
}

impl MarketstackProvider {
    async fn search_marketstack_tickers(
        &self,
        ticker: &str,
    ) -> Result<Vec<MarketstackTicker>, ProviderError> {
        let response = self
            .client
            .get(format!("{API_BASE_URL}/tickers"))
            .query(&[
                ("access_key", self.api_key.as_str()),
                ("search", ticker),
                ("limit", "100"),
            ])
            .send()
            .await
            .map_err(|error| {
                let source = if error.is_timeout() {
                    ProviderError::Timeout
                } else {
                    ProviderError::Unavailable
                };
                at_provider("OpenFIGI", source)
            })?;

        let status = response.status();
        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(at_provider("OpenFIGI", ProviderError::RateLimited));
        }
        if !status.is_success() {
            let code = response
                .json::<MarketstackTickerResponse>()
                .await
                .ok()
                .and_then(|payload| payload.error)
                .and_then(|error| error.code);
            return Err(ProviderError::DetailedRejected {
                provider: "Marketstack Tickers",
                status: status.as_u16(),
                code,
            });
        }

        let payload = response
            .json::<MarketstackTickerResponse>()
            .await
            .map_err(|_| ProviderError::InvalidResponse)?;
        if let Some(error) = payload.error {
            return Err(ProviderError::DetailedRejected {
                provider: "Marketstack Tickers",
                status: status.as_u16(),
                code: error.code,
            });
        }

        Ok(payload.data.unwrap_or_default())
    }
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
        let payload = response.json::<MarketstackResponse>().await.map_err(|_| {
            if status.is_success() {
                ProviderError::InvalidResponse
            } else {
                ProviderError::DetailedRejected {
                    provider: "Marketstack",
                    status: status.as_u16(),
                    code: None,
                }
            }
        })?;

        if let Some(error) = payload.error {
            return Err(match error.code.as_deref() {
                Some("function_access_restricted") => ProviderError::AccessRestricted,
                Some("rate_limit_reached") | Some("too_many_requests") => {
                    ProviderError::RateLimited
                }
                _ => ProviderError::DetailedRejected {
                    provider: "Marketstack",
                    status: status.as_u16(),
                    code: error.code,
                },
            });
        }
        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(ProviderError::RateLimited);
        }
        if !status.is_success() {
            return Err(ProviderError::DetailedRejected {
                provider: "Marketstack",
                status: status.as_u16(),
                code: None,
            });
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

    async fn resolve_instrument_for_isin(
        &self,
        isin: &str,
    ) -> Result<IsinResolution, ProviderError> {
        let response = self
            .client
            .post("https://api.openfigi.com/v3/mapping")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!([{ "idType": "ID_ISIN", "idValue": isin }]))
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
        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(ProviderError::RateLimited);
        }
        if !status.is_success() {
            return Err(ProviderError::DetailedRejected {
                provider: "OpenFIGI",
                status: status.as_u16(),
                code: None,
            });
        }

        let payload = response
            .json::<Vec<OpenfigiMappingResponse>>()
            .await
            .map_err(|_| at_provider("OpenFIGI", ProviderError::InvalidResponse))?;
        let Some(job) = payload.into_iter().next() else {
            return Ok(IsinResolution::NotFound {
                stage: "OpenFIGI",
                detail: "OpenFIGI lieferte kein Mapping-Ergebnis.".to_owned(),
            });
        };
        if let Some(error) = job.error {
            return Ok(IsinResolution::NotFound {
                stage: "OpenFIGI",
                detail: error,
            });
        }

        let data = job.data.unwrap_or_default();
        let Some(entry) = data.into_iter().find(|entry| {
            entry
                .ticker
                .as_deref()
                .is_some_and(|ticker| !ticker.trim().is_empty())
        }) else {
            return Ok(IsinResolution::NotFound {
                stage: "OpenFIGI",
                detail: job
                    .warning
                    .unwrap_or_else(|| "OpenFIGI fand kein Wertpapier zur ISIN.".to_owned()),
            });
        };

        let ticker = entry.ticker.unwrap_or_default();
        let exchange = entry.exch_code;
        let listings = self
            .search_marketstack_tickers(&ticker)
            .await
            .map_err(|source| at_provider("Marketstack Tickers", source))?;
        match select_marketstack_listings(
            &ticker,
            exchange.as_deref(),
            entry.name.as_deref(),
            listings,
        ) {
            Ok(instruments) => Ok(IsinResolution::Found(instruments)),
            Err(detail) => Ok(IsinResolution::NotFound {
                stage: "Marketstack Tickers",
                detail,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MarketstackTickerResponse, OpenfigiMappingResponse, ProviderError,
        select_marketstack_listings,
    };

    #[test]
    fn attributes_transient_errors_to_the_provider_stage() {
        let error = ProviderError::AtProvider {
            provider: "OpenFIGI",
            source: Box::new(ProviderError::Unavailable),
        };

        assert_eq!(
            error.diagnostic(),
            "OpenFIGI: Verbindung zum Anbieter fehlgeschlagen."
        );
    }

    #[test]
    fn parses_openfigi_ticker_from_isin_mapping_response() {
        let payload: Vec<OpenfigiMappingResponse> = serde_json::from_str(
            r#"[
                {
                    "data": [
                        {
                            "figi": "BBG000BLNNH6",
                            "ticker": "IBM",
                            "name": "INTL BUSINESS MACHINES CORP",
                            "exchCode": "US"
                        }
                    ]
                }
            ]"#,
        )
        .unwrap();

        let ticker = payload
            .into_iter()
            .flat_map(|job| job.data.unwrap_or_default())
            .find_map(|entry| entry.ticker)
            .unwrap();

        assert_eq!(ticker, "IBM");
    }

    #[test]
    fn parses_openfigi_name_and_exchange_code() {
        let payload: Vec<OpenfigiMappingResponse> = serde_json::from_str(
            r#"[
                {
                    "data": [
                        {
                            "ticker": "IBM",
                            "name": "INTL BUSINESS MACHINES CORP",
                            "exchCode": "US"
                        }
                    ]
                }
            ]"#,
        )
        .unwrap();

        let entry = payload.into_iter().next().unwrap().data.unwrap().remove(0);

        assert_eq!(entry.name.as_deref(), Some("INTL BUSINESS MACHINES CORP"));
        assert_eq!(entry.exch_code.as_deref(), Some("US"));
    }

    #[test]
    fn selects_eod_listing_matching_openfigi_exchange() {
        let payload: MarketstackTickerResponse = serde_json::from_str(
            r#"{
                "data": [
                    {
                        "symbol": "VUSA",
                        "name": "Vanguard S&P 500 UCITS ETF",
                        "has_eod": false,
                        "stock_exchange": {"mic": "IEXG"}
                    },
                    {
                        "symbol": "VUSA.XETRA",
                        "name": "Vanguard S&P 500 UCITS ETF",
                        "has_eod": true,
                        "stock_exchange": {"mic": "XETRA"}
                    },
                    {
                        "symbol": "VUSA.XLON",
                        "name": "Vanguard S&P 500 UCITS ETF",
                        "has_eod": true,
                        "stock_exchange": {"mic": "XLON"}
                    }
                ]
            }"#,
        )
        .unwrap();

        let selected = select_marketstack_listings(
            "VUSA",
            Some("LN"),
            Some("VANG S&P500 USDD"),
            payload.data.unwrap(),
        )
        .unwrap();

        assert_eq!(selected[0].symbol, "VUSA.XLON");
        assert_eq!(selected[0].exchange.as_deref(), Some("XLON"));
    }
}
