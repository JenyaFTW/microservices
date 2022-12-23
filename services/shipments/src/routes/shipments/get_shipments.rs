use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::AppState;
use crate::lib::rest_response::RestResponse;
use entity::shipment::Entity as Shipment;
use entity::shipment::Model;

pub async fn get_shipments(
    State(s): State<AppState>,
) -> impl IntoResponse {
    let shipments_result = Shipment::find().all(&s.sdb).await;
    let shipments = match shipments_result {
        Ok(s) => s,
        Err(_) => {
            return RestResponse::with_message(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error fetching all shipments",
            )
        }
    };

    RestResponse::<Vec<Model>>::with_data(StatusCode::OK, shipments)
}
