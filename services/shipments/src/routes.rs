use crate::AppState;
use axum::Router;

pub mod shipments;

pub fn create_routes() -> Router<AppState> {
    Router::new().nest("/api/shipments", shipments::create_shipment_routes())
}
