use axum::routing::get;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use lapin::{Channel, Connection, ConnectionProperties};

pub mod auth;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub channel: Channel
}

pub async fn create_routes() -> Router<AppState> {
    let user = env::var("POSTGRES_USER").unwrap_or(String::from("postgres"));
    let password = env::var("POSTGRES_PASSWORD").unwrap_or(String::from("demo"));
    let db = env::var("POSTGRES_DB").unwrap_or("demo".parse().unwrap());
    let host = env::var("POSTGRES_SERVICE_HOST").unwrap_or("localhost".parse().unwrap());
    let port = env::var("POSTGRES_SERVICE_PORT").unwrap_or("5432".parse().unwrap());
    let amqp = env::var("AMQP_URL").unwrap_or("amqp://user:user@localhost:5672".parse().unwrap());

    let conn = Connection::connect(&amqp, ConnectionProperties::default())
        .await
        .expect("connection error");

    let channel = conn.create_channel().await.expect("create_channel");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db))
        .await
        .expect("Can't connect to database");

    let shared_state = Arc::new(AppState { db: pool, channel });

    Router::with_state_arc(shared_state.clone())
        .nest("/api/auth", auth::create_auth_routes(shared_state.clone()))
}
