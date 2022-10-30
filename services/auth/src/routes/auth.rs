use crate::routes::AppState;
use axum::routing::get;
use axum::{routing::post, Router};
use std::sync::Arc;

mod login;
mod me;
mod signup;

pub fn create_auth_routes(state: Arc<AppState>) -> Router<AppState> {
    Router::with_state_arc(state)
        .route("/", get(|| async { "OK!" }))
        .route("/login", post(login::login_handler))
        .route("/signup", post(signup::signup_handler))
        .route("/me", get(me::me_handler))
}
