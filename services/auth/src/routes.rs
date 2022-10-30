use axum::routing::get;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

pub mod auth;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub async fn create_routes() -> Router<AppState> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*env::var("DATABASE_URL").expect("DATABASE_URL missing"))
        .await
        .expect("Can't connect to database");

    let shared_state = Arc::new(AppState { db: pool });

    Router::with_state_arc(shared_state.clone())
        .nest("/api/auth", auth::create_auth_routes(shared_state.clone()))
}
