use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::lib::rest_response::RestResponse;
use crate::models::shipment::Shipment;

pub async fn get_shipments(
    State(s): State<AppState>,
) -> impl IntoResponse {
    let shipments_result = Shipment::get_all(&s.db).await;
    let shipments = match shipments_result {
        Ok(s) => s,
        Err(_) => {
            return RestResponse::with_message(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error fetching all shipments",
            )
        }
    };

    RestResponse::<Vec<Shipment>>::with_data(StatusCode::OK, shipments)
}
