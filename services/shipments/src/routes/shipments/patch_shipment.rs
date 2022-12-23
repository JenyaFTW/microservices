use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;
use crate::lib::rest_response::RestResponse;
use entity::sea_orm_active_enums::Status;
use entity::shipment::{ActiveModel, Entity as Shipment, Model};

#[derive(Serialize, Deserialize)]
pub struct PatchShipmentRequest {
    status: Status
}

pub async fn patch_shipment(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<PatchShipmentRequest>
) -> impl IntoResponse {
    let shipment_result = Shipment::find_by_id(id).one(&s.sdb).await.unwrap();
    let mut shipment: ActiveModel = match shipment_result {
        Some(s) => s.into(),
        None => {
            return RestResponse::with_message(
                StatusCode::NOT_FOUND,
                "Shipment not found",
            )
        }
    };

    shipment.shipping_status = Set(payload.status);

    let shipment_result = shipment.update(&s.sdb).await;
    let shipment = match shipment_result {
        Ok(s) => s,
        Err(e) => {
            return RestResponse::with_message(
                StatusCode::NOT_FOUND,
                &e.to_string(),
            )
        }
    };

    RestResponse::<Model>::with_data(StatusCode::OK, shipment)
}
