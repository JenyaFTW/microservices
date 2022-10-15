use axum::Router;
use axum::routing::get;

pub async fn create_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "OK" }))
}
