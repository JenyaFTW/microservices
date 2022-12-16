use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::AppState;
use crate::lib::rest_response::RestResponse;
use crate::models::shipment::Shipment;

#[derive(Deserialize)]
pub struct CreateShipmentRequest {
    #[serde(rename = "orderId")]
    pub order_id: i32,
}

pub async fn create_shipment(
    State(s): State<AppState>,
    Json(payload): Json<CreateShipmentRequest>,
) -> impl IntoResponse {
    if payload.order_id < 0 {
        return RestResponse::with_message(
            StatusCode::BAD_REQUEST,
            "Order ID must be more than/equal to zero",
        );
    }

    let shipment_result = Shipment::create(&s.db, payload.order_id).await;
    let shipment = match shipment_result {
        Ok(s) => s,
        Err(_) => {
            return RestResponse::with_message(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while creating shipment",
            )
        }
    };

    RestResponse::<Shipment>::with_data(StatusCode::OK, shipment)
}
