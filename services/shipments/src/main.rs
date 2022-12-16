extern crate core;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use diesel::{Connection, PgConnection};
use sea_orm::{Database, DatabaseConnection};

pub mod lib;
pub mod models;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub sdb: DatabaseConnection
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let host = env::var("SRV_HOST").expect("SRV_HOST is missing");
    let port = env::var("SRV_PORT").expect("SRV_PORT is missing");

    let user = env::var("POSTGRES_USER").unwrap_or(String::from("postgres"));
    let password = env::var("POSTGRES_PASSWORD").unwrap_or(String::from("demo"));
    let db = env::var("POSTGRES_DB").unwrap_or("demo".parse().unwrap());
    let db_host = env::var("POSTGRES_SERVICE_HOST").unwrap_or("localhost".parse().unwrap());
    let db_port = env::var("POSTGRES_SERVICE_PORT").unwrap_or("5432".parse().unwrap());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, db_host, db_port, db
        ))
        .await
        .expect("Can't connect to database");

    let db = Database::connect(format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, db_host, db_port, db
    )).await.expect("Can't connect to database");

    let routes = routes::create_routes().with_state(AppState { db: pool, sdb: db });

    println!("Listening on http://{}:{}", host, port);
    axum::Server::bind(&format!("{}:{}", host, port).parse().unwrap())
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
