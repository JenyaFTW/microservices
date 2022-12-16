use crate::routes::AppState;
use axum::{routing::{post, delete, get, patch}, Router};

pub mod create_shipment;
pub mod get_shipments;
pub mod get_shipment;
pub mod delete_shipment;
pub mod patch_shipment;

pub fn create_shipment_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_shipment::create_shipment))
        .route("/", get(get_shipments::get_shipments))
        .route("/:id", get(get_shipment::get_shipment))
        .route("/:id", delete(delete_shipment::delete_shipment))
        .route("/:id", patch(patch_shipment::patch_shipment))
}
