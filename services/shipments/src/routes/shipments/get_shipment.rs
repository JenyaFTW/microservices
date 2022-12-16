use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;
use crate::lib::rest_response::RestResponse;
use crate::models::shipment::Shipment;

pub async fn get_shipment(
    State(s): State<AppState>,
    Path(id): Path<Uuid>
) -> impl IntoResponse {
    let shipment_result = Shipment::get(&s.db, id).await;
    let shipment = match shipment_result {
        Ok(s) => s,
        Err(_) => {
            return RestResponse::with_message(
                StatusCode::NOT_FOUND,
                "Shipment not found",
            )
        }
    };

    RestResponse::<Shipment>::with_data(StatusCode::OK, shipment)
}
