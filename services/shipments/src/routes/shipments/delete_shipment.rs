use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::{EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;
use crate::lib::rest_response::RestResponse;
use entity::shipment::Entity as Shipment;

pub async fn delete_shipment(
    State(s): State<AppState>,
    Path(id): Path<Uuid>
) -> impl IntoResponse {
    let shipment_result = Shipment::find_by_id(id).one(&s.sdb).await.unwrap();
    let shipment = match shipment_result {
        Some(s) => s,
        None => {
            return RestResponse::with_message(
                StatusCode::NOT_FOUND,
                "Shipment not found",
            )
        }
    };

    let delete_result = shipment.delete(&s.sdb).await;
    return match delete_result {
        Ok(_) => RestResponse::<()>::with_message(StatusCode::OK, "Successfully deleted shipment"),
        Err(_) => RestResponse::<()>::with_message(StatusCode::INTERNAL_SERVER_ERROR, "Error while deleting shipment")
    };
}
