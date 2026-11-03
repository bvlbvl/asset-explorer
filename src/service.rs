use crate::domain::{
    Instrument, MonthlyHistory, is_valid_isin, normalize_isin, previous_calendar_month,
    resolve_known_instrument,
};
use crate::marketstack::{IsinResolution, MarketDataProvider, ProviderError};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Debug, Error)]
pub enum HistoryError {
    #[error("Bitte gib eine gueltige ISIN mit korrekter Pruefziffer ein.")]
    InvalidIsin,
    #[error("Das gespeicherte Marketstack-Symbol ist ungueltig.")]
    InvalidSavedSymbol { symbol: String },
    #[error("Das gespeicherte Symbol {symbol} wurde fuer die ISIN {isin} nicht bestaetigt.")]
    UnknownSavedSymbol { isin: String, symbol: String },
    #[error("Fuer diese ISIN konnte kein kompatibles Marketstack-Symbol aufgeloest werden.")]
    UnsupportedIsin { stage: &'static str, detail: String },
    #[error("Fuer diesen Zeitraum wurden keine Schlusskurse gefunden.")]
    NoData {
        instrument: Instrument,
        stage: &'static str,
        attempts: Vec<String>,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    },
    #[error("{source}")]
    Provider {
        source: ProviderError,
        stage: &'static str,
        instrument: Option<Instrument>,
    },
}

impl HistoryError {
    pub fn debug_steps(&self) -> Vec<String> {
        match self {
            Self::InvalidIsin => vec![
                "ISIN-Pruefung fehlgeschlagen: Format oder Pruefziffer ist ungueltig.".to_owned(),
            ],
            Self::InvalidSavedSymbol { symbol } => vec![format!(
                "Wiederverwendetes Marketstack-Symbol ist ungueltig: {symbol}."
            )],
            Self::UnknownSavedSymbol { isin, symbol } => vec![format!(
                "Gespeichertes Symbol {symbol} ist fuer ISIN {isin} nicht serverseitig bestaetigt."
            )],
            Self::UnsupportedIsin { stage, detail } => vec![
                "ISIN-Format und Pruefziffer sind gueltig.".to_owned(),
                format!("{stage}: {detail}"),
            ],
            Self::NoData {
                instrument,
                stage,
                attempts,
                from,
                to,
            } => {
                let mut steps = vec![format!(
                    "{stage}: {} ({}){}.",
                    instrument.symbol,
                    instrument.name,
                    if instrument.exchange.is_empty() {
                        String::new()
                    } else {
                        format!(" / {}", instrument.exchange)
                    }
                )];
                steps.extend(attempts.iter().cloned());
                steps.push(format!(
                    "Marketstack EOD: keine Kurspunkte fuer {} bis {}.",
                    from.format("%Y-%m-%d"),
                    to.format("%Y-%m-%d")
                ));
                steps
            }
            Self::Provider {
                source,
                stage,
                instrument,
            } => {
                let mut steps = vec![format!("{stage}: {}", source.diagnostic())];
                if let Some(instrument) = instrument {
                    steps.push(format!(
                        "Aufgeloester Ticker: {} ({}){}.",
                        instrument.symbol,
                        instrument.name,
                        if instrument.exchange.is_empty() {
                            String::new()
                        } else {
                            format!(" / {}", instrument.exchange)
                        }
                    ));
                }
                steps
            }
        }
    }
}

#[derive(Clone)]
pub struct HistoryService {
    provider: Arc<dyn MarketDataProvider>,
    resolved_instruments: Arc<RwLock<HashMap<(String, String), Instrument>>>,
}

impl HistoryService {
    pub fn new(provider: Arc<dyn MarketDataProvider>) -> Self {
        Self {
            provider,
            resolved_instruments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn previous_month(&self, raw_isin: &str) -> Result<MonthlyHistory, HistoryError> {
        self.load_month(raw_isin, None).await
    }

    pub async fn previous_month_with_symbol(
        &self,
        raw_isin: &str,
        symbol: &str,
    ) -> Result<MonthlyHistory, HistoryError> {
        self.load_month(raw_isin, Some(symbol)).await
    }

    async fn load_month(
        &self,
        raw_isin: &str,
        saved_symbol: Option<&str>,
    ) -> Result<MonthlyHistory, HistoryError> {
        let isin = normalize_isin(raw_isin);
        if !is_valid_isin(&isin) {
            return Err(HistoryError::InvalidIsin);
        }
        if let Some(symbol) = saved_symbol.filter(|symbol| !is_valid_marketstack_symbol(symbol)) {
            return Err(HistoryError::InvalidSavedSymbol {
                symbol: symbol.to_owned(),
            });
        }

        let instruments = if let Some(symbol) = saved_symbol {
            let key = (isin.clone(), symbol.to_owned());
            let instrument = self.resolved_instruments.read().await.get(&key).cloned();
            vec![instrument.ok_or_else(|| HistoryError::UnknownSavedSymbol {
                isin: isin.clone(),
                symbol: symbol.to_owned(),
            })?]
        } else {
            match resolve_known_instrument(&isin) {
                Some(instrument) => vec![instrument],
                None => {
                    let resolution = self
                        .provider
                        .resolve_instrument_for_isin(&isin)
                        .await
                        .map_err(|source| {
                            let stage = match source.provider_name() {
                                Some("OpenFIGI") => "OpenFIGI ISIN-Aufloesung",
                                Some("Marketstack Tickers") => {
                                    "Marketstack Listing-Abgleich (/tickers)"
                                }
                                _ => "Ticker-Aufloesung (OpenFIGI + Marketstack Tickers)",
                            };
                            HistoryError::Provider {
                                source,
                                stage,
                                instrument: None,
                            }
                        })?;

                    match resolution {
                        IsinResolution::Found(resolved) => resolved
                            .into_iter()
                            .map(|resolved| Instrument {
                                isin: isin.clone(),
                                symbol: resolved.symbol,
                                name: resolved.name.unwrap_or_else(|| {
                                    "Name von OpenFIGI nicht geliefert".to_owned()
                                }),
                                exchange: resolved.exchange.unwrap_or_else(|| {
                                    "Boerse von Marketstack nicht geliefert".to_owned()
                                }),
                                currency: None,
                            })
                            .collect(),
                        IsinResolution::NotFound { stage, detail } => {
                            return Err(HistoryError::UnsupportedIsin { stage, detail });
                        }
                    }
                }
            }
        };

        let (from, to) = previous_calendar_month(Utc::now().date_naive());
        let Some(preferred_instrument) = instruments.first().cloned() else {
            return Err(HistoryError::UnsupportedIsin {
                stage: "Marketstack Tickers",
                detail: "Es wurden keine EOD-faehigen Listings geliefert.".to_owned(),
            });
        };
        let mut attempts = Vec::new();
        let mut loaded_history = None;

        for instrument in instruments {
            let mut points = self
                .provider
                .daily_closes(&instrument.symbol, from, to)
                .await
                .map_err(|source| HistoryError::Provider {
                    source,
                    stage: if saved_symbol.is_some() {
                        "Kursabruf mit wiederverwendetem Symbol (Marketstack EOD)"
                    } else {
                        "Kursabruf (Marketstack EOD)"
                    },
                    instrument: Some(instrument.clone()),
                })?;
            points.retain(|point| point.date >= from && point.date <= to);
            points.sort_by_key(|point| point.date);
            if points.is_empty() {
                attempts.push(format!(
                    "Marketstack EOD {}: keine Kurspunkte.",
                    instrument.symbol
                ));
                continue;
            }

            attempts.push(format!(
                "Marketstack EOD {}: {} Kurspunkte geladen.",
                instrument.symbol,
                points.len()
            ));
            loaded_history = Some((instrument, points));
            break;
        }

        let Some((instrument, points)) = loaded_history else {
            return Err(HistoryError::NoData {
                instrument: preferred_instrument,
                stage: if saved_symbol.is_some() {
                    "Kursabruf mit wiederverwendetem Symbol (Marketstack EOD)"
                } else {
                    "Marketstack EOD"
                },
                attempts,
                from,
                to,
            });
        };

        if saved_symbol.is_none() {
            self.resolved_instruments.write().await.insert(
                (instrument.isin.clone(), instrument.symbol.clone()),
                instrument.clone(),
            );
        }

        let mut debug = attempts;
        if saved_symbol.is_some() {
            debug.insert(
                0,
                format!(
                    "Kursabruf mit wiederverwendetem Symbol: {}.",
                    instrument.symbol
                ),
            );
        }
        debug.extend([
            format!("Ticker: {}", instrument.symbol),
            format!("Name: {}", instrument.name),
            format!("Boerse/ExchCode: {}", instrument.exchange),
        ]);

        Ok(MonthlyHistory {
            isin: instrument.isin,
            symbol: instrument.symbol,
            name: instrument.name,
            exchange: instrument.exchange,
            currency: instrument.currency,
            from,
            to,
            points,
            debug: Some(debug),
        })
    }
}

fn is_valid_marketstack_symbol(symbol: &str) -> bool {
    !symbol.is_empty()
        && symbol.len() <= 64
        && symbol
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
}

#[cfg(test)]
mod tests {
    use super::{HistoryError, HistoryService};
    use crate::domain::{DailyClose, previous_calendar_month};
    use crate::marketstack::{
        IsinResolution, MarketDataProvider, ProviderError, ResolvedInstrument,
    };
    use async_trait::async_trait;
    use chrono::{NaiveDate, Utc};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct FakeProvider {
        points: Vec<DailyClose>,
        resolved_instruments: Vec<ResolvedInstrument>,
        empty_symbols: Vec<String>,
    }

    #[async_trait]
    impl MarketDataProvider for FakeProvider {
        async fn daily_closes(
            &self,
            _symbol: &str,
            _from: NaiveDate,
            _to: NaiveDate,
        ) -> Result<Vec<DailyClose>, ProviderError> {
            if self.empty_symbols.iter().any(|symbol| symbol == _symbol) {
                Ok(Vec::new())
            } else {
                Ok(self.points.clone())
            }
        }

        async fn resolve_instrument_for_isin(
            &self,
            _isin: &str,
        ) -> Result<IsinResolution, ProviderError> {
            Ok(if self.resolved_instruments.is_empty() {
                IsinResolution::NotFound {
                    stage: "OpenFIGI",
                    detail: "FakeProvider: kein Mapping.".to_owned(),
                }
            } else {
                IsinResolution::Found(self.resolved_instruments.clone())
            })
        }
    }

    struct SavedSymbolProvider {
        resolve_calls: Arc<AtomicUsize>,
        close_calls: Arc<AtomicUsize>,
    }

    struct OpenFigiFailureProvider;

    #[async_trait]
    impl MarketDataProvider for OpenFigiFailureProvider {
        async fn daily_closes(
            &self,
            _symbol: &str,
            _from: NaiveDate,
            _to: NaiveDate,
        ) -> Result<Vec<DailyClose>, ProviderError> {
            panic!("EOD must not be called when ISIN resolution fails");
        }

        async fn resolve_instrument_for_isin(
            &self,
            _isin: &str,
        ) -> Result<IsinResolution, ProviderError> {
            Err(ProviderError::AtProvider {
                provider: "OpenFIGI",
                source: Box::new(ProviderError::Unavailable),
            })
        }
    }

    #[async_trait]
    impl MarketDataProvider for SavedSymbolProvider {
        async fn daily_closes(
            &self,
            symbol: &str,
            from: NaiveDate,
            _to: NaiveDate,
        ) -> Result<Vec<DailyClose>, ProviderError> {
            assert_eq!(symbol, "AAPL");
            self.close_calls.fetch_add(1, Ordering::SeqCst);
            Ok(vec![DailyClose {
                date: from,
                close: 150.0,
            }])
        }

        async fn resolve_instrument_for_isin(
            &self,
            _isin: &str,
        ) -> Result<IsinResolution, ProviderError> {
            self.resolve_calls.fetch_add(1, Ordering::SeqCst);
            Ok(IsinResolution::Found(vec![ResolvedInstrument {
                symbol: "AAPL".to_owned(),
                name: Some("Apple Inc.".to_owned()),
                exchange: Some("NASDAQ".to_owned()),
            }]))
        }
    }

    #[tokio::test]
    async fn returns_sorted_monthly_data_for_a_known_isin() {
        let (from, to) = previous_calendar_month(Utc::now().date_naive());
        let provider = FakeProvider {
            points: vec![
                DailyClose {
                    date: to,
                    close: 102.0,
                },
                DailyClose {
                    date: from,
                    close: 99.0,
                },
            ],
            resolved_instruments: Vec::new(),
            empty_symbols: Vec::new(),
        };
        let service = HistoryService::new(Arc::new(provider));

        let history = service.previous_month("IE00B0M62Q58").await.unwrap();

        assert_eq!(history.symbol, "IQQW.XETRA");
        assert_eq!(history.points.len(), 2);
        assert_eq!(history.points[0].date, from);
        assert_eq!(history.points[1].date, to);
    }

    #[tokio::test]
    async fn rejects_an_isin_without_a_known_symbol_mapping() {
        let provider = FakeProvider {
            points: Vec::new(),
            resolved_instruments: Vec::new(),
            empty_symbols: Vec::new(),
        };
        let service = HistoryService::new(Arc::new(provider));

        let result = service.previous_month("US0378331005").await;

        assert!(matches!(result, Err(HistoryError::UnsupportedIsin { .. })));
    }

    #[tokio::test]
    async fn resolves_symbols_for_valid_isin_without_manual_mapping() {
        let (from, to) = previous_calendar_month(Utc::now().date_naive());
        let provider = FakeProvider {
            points: vec![
                DailyClose {
                    date: from,
                    close: 147.0,
                },
                DailyClose {
                    date: to,
                    close: 150.0,
                },
            ],
            resolved_instruments: vec![ResolvedInstrument {
                symbol: "AAPL".to_owned(),
                name: Some("Apple Inc.".to_owned()),
                exchange: Some("US".to_owned()),
            }],
            empty_symbols: Vec::new(),
        };
        let service = HistoryService::new(Arc::new(provider));

        let history = service.previous_month("US0378331005").await.unwrap();

        assert_eq!(history.symbol, "AAPL");
        assert_eq!(history.name, "Apple Inc.");
        assert!(
            history
                .debug
                .as_ref()
                .unwrap()
                .iter()
                .any(|line| line.contains("AAPL"))
        );
        assert_eq!(history.points.len(), 2);
    }

    #[tokio::test]
    async fn reuses_saved_symbol_without_resolving_isin_again() {
        let resolve_calls = Arc::new(AtomicUsize::new(0));
        let close_calls = Arc::new(AtomicUsize::new(0));
        let service = HistoryService::new(Arc::new(SavedSymbolProvider {
            resolve_calls: Arc::clone(&resolve_calls),
            close_calls: Arc::clone(&close_calls),
        }));

        let first_history = service.previous_month("US0378331005").await.unwrap();
        assert_eq!(first_history.symbol, "AAPL");

        let history = service
            .previous_month_with_symbol("US0378331005", "AAPL")
            .await
            .unwrap();

        assert_eq!(history.isin, "US0378331005");
        assert_eq!(history.symbol, "AAPL");
        assert_eq!(history.name, "Apple Inc.");
        assert_eq!(history.points.len(), 1);
        assert!(
            history
                .debug
                .as_ref()
                .unwrap()
                .iter()
                .any(|step| { step.contains("wiederverwendetem Symbol") && step.contains("AAPL") })
        );
        assert_eq!(resolve_calls.load(Ordering::SeqCst), 1);
        assert_eq!(close_calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn identifies_openfigi_resolution_errors_in_service_diagnostics() {
        let service = HistoryService::new(Arc::new(OpenFigiFailureProvider));

        let error = service.previous_month("US0378331005").await.unwrap_err();

        match error {
            HistoryError::Provider { stage, source, .. } => {
                assert_eq!(stage, "OpenFIGI ISIN-Aufloesung");
                assert!(source.diagnostic().contains("OpenFIGI"));
            }
            other => panic!("expected a provider error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn rejects_saved_symbol_not_previously_resolved_for_isin() {
        let (from, _) = previous_calendar_month(Utc::now().date_naive());
        let provider = FakeProvider {
            points: vec![DailyClose {
                date: from,
                close: 150.0,
            }],
            resolved_instruments: Vec::new(),
            empty_symbols: Vec::new(),
        };
        let service = HistoryService::new(Arc::new(provider));

        let result = service
            .previous_month_with_symbol("US0378331005", "MSFT")
            .await;

        assert!(
            result.is_err(),
            "unverified symbols must not reach Marketstack"
        );
    }

    #[tokio::test]
    async fn reports_resolved_ticker_when_marketstack_returns_no_data() {
        let provider = FakeProvider {
            points: Vec::new(),
            resolved_instruments: vec![ResolvedInstrument {
                symbol: "IWDA.AS".to_owned(),
                name: Some("iShares Core MSCI World UCITS ETF".to_owned()),
                exchange: Some("NA".to_owned()),
            }],
            empty_symbols: Vec::new(),
        };
        let service = HistoryService::new(Arc::new(provider));

        let error = service.previous_month("IE00B3XXRP09").await.unwrap_err();

        match error {
            HistoryError::NoData { instrument, .. } => {
                assert_eq!(instrument.symbol, "IWDA.AS");
                assert_eq!(instrument.name, "iShares Core MSCI World UCITS ETF");
            }
            other => panic!("expected no-data diagnostic, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn falls_back_to_an_eod_listing_when_preferred_listing_has_no_data() {
        let (from, _) = previous_calendar_month(Utc::now().date_naive());
        let provider = FakeProvider {
            points: vec![DailyClose {
                date: from,
                close: 102.0,
            }],
            resolved_instruments: vec![
                ResolvedInstrument {
                    symbol: "VUSA.XLON".to_owned(),
                    name: Some("Vanguard S&P 500 UCITS ETF".to_owned()),
                    exchange: Some("XLON".to_owned()),
                },
                ResolvedInstrument {
                    symbol: "VUSA.XFRA".to_owned(),
                    name: Some("Vanguard S&P 500 UCITS ETF".to_owned()),
                    exchange: Some("XFRA".to_owned()),
                },
            ],
            empty_symbols: vec!["VUSA.XLON".to_owned()],
        };
        let service = HistoryService::new(Arc::new(provider));

        let history = service.previous_month("IE00B3XXRP09").await.unwrap();

        assert_eq!(history.symbol, "VUSA.XFRA");
        let debug = history.debug.as_ref().unwrap();
        assert!(debug.iter().any(|step| step.contains("VUSA.XLON: keine")));
        assert!(debug.iter().any(|step| step.contains("VUSA.XFRA: 1")));
    }
}
