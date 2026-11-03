use crate::domain::{
    MonthlyHistory, is_valid_isin, normalize_isin, previous_calendar_month,
    resolve_known_instrument,
};
use crate::marketstack::{MarketDataProvider, ProviderError};
use chrono::Utc;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HistoryError {
    #[error("Bitte gib eine gueltige ISIN mit korrekter Pruefziffer ein.")]
    InvalidIsin,
    #[error("Fuer diese ISIN ist noch keine Marketstack-Symbolzuordnung hinterlegt.")]
    UnsupportedIsin,
    #[error("Fuer diesen Zeitraum wurden keine Schlusskurse gefunden.")]
    NoData,
    #[error(transparent)]
    Provider(#[from] ProviderError),
}

#[derive(Clone)]
pub struct HistoryService {
    provider: Arc<dyn MarketDataProvider>,
}

impl HistoryService {
    pub fn new(provider: Arc<dyn MarketDataProvider>) -> Self {
        Self { provider }
    }

    pub async fn previous_month(&self, raw_isin: &str) -> Result<MonthlyHistory, HistoryError> {
        let isin = normalize_isin(raw_isin);
        if !is_valid_isin(&isin) {
            return Err(HistoryError::InvalidIsin);
        }

        let instrument = resolve_known_instrument(&isin).ok_or(HistoryError::UnsupportedIsin)?;
        let (from, to) = previous_calendar_month(Utc::now().date_naive());
        let mut points = self
            .provider
            .daily_closes(&instrument.symbol, from, to)
            .await?;
        points.retain(|point| point.date >= from && point.date <= to);
        points.sort_by_key(|point| point.date);
        if points.is_empty() {
            return Err(HistoryError::NoData);
        }

        Ok(MonthlyHistory {
            isin: instrument.isin,
            symbol: instrument.symbol,
            name: instrument.name,
            exchange: instrument.exchange,
            currency: instrument.currency,
            from,
            to,
            points,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{HistoryError, HistoryService};
    use crate::domain::{DailyClose, previous_calendar_month};
    use crate::marketstack::{MarketDataProvider, ProviderError};
    use async_trait::async_trait;
    use chrono::{NaiveDate, Utc};
    use std::sync::Arc;

    struct FakeProvider {
        points: Vec<DailyClose>,
    }

    #[async_trait]
    impl MarketDataProvider for FakeProvider {
        async fn daily_closes(
            &self,
            _symbol: &str,
            _from: NaiveDate,
            _to: NaiveDate,
        ) -> Result<Vec<DailyClose>, ProviderError> {
            Ok(self.points.clone())
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
        let provider = FakeProvider { points: Vec::new() };
        let service = HistoryService::new(Arc::new(provider));

        let result = service.previous_month("US0378331005").await;

        assert!(matches!(result, Err(HistoryError::UnsupportedIsin)));
    }
}
