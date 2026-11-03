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
}

async fn get_history(
    State(state): State<ApiState>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<MonthlyHistory>, ApiError> {
    match state.service.previous_month(&query.isin).await {
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
    debug: Option<Vec<String>>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let debug = self.debug_output.then(|| self.error.debug_steps());
        let (status, code, message) = match self.error {
            HistoryError::InvalidIsin => (
                StatusCode::BAD_REQUEST,
                "invalid_isin",
                HistoryError::InvalidIsin.to_string(),
            ),
            HistoryError::UnsupportedIsin { .. } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "unsupported_isin",
                "Fuer diese ISIN konnte kein kompatibles Marketstack-Symbol aufgeloest werden."
                    .to_owned(),
            ),
            HistoryError::NoData { .. } => (
                StatusCode::NOT_FOUND,
                "no_data",
                "Fuer diesen Zeitraum wurden keine Schlusskurse gefunden.".to_owned(),
            ),
            HistoryError::Provider { source, .. } => match source {
                crate::marketstack::ProviderError::AccessRestricted => (
                    StatusCode::BAD_GATEWAY,
                    "provider_plan_restricted",
                    "Marketstack-Tarif erlaubt diesen Datenzugriff nicht.".to_owned(),
                ),
                crate::marketstack::ProviderError::RateLimited => (
                    StatusCode::TOO_MANY_REQUESTS,
                    "provider_rate_limited",
                    "Marketstack hat das Anfragelimit erreicht. Bitte spaeter erneut versuchen."
                        .to_owned(),
                ),
                crate::marketstack::ProviderError::DetailedRejected { provider, .. }
                    if provider == "Marketstack" =>
                {
                    (
                        StatusCode::BAD_GATEWAY,
                        "provider_rejected",
                        "Marketstack hat die Kursdatenanfrage abgelehnt.".to_owned(),
                    )
                }
                _ => (
                    StatusCode::BAD_GATEWAY,
                    "provider_unavailable",
                    "Die Marktdaten konnten gerade nicht geladen werden.".to_owned(),
                ),
            },
        };

        (
            status,
            Json(ErrorEnvelope {
                error: ErrorDetail {
                    code,
                    message,
                    debug,
                },
            }),
        )
            .into_response()
    }
}
