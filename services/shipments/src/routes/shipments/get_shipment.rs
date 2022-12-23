use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;
use crate::lib::rest_response::RestResponse;
use entity::shipment::{Entity as Shipment, Model};

pub async fn get_shipment(
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

    RestResponse::<Model>::with_data(StatusCode::OK, shipment)
}
