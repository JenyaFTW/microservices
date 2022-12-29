use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, NotSet};
use serde::Deserialize;
use entity::shipment;
use entity::shipment::Model;
use crate::AppState;
use crate::lib::rest_response::RestResponse;
use crate::models::shipment::Shipment;

#[derive(Deserialize)]
pub struct CreateShipmentRequest {
    #[serde(rename = "orderId")]
    pub order_id: i64,
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

    let body_result = reqwest::get(format!("http://local-orders:8080/api/orders/get/{}", payload.order_id)).await;
    let body = match body_result {
        Ok(s) => s,
        Err(e) => {
            return RestResponse::with_message(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
            )
        }
    };

    if body.status() == StatusCode::INTERNAL_SERVER_ERROR {
        return RestResponse::with_message(
            StatusCode::NOT_FOUND,
            "Orders service unavailable",
        )
    }

    if body.status() == StatusCode::NOT_FOUND {
        return RestResponse::with_message(
            StatusCode::NOT_FOUND,
            "Order not found",
        )
    }

    let shipment = shipment::ActiveModel {
        order_id: Set(payload.order_id),
        ..Default::default()
    };

    let shipment_result = shipment.insert(&s.sdb).await;
    let shipment = match shipment_result {
        Ok(s) => s,
        Err(e) => {
            return RestResponse::with_message(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
            )
        }
    };

    RestResponse::<Model>::with_data(StatusCode::OK, shipment)
}
