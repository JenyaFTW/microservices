use axum::Router;
use axum::routing::{get, post};
use crate::handlers::login::login_handler;
use crate::handlers::me::me_handler;
use crate::handlers::signup::signup_handler;

pub async fn create_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
        .route("/me", get(me_handler))
}
