use crate::domain::MonthlyHistory;
use crate::service::{HistoryError, HistoryService};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

pub fn router(service: HistoryService) -> Router {
    Router::new()
        .route("/api/history", get(get_history))
        .fallback_service(ServeDir::new("public"))
        .with_state(service)
}

#[derive(Deserialize)]
struct HistoryQuery {
    isin: String,
}

async fn get_history(
    State(service): State<HistoryService>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<MonthlyHistory>, ApiError> {
    service
        .previous_month(&query.isin)
        .await
        .map(Json)
        .map_err(ApiError)
}

struct ApiError(HistoryError);

#[derive(Serialize)]
struct ErrorEnvelope {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self.0 {
            HistoryError::InvalidIsin => (
                StatusCode::BAD_REQUEST,
                "invalid_isin",
                HistoryError::InvalidIsin.to_string(),
            ),
            HistoryError::UnsupportedIsin => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "unsupported_isin",
                HistoryError::UnsupportedIsin.to_string(),
            ),
            HistoryError::NoData => (
                StatusCode::NOT_FOUND,
                "no_data",
                HistoryError::NoData.to_string(),
            ),
            HistoryError::Provider(error) => match error {
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
                error: ErrorDetail { code, message },
            }),
        )
            .into_response()
    }
}
