use crate::domain::MonthlyHistory;
use crate::service::{HistoryError, HistoryService};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

pub fn router(service: HistoryService, debug_output: bool) -> Router {
    Router::new()
        .route("/api/history", get(get_history))
        .fallback_service(ServeDir::new("public"))
        .with_state(ApiState {
            service,
            debug_output,
        })
}

#[derive(Clone)]
struct ApiState {
    service: HistoryService,
    debug_output: bool,
}

#[derive(Deserialize)]
struct HistoryQuery {
    isin: String,
    #[serde(default)]
    symbol: Option<String>,
}

async fn get_history(
    State(state): State<ApiState>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<MonthlyHistory>, ApiError> {
    let result = match query.symbol.as_deref() {
        Some(symbol) => match state
                .service
                .previous_month_with_symbol(&query.isin, symbol)
                .await
            {
                Err(HistoryError::UnknownSavedSymbol { .. }) => state
                    .service
                    .previous_month(&query.isin)
                    .await
                    .map(|mut history| {
                        if let Some(steps) = history.debug.as_mut() {
                            steps.insert(
                                0,
                                "Gespeicherter Ticker nicht im Backend-Cache; ISIN normal erneut verarbeitet."
                                    .to_owned(),
                            );
                        }
                        history
                    }),
                result => result,
            },
        None => state.service.previous_month(&query.isin).await,
    };

    match result {
        Ok(mut history) => {
            if !state.debug_output {
                history.debug = None;
            }
            Ok(Json(history))
        }
        Err(error) => Err(ApiError {
            error,
            debug_output: state.debug_output,
        }),
    }
}

struct ApiError {
    error: HistoryError,
    debug_output: bool,
}

#[derive(Serialize)]
struct ErrorEnvelope {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    debug: Option<Vec<String>>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let debug = self.debug_output.then(|| self.error.debug_steps());
        let (status, code, message, stage) =
            match &self.error {
                HistoryError::InvalidIsin => (
                    StatusCode::BAD_REQUEST,
                    "invalid_isin",
                    HistoryError::InvalidIsin.to_string(),
                    None,
                ),
                HistoryError::InvalidSavedSymbol { symbol } => (
                    StatusCode::BAD_REQUEST,
                    "invalid_saved_symbol",
                    format!("Das gespeicherte Marketstack-Symbol {symbol} ist ungueltig."),
                    Some("Validierung des gespeicherten Symbols"),
                ),
                HistoryError::UnknownSavedSymbol { isin, symbol } => (
                    StatusCode::BAD_REQUEST,
                    "unverified_saved_symbol",
                    format!(
                        "Gespeichertes Symbol {symbol} wurde fuer ISIN {isin} nicht bestaetigt."
                    ),
                    Some("Validierung des gespeicherten ISIN-/Symbol-Paars"),
                ),
                HistoryError::UnsupportedIsin { stage, detail } => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    match *stage {
                        "OpenFIGI" => "openfigi_mapping_failed",
                        "Marketstack Tickers" => "marketstack_listing_not_found",
                        _ => "ticker_resolution_failed",
                    },
                    format!("Ticker-/Listing-Aufloesung bei {stage} fehlgeschlagen: {detail}"),
                    Some(*stage),
                ),
                HistoryError::NoData {
                    instrument, stage, ..
                } => (
                    StatusCode::NOT_FOUND,
                    if stage.contains("wiederverwendetem Symbol") {
                        "saved_symbol_no_data"
                    } else {
                        "no_data"
                    },
                    format!(
                        "{stage}: fuer Symbol {} wurden im Zeitraum keine Schlusskurse gefunden.",
                        instrument.symbol
                    ),
                    Some(*stage),
                ),
                HistoryError::Provider { source, .. } => {
                    let provider = source.provider_name();
                    match source.root_cause() {
                        crate::marketstack::ProviderError::AccessRestricted => (
                            StatusCode::BAD_GATEWAY,
                            "provider_plan_restricted",
                            source.to_string(),
                            None,
                        ),
                        crate::marketstack::ProviderError::RateLimited => (
                            StatusCode::TOO_MANY_REQUESTS,
                            match provider {
                                Some("OpenFIGI") => "openfigi_rate_limited",
                                Some("Marketstack Tickers") => "marketstack_tickers_rate_limited",
                                _ => "provider_rate_limited",
                            },
                            source.to_string(),
                            None,
                        ),
                        crate::marketstack::ProviderError::DetailedRejected {
                            provider, ..
                        } if *provider == "Marketstack Tickers" => (
                            StatusCode::BAD_GATEWAY,
                            "marketstack_tickers_rejected",
                            source.to_string(),
                            None,
                        ),
                        crate::marketstack::ProviderError::DetailedRejected {
                            provider, ..
                        } if *provider == "OpenFIGI" => (
                            StatusCode::BAD_GATEWAY,
                            "openfigi_rejected",
                            source.to_string(),
                            None,
                        ),
                        crate::marketstack::ProviderError::DetailedRejected {
                            provider, ..
                        } if *provider == "Marketstack" => (
                            StatusCode::BAD_GATEWAY,
                            "provider_rejected",
                            source.to_string(),
                            None,
                        ),
                        _ => (
                            StatusCode::BAD_GATEWAY,
                            match provider {
                                Some("OpenFIGI") => "openfigi_provider_unavailable",
                                Some("Marketstack Tickers") => "marketstack_tickers_unavailable",
                                _ => "provider_unavailable",
                            },
                            source.to_string(),
                            None,
                        ),
                    }
                }
            };

        let (status, code, message, stage) = match (&self.error, status, code, message, stage) {
            (
                HistoryError::Provider {
                    stage: error_stage,
                    instrument,
                    ..
                },
                status,
                code,
                message,
                _,
            ) => {
                let saved_symbol = error_stage.contains("wiederverwendetem Symbol");
                let code = if saved_symbol {
                    match code {
                        "provider_plan_restricted" => "saved_symbol_plan_restricted",
                        "provider_rate_limited" => "saved_symbol_rate_limited",
                        "provider_rejected" => "saved_symbol_rejected",
                        _ => "saved_symbol_provider_unavailable",
                    }
                } else {
                    code
                };
                let symbol = instrument
                    .as_ref()
                    .map(|instrument| format!(" fuer Symbol {}", instrument.symbol))
                    .unwrap_or_default();
                (
                    status,
                    code,
                    format!("{error_stage}{symbol}: {message}"),
                    Some(*error_stage),
                )
            }
            (_, status, code, message, stage) => (status, code, message, stage),
        };

        (
            status,
            Json(ErrorEnvelope {
                error: ErrorDetail {
                    code,
                    message,
                    stage,
                    debug,
                },
            }),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiError, ApiState, HistoryQuery, get_history};
    use crate::domain::{DailyClose, Instrument};
    use crate::marketstack::{IsinResolution, MarketDataProvider, ProviderError};
    use crate::service::{HistoryError, HistoryService};
    use async_trait::async_trait;
    use axum::body::to_bytes;
    use axum::extract::{Query, State};
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use chrono::NaiveDate;
    use serde_json::Value;
    use std::sync::Arc;

    struct SavedSymbolProvider;

    #[async_trait]
    impl MarketDataProvider for SavedSymbolProvider {
        async fn daily_closes(
            &self,
            symbol: &str,
            from: NaiveDate,
            _to: NaiveDate,
        ) -> Result<Vec<DailyClose>, ProviderError> {
            assert_eq!(symbol, "IQQW.XETRA");
            Ok(vec![DailyClose {
                date: from,
                close: 150.0,
            }])
        }

        async fn resolve_instrument_for_isin(
            &self,
            _isin: &str,
        ) -> Result<IsinResolution, ProviderError> {
            panic!("a saved symbol must bypass ISIN resolution");
        }
    }

    #[tokio::test]
    async fn uses_saved_symbol_query_without_resolving_isin_again() {
        let service = HistoryService::new(Arc::new(SavedSymbolProvider));
        service.previous_month("IE00B0M62Q58").await.unwrap();
        let state = ApiState {
            service,
            debug_output: true,
        };

        let response = get_history(
            State(state),
            Query(HistoryQuery {
                isin: "IE00B0M62Q58".to_owned(),
                symbol: Some("IQQW.XETRA".to_owned()),
            }),
        )
        .await
        .ok()
        .unwrap();

        assert_eq!(response.0.symbol, "IQQW.XETRA");
        assert_eq!(response.0.points.len(), 1);
        assert!(response.0.debug.unwrap().iter().any(|step| {
            step.contains("wiederverwendetem Symbol") && step.contains("IQQW.XETRA")
        }));
    }

    #[tokio::test]
    async fn falls_back_to_isin_resolution_when_saved_symbol_is_not_in_server_cache() {
        let state = ApiState {
            service: HistoryService::new(Arc::new(SavedSymbolProvider)),
            debug_output: true,
        };

        let response = get_history(
            State(state),
            Query(HistoryQuery {
                isin: "IE00B0M62Q58".to_owned(),
                symbol: Some("MSFT".to_owned()),
            }),
        )
        .await;

        assert!(response.is_ok(), "cache misses must safely resolve by ISIN");
        let history = response.ok().unwrap().0;
        assert_eq!(history.symbol, "IQQW.XETRA");
        assert!(history.debug.unwrap().iter().any(|step| {
            step.contains("Ticker nicht im Backend-Cache")
                && step.contains("normal erneut verarbeitet")
        }));
    }

    #[tokio::test]
    async fn identifies_saved_symbol_provider_errors_by_stage_and_symbol() {
        let response = ApiError {
            error: HistoryError::Provider {
                source: ProviderError::Unavailable,
                stage: "Kursabruf mit wiederverwendetem Symbol (Marketstack EOD)",
                instrument: Some(Instrument {
                    isin: "US0378331005".to_owned(),
                    symbol: "AAPL".to_owned(),
                    name: "AAPL".to_owned(),
                    exchange: String::new(),
                    currency: None,
                }),
            },
            debug_output: true,
        }
        .into_response();

        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let payload: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            payload["error"]["code"],
            "saved_symbol_provider_unavailable"
        );
        assert_eq!(
            payload["error"]["stage"],
            "Kursabruf mit wiederverwendetem Symbol (Marketstack EOD)"
        );
        assert!(
            payload["error"]["message"]
                .as_str()
                .unwrap()
                .contains("AAPL")
        );
        assert!(payload["error"]["debug"].to_string().contains("AAPL"));
    }

    #[tokio::test]
    async fn identifies_no_data_for_a_saved_symbol() {
        let response = ApiError {
            error: HistoryError::NoData {
                instrument: Instrument {
                    isin: "US0378331005".to_owned(),
                    symbol: "AAPL".to_owned(),
                    name: "AAPL".to_owned(),
                    exchange: String::new(),
                    currency: None,
                },
                stage: "Kursabruf mit wiederverwendetem Symbol (Marketstack EOD)",
                attempts: vec!["Marketstack EOD AAPL: keine Kurspunkte.".to_owned()],
                from: NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
                to: NaiveDate::from_ymd_opt(2026, 10, 31).unwrap(),
            },
            debug_output: false,
        }
        .into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let payload: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(payload["error"]["code"], "saved_symbol_no_data");
        assert_eq!(
            payload["error"]["stage"],
            "Kursabruf mit wiederverwendetem Symbol (Marketstack EOD)"
        );
        assert!(
            payload["error"]["message"]
                .as_str()
                .unwrap()
                .contains("AAPL")
        );
    }

    #[tokio::test]
    async fn identifies_marketstack_listing_resolution_failure() {
        let response = ApiError {
            error: HistoryError::UnsupportedIsin {
                stage: "Marketstack Tickers",
                detail: "mehrere Listings ohne verifizierte Boersenzuordnung".to_owned(),
            },
            debug_output: false,
        }
        .into_response();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let payload: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(payload["error"]["code"], "marketstack_listing_not_found");
        assert_eq!(payload["error"]["stage"], "Marketstack Tickers");
        assert!(
            payload["error"]["message"]
                .as_str()
                .unwrap()
                .contains("mehrere Listings")
        );
    }
}
